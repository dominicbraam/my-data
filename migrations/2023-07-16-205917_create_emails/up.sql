CREATE TABLE emails (
    id SERIAL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES persons(id),
    email VARCHAR(255) UNIQUE NOT NULL
);