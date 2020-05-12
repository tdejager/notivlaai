table! {
    customer (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
    }
}

table! {
    order (id) {
        id -> Integer,
        customer_id -> Integer,
    }
}

table! {
    vlaai (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    vlaai_to_order (id) {
        id -> Integer,
        order_id -> Integer,
        vlaai_id -> Integer,
        amount -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    customer,
    order,
    vlaai,
    vlaai_to_order,
);
