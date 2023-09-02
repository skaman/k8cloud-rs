use juniper::{graphql_value, FieldError, FieldResult};
use uuid::Uuid;

use self::models::{Cluster, CreateCluster, UpdateCluster};

pub mod models;
mod services;

pub fn create_cluster(data: CreateCluster) -> FieldResult<Cluster> {
    let cluster = services::clusters::create(data);
    if cluster.is_err() {
        return Err(FieldError::new(
            "Cannot create cluster",
            graphql_value!({ "internal_error": "Cannot create cluster" }),
        ));
    }

    Ok(cluster.unwrap())
}

pub fn update_cluster(cluster_id: Uuid, data: UpdateCluster) -> FieldResult<Cluster> {
    let cluster = services::clusters::update(cluster_id, data);
    if cluster.is_err() {
        return Err(FieldError::new(
            "Cannot update cluster",
            graphql_value!({ "internal_error": "Cannot create cluster" }),
        ));
    }

    Ok(cluster.unwrap())
}
