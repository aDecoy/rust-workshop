-- Add migration script here
CREATE TABLE users (
    email_address VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);