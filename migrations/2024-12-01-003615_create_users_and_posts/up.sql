-- Your SQL goes here
CREATE TABLE
    users (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        email VARCHAR NOT NULL
    );

CREATE TABLE
    posts (
        id SERIAL PRIMARY KEY,
        title VARCHAR NOT NULL,
        body TEXT NOT NULL,
        user_id INTEGER NOT NULL REFERENCES users (id)
    );