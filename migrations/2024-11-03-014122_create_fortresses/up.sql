-- Your SQL goes here

CREATE TABLE fortresses (
    id SERIAL PRIMARY KEY,
    gold INTEGER NOT NULL,
    food INTEGER NOT NULL,
    wood INTEGER NOT NULL,
    energy INTEGER NOT NULL
);
