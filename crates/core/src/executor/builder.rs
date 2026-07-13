use crate::executor::rate_limiter::RateLimitMiddleware;
use crate::{ApiClientsError, ApiClientsResult, Executor};
use derive_setters::Setters;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::sync::Arc;
use std::time::Duration;

/// Builder for [`Executor`].
///
/// Defaults to 3 retries, a 10-second request timeout, and a smooth 10 RPS
/// client-side rate limit. Set `max_rps` to `0` only when the caller wants
/// requests to wait indefinitely instead of sending.
#[derive(Setters, Debug)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Builder {
    #[setters(skip)]
    api_url: String,
    retry_count: u32,
    timeout: Duration,
    max_rps: usize,
    http_client: Option<Arc<ClientWithMiddleware>>,
}

impl Builder {
    pub(super) fn new(api_url: String) -> Self {
        Self {
            api_url,
            retry_count: 3,
            timeout: Duration::from_secs(10),
            max_rps: 10,
            http_client: None,
        }
    }

    pub fn build(self) -> ApiClientsResult<Executor> {
        let rate_limit = RateLimitMiddleware::new(self.max_rps);
        let http_client = match self.http_client {
            Some(cli) => ClientBuilder::from_client((*cli).clone()).with(rate_limit).build().into(),
            None => {
                let retry = ExponentialBackoff::builder().build_with_max_retries(self.retry_count);
                let client = reqwest::ClientBuilder::new()
                    .timeout(self.timeout)
                    .build()
                    .map_err(|err| ApiClientsError::Internal(err.to_string()))?;

                ClientBuilder::new(client)
                    .with(RetryTransientMiddleware::new_with_policy(retry))
                    .with(rate_limit)
                    .build()
                    .into()
            }
        };

        let executor = Executor {
            api_endpoint: self.api_url,
            http_client,
        };
        Ok(executor)
    }
}
