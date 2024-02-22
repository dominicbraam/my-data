CREATE TABLE IF NOT EXISTS financial.bank_accounts (
    id SERIAL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES persons(id),
    account_type_id INTEGER NOT NULL REFERENCES financial.bank_account_types(id),
    currency_id INTEGER NOT NULL REFERENCES financial.currencies(id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);
