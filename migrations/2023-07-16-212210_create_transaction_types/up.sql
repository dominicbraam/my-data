CREATE TABLE transaction_types (
    id SERIAL PRIMARY KEY,
    type VARCHAR(50) NOT NULL
);

INSERT INTO transaction_types (type) VALUES ('income'), ('expenditure');
