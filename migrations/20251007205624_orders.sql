-- Add migration script here
CREATE TABLE orders (
    id UUID NOT NULL DEFAULT uuid_generate_v1(),
    customer_id INTEGER,
    is_confirmed BOOLEAN,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT id_primary_key PRIMARY KEY (id),
    FOREIGN KEY (customer_id) REFERENCES customers(id)
);
