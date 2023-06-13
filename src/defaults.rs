use crate::apis::nats_types::Cluster;

pub fn cluster() -> Cluster {
    Cluster { size: cluster_size() }
}

pub fn cluster_size() -> i32 {
    3
}