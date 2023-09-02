use crate::cluster;
use crate::cluster::models::{Cluster, CreateCluster, UpdateCluster};
use juniper::{EmptySubscription, FieldResult, RootNode};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn cluster(&self, cluster_id: Uuid) -> Cluster {
        todo!()
        //Ok(Cluster {
        //    id: "1234".to_owned(),
        //    name: "Luke".to_owned(),
        //    address: "Mars".to_owned(),
        //    certificate_authority_data: "Mars".to_owned(),
        //    user_name: "Mars".to_owned(),
        //    user_credentials_certificate_data: "Mars".to_owned(),
        //    user_crendetiral_key_data: "Mars".to_owned(),
        //    created_at: "Mars".to_owned(),
        //    updated_at: "Mars".to_owned(),
        //})
    }

    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_cluster(data: CreateCluster) -> FieldResult<Cluster> {
        cluster::create_cluster(data)
    }

    fn update_cluster(cluster_id: Uuid, data: UpdateCluster) -> FieldResult<Cluster> {
        cluster::update_cluster(cluster_id, data)
    }

    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
