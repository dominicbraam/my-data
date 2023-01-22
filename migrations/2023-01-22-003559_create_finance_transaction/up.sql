CREATE TABLE finance_transaction (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  account_id INTEGER NOT NULL REFERENCES finance_account (id),
  description TEXT NOT NULL,
  bank_description TEXT,
  item_link TEXT,
  amount REAL NOT NULL,
  tentative_amount REAL,
  is_amount_tentative BOOLEAN NOT NULL,
  category_id INTEGER NOT NULL REFERENCES finance_category (id),
  currency_id SMALLINT NOT NULL REFERENCES finance_currency (id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP
);
