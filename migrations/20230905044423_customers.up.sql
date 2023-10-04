CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE customer_role AS ENUM ('admin', 'staff', 'viewer');

CREATE TABLE customers (
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    firstname varchar(255) NOT NULL,
    lastname varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    "role" customer_role NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE (email)
);