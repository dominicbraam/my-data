CREATE TABLE banks (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

INSERT INTO banks (name) VALUES
  ('BANK OF GUYANA'),
  ('BANK OF NOVA SCOTIA,THE'),
  ('CITIZENS BANK GUYANA INC'),
  ('DEMERARA BANK LIMITED'),
  ('GUYANA BANK FOR TRADE AND INDUSTRY LTD.'),
  ('REPUBLIC BANK (GUYANA) LIMITED');