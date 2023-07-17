CREATE TABLE wishlist (
    id SERIAL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES persons(id),
    product_id INTEGER REFERENCES transaction_products(id),
    added_date DATE NOT NULL,
    list_order INTEGER NOT NULL
);
