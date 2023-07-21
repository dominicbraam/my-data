CREATE TABLE countries (
    id SERIAL PRIMARY KEY,
    record_group INTEGER NOT NULL,
    is_current BOOLEAN NOT NULL DEFAULT TRUE,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(3) NOT NULL,
    currency_main_id INTEGER NOT NULL REFERENCES currencies(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO countries (record_group,name,code,currency_main_id) VALUES
  (1,'Guyana','GY',(SELECT id FROM currencies WHERE code = 'GYD')),
  (2,'United States of America','USA',(SELECT id FROM currencies WHERE code = 'USD'));
