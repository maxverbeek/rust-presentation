-- Add migration script here
create table macusers (
  id bigserial primary key,
  name varchar(255) unique not null,
  macaddr varchar(128) unique not null
);
