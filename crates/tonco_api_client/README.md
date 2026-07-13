# tonco_api_client

Low-level GraphQL wrapper for the [Tonco Indexer API](https://indexer.tonco.io/docs).

Use this crate when an application needs to execute Tonco GraphQL queries while
keeping query files, generated query modules, schema refreshes, pagination, and
domain mapping in the application layer. The crate intentionally exposes
`client.graphql.exec(operation_name, query)` instead of higher-level pool or
swap APIs.

## Usage

```toml
[dependencies]
tonco_api_client = "0.1"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Run requests inside an async Tokio runtime. `client.graphql.exec` sends the
`x-apollo-operation-name` header, unwraps GraphQL `data`, and maps GraphQL
`errors` or missing `data` to `ApiClientsError::UnexpectedResponse`.

```rust,no_run
use serde::{Deserialize, Serialize};
use tonco_api_client::api_client::ToncoApiClient;

#[derive(Serialize)]
struct GraphqlRequest<V> {
    #[serde(rename = "operationName")]
    operation_name: &'static str,
    query: &'static str,
    variables: V,
}

#[derive(Serialize)]
struct EmptyVariables;

#[derive(Deserialize)]
struct PoolsData {
    pools: Option<Vec<Option<Pool>>>,
}

#[derive(Deserialize)]
struct Pool {
    address: String,
}

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = ToncoApiClient::builder().build()?;
let request = GraphqlRequest {
    operation_name: "Pools",
    query: "query Pools { pools { address } }",
    variables: EmptyVariables,
};

let data: PoolsData = client.graphql.exec("Pools", &request).await?;
let pool_count = data.pools.unwrap_or_default().len();
println!("pools: {pool_count}");
# Ok(())
# }
```

For larger applications, prefer generated query types from `graphql_client`.
Keep schema and query paths relative to the consuming application crate:

```rust,ignore
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql_schema.json",
    query_path = "src/queries/pools.graphql",
    response_derives = "Debug,Clone"
)]
pub struct Pools;
```

The checked-in `src/graphql_schema.json`, `tests/pools.graphql`, and
`tests/test_graphql.rs` files show the repository's live-test setup. Application
crates should own their own query files and schema update workflow.

Public client types are marked `#[non_exhaustive]` where applicable for semver
headroom. The default endpoint intentionally has no trailing slash:
`https://indexer.tonco.io`. Adding a trailing slash can produce `POST //`
against the live service.
