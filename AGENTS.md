# AGENTS.md

## Scope

This repository is an open source Rust workspace for raw API-client wrappers
around external web services. It is a library workspace, not an application.

Use the `rust-library-review` skill for changes that affect public API shape,
crate usability, README/rustdoc accuracy, crate metadata, package contents, or
agent-facing documentation.

Root guidance applies to the whole workspace. Also read the crate-local
`AGENTS.md` before changing a member crate.

## Workspace Crates

- `api_clients_core` (`crates/core/`): shared HTTP executor, retry setup, and
  common error/result types.
- `stonfi_api_client` (`crates/stonfi/`): REST wrapper for STON.fi API v1 and
  public export endpoints.
- `stonks_api_client` (`crates/stonks/`): REST wrapper for Stonks public-token
  metadata, batch jetton metadata, and Virtual Pool address discovery.
- `dedust_api_client` (`crates/dedust/`): REST wrapper for the DeDust asset
  registry, API v4 pool registries, and legacy API v2 operations.
- `swap_coffee_api_client` (`crates/swap_coffee/`): REST wrapper for Swap Coffee
  API v1.
- `tonco_api_client` (`crates/tonco_api_client/`): GraphQL wrapper for Tonco
  Indexer.
- `bidask_api_client` (`crates/bidask/`): unsupported legacy Bidask crate; it is
  intentionally excluded from publishing and should not be recommended for
  final-app integration.

## Project Goals

- Provide thin, typed Rust wrappers over service APIs.
- Keep request/response structs close to upstream API contracts.
- Keep clients easy to embed in final applications.
- Make live API drift explicit through focused tests and README endpoint
  matrices.

## Non-Goals

- Do not add application-specific business logic.
- Do not normalize unrelated services into a shared domain model unless the
  user asks for that design.
- Do not hide upstream API instability behind silent fallback behavior.
- Do not introduce background workers, caching, rate-limit orchestration, or
  secret management without a specific request.

## Public API Boundaries

Treat these as public library surface:

- crate names, versions, and manifest metadata
- public clients and builders
- public request enums and parameter structs
- public response enums and response/type structs
- public constants such as default API endpoints
- `api_clients_core::{ApiClientsError, ApiClientsResult, Executor}`
- exported helper macros such as `unwrap_response!`
- README endpoint support tables and examples

Release versions and generated release changelog entries are managed by
release-plz in CI. Do not treat unchanged crate versions on a feature branch as
a release-readiness finding unless the user explicitly asks for a manual
release-prep change.

The workspace MSRV is Rust 1.88. Dependency changes must preserve it and pass
the dedicated MSRV CI job. The release workflow must refuse to process a
default-branch commit other than the exact SHA that completed `Build-Test`.

Public structs and enums are generally marked `#[non_exhaustive]` to preserve
semver headroom for new fields and variants. Use
`Default::default().with_<field>(...)` or request parameter constructors in
examples/tests instead of public struct literals, pass request parameters
directly to clients where `Into<Request>` is implemented, and include wildcard
arms when matching public enums.

When any public interface, endpoint capability, feature flag, workspace member,
default endpoint, or crate package surface changes, review and update the
affected root/crate `AGENTS.md`, root/crate `README.md`, rustdoc comments,
examples/tests, and package include/exclude rules in the same change. If no docs
change is needed, state why in the final report.

## Development Rules

- Preserve thin-wrapper behavior. Prefer adding the missing endpoint mapping,
  request params, response types, and tests over redesigning the client.
- Follow the existing crate shape: `api_client.rs`, `api_client/builder.rs`, API module
  files such as `v1.rs`, `v1/request.rs`, `v1/response.rs`, and `v1/types.rs`
  for versioned REST APIs. Keep unversioned service groups in a descriptive
  module such as `api.rs` or `graphql.rs`.
- Use the existing `derive_setters` builder pattern for client configuration.
- Prefer typed request parameter structs with serde derives over manual query
  string construction.
- Keep endpoint paths centralized in client match arms. Do not scatter raw URLs
  through tests or examples.
- Do not add broad re-exports or flattened public paths unless matching an
  existing crate-local pattern.
- Public errors should flow through `ApiClientsResult`; production code must
  not use `unwrap()`, `expect()`, or panic-driven control flow.
- Tests that call fallible APIs should return `anyhow::Result<()>` or the
  project result type and use `?`.

## Live API Tests

Most service-crate tests hit live third-party APIs. When they fail:

- First verify whether the upstream API contract or fixture data changed.
- Update stale expectations when live responses intentionally changed.
- Fix client code only when request construction, endpoint paths, parsing, or
  project-owned behavior is wrong.
- Avoid asserting volatile counts, ordering, balances, timestamps, or names
  unless that exact value is the project-owned contract under test.
- Prefer assertions that prove shape, parsing, request routing, and known
  stable fields.

Integration tests must call the real upstream service. They are the primary
correctness check for these thin wrappers because upstream response contracts
can drift.

Do not replace live integration coverage with mock-server tests, and do not add
environment gates that make integration tests silently skip. CI/CD must not run
integration tests. Run integration tests manually before pushing to a remote:

```bash
cargo test --workspace --tests
```

Known project pitfall: `api_clients_core::Executor` builds URLs with
`format!("{endpoint}/{path}")`. Default endpoints should not have a trailing
slash unless the specific client path construction is designed for it. Tonco's
GraphQL endpoint is intentionally `https://indexer.tonco.io` because GraphQL
queries post to an empty path.

## Adding Or Changing An Endpoint

For REST crates:

1. Add or update the request enum and parameter struct in `request.rs`.
2. Add or update response enum variants and response/type structs in `response.rs`
   and `types.rs`.
3. Wire the endpoint path and HTTP method in the crate client.
4. Add focused live or unit coverage for the new project-owned mapping.
5. Update the crate README endpoint matrix.
6. Check whether downstream integration guidance in crate `AGENTS.md` changed.

For GraphQL crates:

1. Keep checked-in schema/query files aligned with the generated types used in
   tests or examples.
2. Preserve the `exec(operation_name, query)` GraphQL boundary on the child
   GraphQL client unless the user asks for a higher-level wrapper.
3. Verify operation names and headers against the live endpoint.
4. Document usage through tests or README examples.

## Final-App Integration Guidance

When helping an agent integrate these crates into a final application:

- Pick the narrowest service crate needed by the app.
- Use the crate client builder and run calls inside an async Tokio runtime.
- Let application code decide retry policy, observability, persistence,
  throttling, and fallback behavior unless this library already exposes a
  typed option for it.
- Handle `ApiClientsError::{Client, Server, Network, UnexpectedResponse, ...}`
  explicitly at the application boundary.
- Do not assume live API tests prove an endpoint is stable forever. External
  services may change names, supported assets, GraphQL schema, pagination, or
  fixture availability.
- If the app overrides endpoints or injects an executor, avoid trailing slash
  mismatches with `Executor` URL construction.
- Prefer matching response enum variants or using the crate `unwrap_response!`
  helper where it improves clarity. Do not unwrap blindly in app code.
- Pin crate versions or git revisions in applications that need reproducible
  behavior.

## Validation

For docs-only changes:

```bash
git diff --check
cargo metadata --no-deps --format-version 1
cargo package --list -p <changed-crate>
```

For production code or public API changes, use targeted tests first, then the
workspace gate:

```bash
cargo test -p <changed-crate> --tests
cargo test
cargo +nightly fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo +1.88.0 check --workspace --all-targets --all-features
cargo deny check
git diff --check
```

Use `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` when rustdoc or public API
documentation changed. Use `cargo semver-checks check-release --workspace` for
intentional breaking releases and review the release-plz version bumps plus all
README dependency snippets before publishing.
