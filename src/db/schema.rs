table! {
    orders (order_id) {
        order_id -> Integer,
        created -> Timestamptz,
        created_day -> Text,
        amount -> Float,
    }
}