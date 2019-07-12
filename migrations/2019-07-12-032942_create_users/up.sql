CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  fullname VARCHAR(100) NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL,
  password VARCHAR(64) NOT NULL,
  avatar VARCHAR NULL,
  biography VARCHAR NULL,
  created_at TIMESTAMP NOT NULL,
  role_id SERIAL NOT NULL REFERENCES roles(id) ON UPDATE CASCADE DEFAULT 3
);
CREATE INDEX users_email_name_idx ON users (email, fullname);