-- Add migration script here
alter table orders
    alter column customer_id type bigint using customer_id::bigint;

