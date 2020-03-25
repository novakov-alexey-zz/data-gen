extern crate openssl;
#[macro_use]
extern crate diesel;
extern crate data_gen;
extern crate dotenv;
extern crate job_scheduler;
extern crate rand;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::iter;
use std::time::Duration;

use dotenv::dotenv;
use job_scheduler::{Job, JobScheduler};
use rand::prelude::ThreadRng;
use rand::Rng;
use env_logger::{Builder, Target};
use data_gen::create_db_pool;
use data_gen::db::core::DbService;
use data_gen::db::models::Order;

fn generate_data(count: usize, mut rng: ThreadRng) -> Vec<Order> {
    iter::from_fn(move || Some(rng.gen())).take(count).collect()
}

fn insert_data(db: &DbService, max_count: usize) {
    let mut rng = rand::thread_rng();
    let count: usize = rng.gen_range(1, max_count + 1);
    let records = generate_data(count, rng);
    db.insert(records).expect("Failed to insert a call");
    info!("Generated records: {:?}", count);
}

fn init_logger() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}

fn main() {
    init_logger();
    info!("starting up...");
    dotenv().ok();

    let schedule = env::var("CRON_EXP")
        .unwrap_or_else(|_| String::from("1/10 * * * * *"));
    let max_count = env::var("MAX_COUNT").ok()
        .map_or(30, |e| e.parse::<usize>().expect("MAX_COUNT must be set"));

    let db = DbService::new(create_db_pool());
    let mut sched = JobScheduler::new();

    sched.add(Job::new(schedule.parse().unwrap(), || insert_data(&db, max_count)));
    info!("data-gen job scheduled on: {}", schedule);

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}
