ALTER TABLE financial.bank_accounts
  DROP COLUMN branch_id,
  DROP COLUMN account_number;

ALTER TABLE financial.bank_account_balance_history
  DROP COLUMN transaction_id;
