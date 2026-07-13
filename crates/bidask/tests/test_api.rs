// Bidask support is disabled for this workspace. Keep the live API test
// commented out while `bidask` remains unsupported and unpublished.
//
// use std::vec;
//
// use anyhow::Result;
// use bidask::api::Request;
// use bidask::api_client::BidaskApiClient;
// use bidask::unwrap_response;
//
// fn init_env() -> BidaskApiClient {
//     let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
//     BidaskApiClient::builder().build().unwrap()
// }
//
// #[tokio::test]
// async fn test_pools() -> Result<()> {
//     let client = init_env();
//     let request = Request::Pools;
//     let response = unwrap_response!(Pools, client.api.exec(&request).await?)?;
//     assert_ne!(response, vec![]);
//     log::debug!("{:?}", response.len());
//     Ok(())
// }
