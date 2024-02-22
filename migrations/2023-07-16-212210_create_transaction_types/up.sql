CREATE TABLE IF NOT EXISTS financial.transaction_types (
    id SERIAL PRIMARY KEY,
    type VARCHAR(50) NOT NULL
);

INSERT INTO financial.transaction_types (type) VALUES ('income'), ('expenditure');
