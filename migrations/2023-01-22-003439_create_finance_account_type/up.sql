CREATE TABLE finance_account_type (
  id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name TEXT NOT NULL
);

INSERT INTO finance_account_type (name) VALUES ('Checking');
INSERT INTO finance_account_type (name) VALUES ('Savings');
INSERT INTO finance_account_type (name) VALUES ('Credit Card');
