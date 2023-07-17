CREATE TABLE bank_account_balance_history (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES bank_accounts(id),
    balance DECIMAL(12, 2) NOT NULL,
    record_date TIMESTAMP NOT NULL
);
