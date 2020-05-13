-- Your SQL goes here

CREATE TABLE vlaai (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE customer (
    id INTEGER NOT NULL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);

CREATE TABLE `order` (
    id INTEGER NOT NULL PRIMARY KEY,
    customer_id INTEGER NOT NULL,
    in_transit BOOLEAN NOT NULL DEFAULT false,
    picked_up BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY(customer_id) REFERENCES Customer(id)
);

CREATE TABLE vlaai_to_order (
    id INTEGER NOT NULL PRIMARY KEY,
    order_id INTEGER NOT NULL,
    vlaai_id INTEGER NOT NULL,
    amount INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY(order_id) REFERENCES `Order`(id),
    FOREIGN KEY(vlaai_id) REFERENCES Vlaai(id)
);

