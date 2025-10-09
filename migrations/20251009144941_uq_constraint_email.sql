-- Add migration script here
alter table customers
    add constraint customers_pk
        unique (email);