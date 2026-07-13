use api_clients_core::{ApiClientsError, ApiClientsResult, Executor};
use graphql_client::Response;
use serde::{de, ser};
use std::sync::Arc;

#[derive(Clone)]
pub struct GraphqlApiClient {
    executor: Arc<Executor>,
}

impl GraphqlApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    pub async fn exec<PARAMS, RSP>(&self, op_name: &str, graphql_query: &PARAMS) -> ApiClientsResult<RSP>
    where
        PARAMS: ser::Serialize,
        RSP: de::DeserializeOwned,
    {
        let headers = &[
            (reqwest::header::CONTENT_TYPE.to_string(), "application/json".to_string()),
            ("x-apollo-operation-name".to_string(), op_name.to_string()),
        ];
        let response = self.executor.exec_post_body("", graphql_query, headers).await?;
        handle_graphql_result(response)
    }
}

fn handle_graphql_result<R>(graphql_result: Response<R>) -> ApiClientsResult<R> {
    if let Some(errors) = graphql_result.errors {
        let msgs: Vec<String> = errors.into_iter().map(|e| e.message).collect();
        let err_msg = msgs.join(", ");
        Err(ApiClientsError::UnexpectedResponse(err_msg))
    } else if let Some(data) = graphql_result.data {
        Ok(data)
    } else {
        Err(ApiClientsError::UnexpectedResponse("No data in GraphQL response".to_string()))
    }
}
