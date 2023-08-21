-- Add migration script here
CREATE TABLE customers (
    id uuid PRIMARY KEY ,
    username varchar(255) NOT NULL,
    password varchar(255) NOT NULL
);