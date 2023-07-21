CREATE TABLE transaction_actions (
  id SERIAL PRIMARY KEY,
  actions VARCHAR(50) NOT NULL,
  transaction_type_id INTEGER NOT NULL REFERENCES transaction_types(id)
);

INSERT INTO transaction_actions (actions,transaction_type_id) VALUES 
  ('Initial Balance',(SELECT id FROM transaction_types WHERE type = 'income')),
  ('Deposit',(SELECT id FROM transaction_types WHERE type = 'income')),
  ('Salary',(SELECT id FROM transaction_types WHERE type = 'income')),
  ('Withdrawal',(SELECT id FROM transaction_types WHERE type = 'expenditure')),
  ('Fee',(SELECT id FROM transaction_types WHERE type = 'expenditure')),
  ('Interest',(SELECT id FROM transaction_types WHERE type = 'income')),
  ('Refund',(SELECT id FROM transaction_types WHERE type = 'income')),
  ('Purchase',(SELECT id FROM transaction_types WHERE type = 'expenditure'));
