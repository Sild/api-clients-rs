# api_clients_core

Shared transport and error primitives for the API client crates in this
workspace.

Most applications should depend on one of the service crates, such as
`stonfi_api_client` or `dedust_api_client`, instead of using this crate
directly. Use `api_clients_core` directly when you need to share a configured
HTTP executor across service clients, inject a custom `reqwest_middleware`
client, or build another thin service wrapper in this workspace.

## Capabilities

- `Executor` executes JSON HTTP requests for the service-specific clients.
- `Executor::builder(endpoint)` configures the base endpoint, retry count,
  request timeout, max RPS, and optional middleware client.
- `exec_get`, `exec_get_extra`, `exec_post_qs`, and `exec_post_body` parse
  successful JSON responses into caller-provided `DeserializeOwned` types.
- `ApiClientsError` classifies client errors, server errors, invalid arguments,
  unexpected response bodies, and transport/internal failures.

The default executor retries transient failures 3 times, uses a 10-second
request timeout, and applies a smooth 10 RPS client-side rate limit.

## Usage

```toml
[dependencies]
api_clients_core = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust,no_run
use api_clients_core::{ApiClientsError, Executor};
use std::time::Duration;

# async fn example() -> Result<(), ApiClientsError> {
let executor = Executor::builder("https://api.example.com")
    .with_retry_count(3)
    .with_timeout(Duration::from_secs(10))
    .with_max_rps(10)
    .build()?;

let response: String = executor.exec_get("health").await?;
println!("{response}");
# Ok(())
# }
```

## Error Handling

Handle errors at the application boundary and include a wildcard arm because
`ApiClientsError` is `#[non_exhaustive]`:

```rust
use api_clients_core::ApiClientsError;

fn classify_error(err: ApiClientsError) -> &'static str {
    match err {
        ApiClientsError::Client(_, _) => "client",
        ApiClientsError::Server(_, _) => "server",
        ApiClientsError::Network(_, _) => "network",
        ApiClientsError::UnexpectedResponse(_) => "unexpected-response",
        _ => "other",
    }
}
```

## Endpoint Joining

`Executor` joins endpoint and path with `format!("{endpoint}/{path}")`. Endpoint
strings normally should not end with `/`; otherwise requests can accidentally
target paths with `//`.

`exec_get` and `exec_get_extra` serialize query strings with `serde_qs`.
`exec_post_qs` sends POST requests with query-string parameters.
`exec_post_body` serializes the request body as JSON.
