CREATE TABLE currencies (
  id SERIAL PRIMARY KEY,
  code CHAR(3) UNIQUE NOT NULL,
  name VARCHAR(50) NOT NULL
);

INSERT INTO currencies (code,name) VALUES
  ('GYD','Guyanese Dollar'),
  ('USD','United States Dollar');
