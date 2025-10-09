-- Add migration script here
CREATE TABLE orders_product (
    id SERIAL PRIMARY KEY,
    order_id UUID,
    product_id INTEGER,
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
)