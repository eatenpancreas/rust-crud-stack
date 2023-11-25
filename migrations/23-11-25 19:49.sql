CREATE DATABASE example_db;

\c example_db;

CREATE TABLE users (
    id serial PRIMARY KEY,
    username varchar(20) UNIQUE not null,
    password varchar(255) not null,
    email varchar(255) unique not null,
    created_on timestamp NOT NULL,
    last_login timestamp
)