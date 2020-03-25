# data-gen

Generates simple order record and inserts into a Postgres database.

model:
```rust
pub struct Order {
    order_id: i32,
    created: DateTime<Utc>,
    created_day: String,
    amount: f32
}
```

Envrionment variables:

- DATABASE_URL=postgres://exchange:exchange@localhost/exchange
- CRON_EXP="1/10 * * * * *"
- MAX_COUNT=30

## Build Docker image

```bash
sh docker-image.sh 0.0.1
```
