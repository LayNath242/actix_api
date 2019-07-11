CREATE TABLE courses (
    id SERIAL PRIMARY KEY,
    title VARCHAR(120) NOT NULL,
    price FLOAT NOT NULL,
    thumbnail VARCHAR NULL,
    video_url VARCHAR NOT NULL,
    description VARCHAR NULL,
    cat_id SERIAL REFERENCES categories(id) ON UPDATE CASCADE,
    created_at timestamp default current_timestamp
);