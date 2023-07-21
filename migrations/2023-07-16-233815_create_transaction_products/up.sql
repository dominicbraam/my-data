CREATE TABLE transaction_products (
    id SERIAL PRIMARY KEY,
    record_group INTEGER NOT NULL,
    is_current BOOLEAN NOT NULL DEFAULT TRUE,
    name VARCHAR(100) NOT NULL,
    product_link TEXT,
    description TEXT,
    price DECIMAL(12, 2) NOT NULL,
    currency_id INTEGER NOT NULL REFERENCES currencies(id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
