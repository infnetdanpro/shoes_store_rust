-- Add migration script here
CREATE TABLE orders_products
(
    id         SERIAL PRIMARY KEY,
    order_id   uuid NOT NULL,
    product_id INTEGER NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products (id),
    FOREIGN KEY (order_id) REFERENCES orders (id)
)