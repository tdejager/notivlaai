table! {
    Customer (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
    }
}

table! {
    Order (id) {
        id -> Integer,
        customer_id -> Integer,
    }
}

table! {
    Vlaai (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    VlaaiToOrder (id) {
        id -> Integer,
        order_id -> Integer,
        vlaai_id -> Integer,
    }
}

joinable!(Order -> Customer (customer_id));
joinable!(VlaaiToOrder -> Order (order_id));
joinable!(VlaaiToOrder -> Vlaai (vlaai_id));

allow_tables_to_appear_in_same_query!(
    Customer,
    Order,
    Vlaai,
    VlaaiToOrder,
);
