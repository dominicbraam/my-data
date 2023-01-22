CREATE TABLE finance_currency (
  id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name TEXT NOT NULL,
  abbreviation TEXT NOT NULL
);

INSERT INTO finance_currency (name,abbreviation) VALUES ('Guyanese Dollar','GYD');
INSERT INTO finance_currency (name,abbreviation) VALUES ('United States Dollar','USD');
