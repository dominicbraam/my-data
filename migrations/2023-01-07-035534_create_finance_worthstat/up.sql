CREATE TABLE finance_worthstat (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  person_id INTEGER NOT NULL REFERENCES person (id),
  -- amount REAL(15,2) NOT NULL,
  amount REAL NOT NULL,
  currency_id SMALLINT NOT NULL REFERENCES finance_currency (id)
);
