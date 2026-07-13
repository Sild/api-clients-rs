use std::future;
use std::time::Duration;

use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use tokio::sync::Mutex;
use tokio::time::Instant;

pub(super) struct RequestRateLimiter {
    interval: Option<Duration>,
    next_request_at: Mutex<Instant>,
}

impl RequestRateLimiter {
    pub(super) fn new(max_rps: usize) -> Self {
        Self {
            interval: Duration::from_secs(1).checked_div(max_rps as u32),
            next_request_at: Mutex::new(Instant::now()),
        }
    }

    pub(super) async fn wait(&self) {
        let Some(interval) = self.interval else {
            return future::pending().await;
        };
        let request_at = {
            let mut next_request_at = self.next_request_at.lock().await;
            let now = Instant::now();
            let request_at = (*next_request_at).max(now);
            *next_request_at = request_at + interval;
            request_at
        };

        tokio::time::sleep_until(request_at).await;
    }
}

pub(super) struct RateLimitMiddleware {
    limiter: RequestRateLimiter,
}

impl RateLimitMiddleware {
    pub(super) fn new(max_rps: usize) -> Self {
        Self {
            limiter: RequestRateLimiter::new(max_rps),
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimitMiddleware {
    async fn handle(&self, req: Request, extensions: &mut Extensions, next: Next<'_>) -> Result<Response> {
        self.limiter.wait().await;
        next.run(req, extensions).await
    }
}

#[cfg(test)]
mod tests {
    use super::RequestRateLimiter;
    use std::time::Duration;
    use tokio::time::{self, Instant};

    #[tokio::test(start_paused = true)]
    async fn test_rate_limiter_first_wait_is_immediate() {
        let limiter = RequestRateLimiter::new(1);
        let started_at = Instant::now();

        limiter.wait().await;

        assert_eq!(started_at.elapsed(), Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limiter_delays_concurrent_calls_without_burst() {
        let limiter = RequestRateLimiter::new(2);
        let started_at = Instant::now();

        let ((), ()) = tokio::join!(limiter.wait(), limiter.wait());

        assert!(started_at.elapsed() >= Duration::from_millis(500));
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limiter_serializes_reservations() {
        let limiter = RequestRateLimiter::new(2);
        let started_at = Instant::now();

        let third_wait = async {
            limiter.wait().await;
            limiter.wait().await;
            limiter.wait().await;
        };

        assert!(time::timeout(Duration::from_millis(999), third_wait).await.is_err());
        time::advance(Duration::from_millis(1)).await;

        assert!(started_at.elapsed() >= Duration::from_secs(1));
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limiter_zero_rps_waits_forever() {
        let limiter = RequestRateLimiter::new(0);

        assert!(time::timeout(Duration::from_secs(60), limiter.wait()).await.is_err());
    }
}
