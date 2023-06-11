use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[cfg_attr(test, derive(Default))]
#[kube(kind = "Nats", group = "nats.grp", version = "v1alpha1", namespaced)]
#[kube(status = "NatsStatus", shortname = "nmgr")]
#[allow(non_snake_case)]
pub struct NatsSpec {
    pub cluster: Cluster,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[allow(non_snake_case)]
pub struct Cluster {
    pub size: i32
}

#[derive(Deserialize, Serialize, Clone, Default, Debug, JsonSchema)]
#[allow(non_snake_case)]
pub struct NatsStatus {
    pub running: bool,
}
