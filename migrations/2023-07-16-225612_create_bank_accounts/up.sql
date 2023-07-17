CREATE TABLE bank_accounts (
    id SERIAL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES persons(id),
    account_type_id INTEGER NOT NULL REFERENCES bank_account_types(id),
    currency_id INTEGER NOT NULL REFERENCES currencies(id),
    balance DECIMAL(12, 2) NOT NULL DEFAULT 0.00
);
