extern crate rand;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde_derive;

use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;
use std::time::Duration;

pub mod db;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(3)
        .connection_timeout(Duration::new(5, 0))
        .build(manager)
        .expect("Failed to create a database pool.")
}