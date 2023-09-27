-- Add up migration script here
CREATE TYPE customer_role AS ENUM ('admin', 'staff', 'viewer');

CREATE TABLE customers (
    id uuid PRIMARY KEY ,
    firstname varchar(255) NOT NULL,
    lastname varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    "role" customer_role NOT NULL,

    UNIQUE (email)
);