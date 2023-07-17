CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  group_id INTEGER REFERENCES transaction_groups(id),
  account_id INTEGER NOT NULL REFERENCES bank_accounts(id),
  action_id INTEGER NOT NULL REFERENCES transaction_actions(id),
  tag_id INTEGER REFERENCES transaction_tags(id),
  product_id INTEGER REFERENCES transaction_products(id),
  document_id INTEGER REFERENCES documents(id),
  amount DECIMAL(10, 2) NOT NULL,
  transaction_date DATE NOT NULL,
  description TEXT
);
