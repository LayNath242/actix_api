-- Your SQL goes here
CREATE TABLE roles (
    Id SERIAL PRIMARY KEY,
    Title VARCHAR(100) NOT NULL,
    created_at timestamp default current_timestamp
);