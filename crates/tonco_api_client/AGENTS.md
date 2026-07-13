# AGENTS.md

## Scope

This crate is `tonco_api_client`, a Rust library crate that wraps the Tonco
Indexer GraphQL API.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a low-level GraphQL execution boundary:

- `ToncoApiClient::builder().build()?`
- `client.graphql.exec(operation_name, &query)`
- checked-in GraphQL schema at `src/graphql_schema.json`
- usage examples in `tests/test_graphql.rs` and `tests/pools.graphql`

Do not invent higher-level pool, swap, or indexer domain APIs unless the user
explicitly asks for that layer.

## Public API Boundary

Treat these as public contracts:

- `ToncoApiClient`
- `DEFAULT_GRAPHQL_ENDPOINT`
- `GraphqlApiClient`

Public client types are `#[non_exhaustive]` where applicable; avoid public
struct literal construction in examples.

The default endpoint intentionally has no trailing slash:
`https://indexer.tonco.io`. `client.graphql.exec` posts through
`Executor::exec_post_body("", ...)`, so a trailing slash would produce `POST //`
against the live service.

## GraphQL Rules

- Keep operation names aligned with generated `graphql_client` query types.
- Preserve the `x-apollo-operation-name` header unless live endpoint
  verification proves it is obsolete.
- Regenerate or update schema/query artifacts when the Tonco schema changes.
- Treat GraphQL `errors` as `ApiClientsError::UnexpectedResponse`.
- Treat missing `data` as `ApiClientsError::UnexpectedResponse`.

## Downstream Integration Example

```rust
use graphql_client::GraphQLQuery;
use tonco_api_client::api_client::ToncoApiClient;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tonco_api_client/src/graphql_schema.json",
    query_path = "tonco_api_client/tests/pools.graphql",
    response_derives = "Debug"
)]
pub struct Pools;

# async fn example() -> anyhow::Result<()> {
let client = ToncoApiClient::builder().build()?;
let query = Pools::build_query(pools::Variables);
let data: pools::ResponseData = client.graphql.exec(pools::OPERATION_NAME, &query).await?;
if let Some(pools) = data.pools {
    println!("pools: {}", pools.len());
}
# Ok(())
# }
```

Final applications should own their query files, generated query modules, and
schema update workflow.

## Validation

```bash
cargo test -p tonco_api_client --tests
cargo +nightly fmt
cargo clippy -p tonco_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p tonco_api_client --no-deps
cargo package --list -p tonco_api_client
```
