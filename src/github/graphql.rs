use graphql_client::GraphQLQuery;


type DateTime = chrono::DateTime<chrono::Utc>;
type URI = String;
type GitObjectID = String;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/pull-requests.graphql",
    response_derives = "Debug",
)]
pub struct PullRequestsView;


#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task;

    #[test]
    fn it_works() {
        let token = "Bearer <<insert token here>>";

        let query = PullRequestsView::build_query(pull_requests_view::Variables{
            owner: "felipesere".into(),
            name: "advisorex".into(),
        });

        let data = task::block_on(async {
            let mut response = surf::post("https://api.github.com/graphql").set_header("Authorization", token).body_json(&query).unwrap().await.unwrap();

            let inner: graphql_client::Response<pull_requests_view::ResponseData> = response.body_json().await.unwrap();

            let response_data: pull_requests_view::ResponseData = inner.data.expect("missing response data");

            response_data
        });

        dbg!(data);
        assert!(false);
    }
}
