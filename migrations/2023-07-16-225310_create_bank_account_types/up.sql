CREATE TABLE IF NOT EXISTS financial.bank_account_types (
    id SERIAL PRIMARY KEY,
    type VARCHAR(50) NOT NULL
);

INSERT INTO financial.bank_account_types (type) VALUES
  ('Checking'),
  ('Savings'),
  ('Credit Card');
