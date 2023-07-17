CREATE TABLE transaction_products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    product_link TEXT,
    description TEXT,
    price DECIMAL(12, 2) NOT NULL,
    currency_id INTEGER NOT NULL REFERENCES currencies(id)
);
