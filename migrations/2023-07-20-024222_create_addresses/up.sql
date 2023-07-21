CREATE TABLE addresses (
  id SERIAL PRIMARY KEY,
  person_id INTEGER REFERENCES persons(id),
  address_type_id INTEGER REFERENCES address_types(id),
  street VARCHAR(100) NOT NULL,
  city VARCHAR(50) NOT NULL,
  state VARCHAR(50),
  country_id INTEGER REFERENCES countries(id),
  postal_code VARCHAR(20),
  is_legal BOOLEAN NOT NULL DEFAULT false,
  is_billing BOOLEAN NOT NULL DEFAULT false,
  is_shipping BOOLEAN NOT NULL DEFAULT false,
  description TEXT,
  archived BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP
);
