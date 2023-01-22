CREATE TABLE finance_account_balance (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  account_id INT NOT NULL REFERENCES finance_account (id),
  amount REAL NOT NULL
);
