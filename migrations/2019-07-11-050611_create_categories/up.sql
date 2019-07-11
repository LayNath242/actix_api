CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    user_id SERIAL REFERENCES users(id) ON UPDATE CASCADE,
    created_at timestamp default current_timestamp
);