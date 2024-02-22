CREATE TABLE IF NOT EXISTS address_types (
    id SERIAL PRIMARY KEY,
    type VARCHAR(50) NOT NULL
);

INSERT INTO address_types (type) VALUES
  ('Residential'),
  ('Work'),
  ('PO Box'),
  ('Temporary'),
  ('Other');
