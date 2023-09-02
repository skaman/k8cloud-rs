use chrono::{DateTime, Utc};
use diesel::Queryable;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

#[derive(GraphQLObject, Queryable, Debug)]
pub struct Cluster {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub certificate_authority_data: String,
    pub user_name: String,
    pub user_credentials_certificate_data: String,
    pub user_crendetiral_key_data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject)]
pub struct CreateCluster {
    pub name: String,
    pub address: String,
    pub certificate_authority_data: String,
    pub user_name: String,
    pub user_credentials_certificate_data: String,
    pub user_crendetiral_key_data: String,
}

#[derive(GraphQLInputObject)]
pub struct UpdateCluster {
    pub name: String,
    pub address: String,
    pub certificate_authority_data: String,
    pub user_name: String,
    pub user_credentials_certificate_data: String,
    pub user_crendetiral_key_data: String,
}
