CREATE TABLE wishlist (
    id SERIAL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES persons(id),
    product_id INTEGER REFERENCES transaction_products(id),
    list_order INTEGER NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);
