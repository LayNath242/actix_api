-- Your SQL goes here
CREATE TABLE question_answer (
    id SERIAL PRIMARY KEY,
    title VARCHAR NULL,
    descriton VARCHAR NULL,
    user_id SERIAL NOT NULL REFERENCES users(id) ON UPDATE CASCADE,
    created_at timestamp default current_timestamp
)