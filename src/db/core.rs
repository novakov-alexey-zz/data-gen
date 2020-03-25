use std::error;

use diesel::*;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use super::models::Order;
use super::schema::orders::dsl::*;

pub struct DbService {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> DbService {
        DbService {
            connection_pool: pool,
        }
    }

    pub fn insert(&self, records: Vec<Order>) -> Result<usize, String> {
        self.conn().and_then(|c| {
            super::diesel::insert_into(orders)
                .values(records)
                .execute(&*c)
                .map_err(self::to_string)
        })
    }

    fn conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, String> {
        self.connection_pool.get().map_err(self::to_string)
    }
}

fn to_string<E>(e: E) -> String
    where
        E: error::Error,
{
    e.to_string()
}