-- Add migration script here
alter table orders
    add status varchar(16);