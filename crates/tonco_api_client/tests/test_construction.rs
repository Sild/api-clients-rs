use tonco_api_client::api_client::ToncoApiClient;

#[test]
fn test_client_exposes_graphql_executor() -> anyhow::Result<()> {
    let client = ToncoApiClient::builder().build()?;
    let query = graphql_client::QueryBody {
        variables: (),
        query: "query Pools { pools { id } }",
        operation_name: "Pools",
    };
    let future = client.graphql.exec::<_, ()>("Pools", &query);
    drop(future);
    Ok(())
}
