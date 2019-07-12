CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    descriton VARCHAR NULL,
    user_id SERIAL NOT NULL REFERENCES users(id) ON UPDATE CASCADE,
    created_at timestamp default current_timestamp
)