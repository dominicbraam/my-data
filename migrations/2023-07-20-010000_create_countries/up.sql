CREATE TABLE countries (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(3) NOT NULL,
    capital VARCHAR(100) NOT NULL,
    currency_main_id INTEGER NOT NULL REFERENCES currencies(id)
);

INSERT INTO countries (name,code,capital,currency_main_id) VALUES
  ('Guyana','GY','Georgetown',1),
  ('United States of America','USA','Washington, D.C.',2);
