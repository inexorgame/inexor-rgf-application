use async_graphql::Request;
use std::sync::Arc;

use serde::Deserialize;

use crate::api::GraphQLQueryService;
use crate::model_runtime::InstanceInfo;
use crate::runtime::Runtime;
use crate::runtime::RuntimeBuilder;

#[derive(Deserialize, Debug)]
struct Mutation {
    system: MutationSystem,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationSystem {
    remotes: MutationSystemFetchRemotesFromAllRemotes,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationSystemFetchRemotesFromAllRemotes {
    fetch_remotes_from_all_remotes: Vec<InstanceInfo>,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fetch_remotes_from_all_remotes() {
    RuntimeBuilder::new()
        .ignore_config_files()
        .disable_all_plugins(true)
        .pick_free_port()
        .init()
        .await
        .post_init()
        .await
        .spawn()
        .await
        .with_runtime(|runtime: Arc<dyn Runtime>| async move {
            let query_service = runtime.get_graphql_query_service();
            let instance_service = runtime.get_instance_service();
            let remotes_manager = runtime.get_remotes_manager();
            remotes_manager.remove_all();

            // RT: Expect that no remotes exists
            assert_eq!(remotes_manager.get_all().len(), 0);

            // RT: Add self as remote
            let address = instance_service.get_instance_info().address();
            let _instance_info = remotes_manager.add(&address).await.expect("Failed to add remote");

            // GQL: Fetch remotes of all remotes
            let _fetched_remotes_from_all_remotes = mutation_fetch_remotes_from_all_remotes(&query_service).await;
        })
        .await
        .stop()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const MUTATION_FETCH_REMOTES_FROM_ALL_REMOTES: &str = include_str!("../../../../../graphql/system/remotes/fetch_remotes_from_all_remotes.graphql");

async fn mutation_fetch_remotes_from_all_remotes(query_service: &Arc<dyn GraphQLQueryService>) -> Vec<InstanceInfo> {
    let request = Request::new(MUTATION_FETCH_REMOTES_FROM_ALL_REMOTES);
    let response = query_service.execute(request).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    println!("{}", data);
    let data: Mutation = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.system.remotes.fetch_remotes_from_all_remotes
}
