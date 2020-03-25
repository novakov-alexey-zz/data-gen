use rand::Rng;
use rand::distributions::{Distribution, Standard};
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use super::schema::orders;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[primary_key(order_id)]
#[derive(Identifiable, Debug, Insertable, Serialize)]
pub struct Order {
    order_id: i32,
    created: DateTime<Utc>,
    created_day: String,
    amount: f32
}

impl Distribution<Order> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Order {
        let rand_id = rng.gen_range(1, 999_999);
        let rand_amount = rng.gen();

        let rand_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let d = UNIX_EPOCH + Duration::from_secs(rand_time as u64);
        let datetime = DateTime::<Utc>::from(d);
        let day = datetime.format("%Y-%m-%d").to_string();

        Order {
            order_id: rand_id,
            created: Utc.timestamp(rand_time, 0),
            created_day: day,
            amount: rand_amount
        }
    }
}