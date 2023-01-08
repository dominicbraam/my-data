CREATE TABLE finance_incexp (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  person_id INTEGER NOT NULL REFERENCES person (id),
  label TEXT NOT NULL,
  item_link TEXT NOT NULL,
  amount REAL NOT NULL,
  category_id INTEGER NOT NULL REFERENCES finance_category (id),
  currency_id SMALLINT NOT NULL REFERENCES finance_currency (id),
  transaction_type_id SMALLINT NOT NULL REFERENCES finance_transaction_type (id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP
);
