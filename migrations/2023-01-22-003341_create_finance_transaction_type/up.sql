CREATE TABLE finance_transaction_type (
  id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name TEXT NOT NULL
);

INSERT INTO finance_transaction_type (name) VALUES ('income');
INSERT INTO finance_transaction_type (name) VALUES ('expense');
