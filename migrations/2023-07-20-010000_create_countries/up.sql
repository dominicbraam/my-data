CREATE TABLE IF NOT EXISTS countries (
    id SERIAL PRIMARY KEY,
    record_group INTEGER NOT NULL,
    is_current BOOLEAN NOT NULL,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(3) NOT NULL,
    currency_main_id INTEGER NOT NULL REFERENCES financial.currencies(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO countries (record_group,is_current,name,code,currency_main_id) VALUES
  (1,True,'Guyana','GY',(SELECT id FROM financial.currencies WHERE code = 'GYD')),
  (2,False,'United States of America','USA',(SELECT id FROM financial.currencies WHERE code = 'USD'));
