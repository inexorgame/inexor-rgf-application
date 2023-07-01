use async_graphql::Response;
use async_trait::async_trait;
use serde_json::Error;

use crate::api::Lifecycle;

#[async_trait]
pub trait GraphQLQueryService: Send + Sync + Lifecycle {
    /// Runs the given GraphQL query.
    async fn query(&self, request: &str) -> Result<String, Error>;

    /// Runs the given GraphQL query and returns the response.
    async fn query_response(&self, request: &str) -> Response;
}