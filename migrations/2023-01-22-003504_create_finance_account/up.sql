CREATE TABLE finance_account (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  person_id INT NOT NULL REFERENCES person (id),
  name TEXT NOT NULL,
  account_type_id SMALLINT NOT NULL REFERENCES finance_account_type (id),
  currency_id SMALLINT NOT NULL REFERENCES finance_currency (id)
);
