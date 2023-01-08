CREATE TABLE finance_currency (
  id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  label TEXT NOT NULL,
  abbreviation TEXT NOT NULL
);

INSERT INTO finance_currency (label,abbreviation) VALUES ('Guyanese Dollar','GYD');
INSERT INTO finance_currency (label,abbreviation) VALUES ('United States Dollar','USD');
