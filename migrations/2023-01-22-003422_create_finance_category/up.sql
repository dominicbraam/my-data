CREATE TABLE finance_category (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name TEXT NOT NULL,
  transaction_type_id SMALLINT NOT NULL REFERENCES finance_transaction_type (id)
);
