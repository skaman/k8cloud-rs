use chrono::{DateTime, Utc};
use diesel::{query_dsl::QueryDsl, result::Error, ExpressionMethods, RunQueryDsl};
use std::ops::DerefMut;
use uuid::Uuid;

use crate::{
    cluster::models::{Cluster, CreateCluster, UpdateCluster},
    db::{get_pooled_connection, schema::clusters, schema::clusters::dsl::*},
};

pub fn create(data: CreateCluster) -> Result<Cluster, Error> {
    let mut connection = get_pooled_connection();

    let now: DateTime<Utc> = DateTime::from(Utc::now());
    let record = diesel::insert_into(clusters::table)
        .values((
            id.eq(Uuid::new_v4()),
            name.eq(data.name),
            address.eq(data.address),
            certificate_authority_data.eq(data.certificate_authority_data),
            user_name.eq(data.user_name),
            user_credentials_certificate_data.eq(data.user_credentials_certificate_data),
            user_crendetiral_key_data.eq(data.user_crendetiral_key_data),
            created_at.eq(now),
            updated_at.eq(now),
        ))
        .get_result::<Cluster>(connection.deref_mut());

    record
}

pub fn update(cluster_id: Uuid, data: UpdateCluster) -> Result<Cluster, Error> {
    let mut connection = get_pooled_connection();

    //let now: DateTime<Utc> = DateTime::from(Utc::now());
    let record = diesel::update(clusters::table)
        .filter(id.eq(cluster_id))
        .set((
            name.eq(data.name),
            address.eq(data.address),
            certificate_authority_data.eq(data.certificate_authority_data),
            user_name.eq(data.user_name),
            user_credentials_certificate_data.eq(data.user_credentials_certificate_data),
            user_crendetiral_key_data.eq(data.user_crendetiral_key_data),
            //updated_at.eq(now),
        ))
        .get_result::<Cluster>(connection.deref_mut());

    record
}

pub fn get_cluster_by_id(cluster_id: Uuid) -> Result<Cluster, Error> {
    let mut connection = get_pooled_connection();
    clusters
        .filter(id.eq(cluster_id))
        .first::<Cluster>(connection.deref_mut())
}
