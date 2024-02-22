CREATE TABLE IF NOT EXISTS financial.transactions (
  id SERIAL PRIMARY KEY,
  group_id INTEGER REFERENCES financial.transaction_groups(id),
  account_id INTEGER NOT NULL REFERENCES financial.bank_accounts(id),
  action_id INTEGER NOT NULL REFERENCES financial.transaction_actions(id),
  tag_id INTEGER REFERENCES financial.transaction_tags(id),
  product_id INTEGER REFERENCES financial.transaction_products(id),
  document_id INTEGER REFERENCES documents(id),
  is_need BOOLEAN,
  amount DECIMAL(10, 2) NOT NULL,
  transaction_datetime TIMESTAMP NOT NULL,
  description TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP
);
