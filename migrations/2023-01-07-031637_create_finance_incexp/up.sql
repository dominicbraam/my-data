CREATE TABLE finance_incexp (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  person_id INTEGER NOT NULL REFERENCES person (id),
  label TEXT NOT NULL,
  item_link TEXT NOT NULL,
  -- amount REAL(15,2) NOT NULL,
  amount REAL NOT NULL,
  currency_id SMALLINT NOT NULL,
  transaction_type SMALLINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);