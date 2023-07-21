ALTER TABLE bank_accounts
  ADD COLUMN branch_id INTEGER NOT NULL REFERENCES bank_branches(id),
  ADD COLUMN account_number VARCHAR(34) NOT NULL;

ALTER TABLE bank_account_balance_history
  ADD COLUMN transaction_id INTEGER NOT NULL REFERENCES transactions(id);
