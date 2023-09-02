use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub mod schema;

lazy_static! {
    static ref POOL: Pool<ConnectionManager<PgConnection>> = init_pool();
}

fn init_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_pooled_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    let pool = POOL.clone();
    let database_connection = pool.get().expect("Failed to get pooled connection");
    database_connection
}
