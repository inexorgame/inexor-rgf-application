use async_graphql::Object;

use crate::model_runtime::InstanceInfo;

pub struct GraphQLInstanceInfo {
    /// The instance information.
    pub instance_info: InstanceInfo,
}

#[Object(name = "InstanceInfo")]
impl GraphQLInstanceInfo {
    async fn name(&self) -> String {
        self.instance_info.name.clone()
    }

    async fn description(&self) -> String {
        self.instance_info.description.clone()
    }

    async fn hostname(&self) -> String {
        self.instance_info.hostname.clone()
    }

    async fn port(&self) -> u16 {
        self.instance_info.port
    }

    async fn secure(&self) -> bool {
        self.instance_info.secure
    }

    async fn version(&self) -> String {
        self.instance_info.version.clone()
    }

    async fn build_date(&self) -> String {
        self.instance_info.build_date.clone()
    }

    async fn git_branch(&self) -> String {
        self.instance_info.git_branch.clone()
    }

    async fn git_commit(&self) -> String {
        self.instance_info.git_commit.clone()
    }

    async fn rustc_version(&self) -> String {
        self.instance_info.rustc_version.clone()
    }

    async fn plugin_api_version(&self) -> String {
        self.instance_info.plugin_api_version.clone()
    }
}