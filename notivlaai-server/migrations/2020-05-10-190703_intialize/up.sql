-- Your SQL goes here
PRAGMA foreign_keys = ON;
CREATE TABLE vlaai (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE customer (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR
);

CREATE TABLE `order` (
    id INTEGER NOT NULL PRIMARY KEY,
    customer_id INTEGER NOT NULL,
    in_transit BOOLEAN NOT NULL DEFAULT false,
    picked_up BOOLEAN NOT NULL DEFAULT false,
    order_number INTEGER,
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

