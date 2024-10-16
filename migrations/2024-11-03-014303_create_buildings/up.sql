-- Your SQL goes here

CREATE TABLE buildings (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    level INTEGER NOT NULL,
    fortress_id INTEGER NOT NULL REFERENCES fortresses(id)
);
