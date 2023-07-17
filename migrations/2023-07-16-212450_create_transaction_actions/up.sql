CREATE TABLE transaction_actions (
  id SERIAL PRIMARY KEY,
  actions VARCHAR(50) NOT NULL,
  transaction_type_id INTEGER NOT NULL REFERENCES transaction_types(id)
);

INSERT INTO transaction_actions (actions,transaction_type_id) VALUES 
  ('deposit',1), 
  ('withdrawal',2), 
  ('fee',2), 
  ('interest',2), 
  ('refund',1), 
  ('payment',2);
