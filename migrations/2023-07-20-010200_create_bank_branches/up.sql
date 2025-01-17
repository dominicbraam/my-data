CREATE TABLE IF NOT EXISTS financial.bank_branches (
    id SERIAL PRIMARY KEY,
    record_group INTEGER NOT NULL,
    is_current BOOLEAN NOT NULL DEFAULT TRUE,
    bank_id INTEGER NOT NULL REFERENCES financial.banks(id),
    name VARCHAR(100) NOT NULL,
    street TEXT,
    city TEXT NOT NULL,
    state VARCHAR(50),
    postal_code VARCHAR(20),
    country_id INTEGER NOT NULL REFERENCES countries(id),
    swift VARCHAR(11),
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO financial.bank_branches (record_group,bank_id,name,city,country_id,swift) VALUES
  (1,(SELECT id FROM financial.banks WHERE name = 'BANK OF GUYANA'),'BANK OF GUYANA, GEORGETOWN','GEORGETOWN',(SELECT id FROM countries WHERE name = 'Guyana'),'GUBAGYGG'),
  (2,(SELECT id FROM financial.banks WHERE name = 'BANK OF NOVA SCOTIA,THE'),'BANK OF NOVA SCOTIA,THE, GEORGETOWN','GEORGETOWN',(SELECT id FROM countries WHERE name = 'Guyana'),'NOSCGYGE'),
  (3,(SELECT id FROM financial.banks WHERE name = 'CITIZENS BANK GUYANA INC'),'CITIZENS BANK GUYANA INC, GEORGETOWN','GEORGETOWN',(SELECT id FROM countries WHERE name = 'Guyana'),'CBGIGYGG'),
  (4,(SELECT id FROM financial.banks WHERE name = 'DEMERARA BANK LIMITED'),'DEMERARA BANK LIMITED, GEORGETOWN','GEORGETOWN',(SELECT id FROM countries WHERE name = 'Guyana'),'DMBKGYGT'),
  (5,(SELECT id FROM financial.banks WHERE name = 'GUYANA BANK FOR TRADE AND INDUSTRY LTD.'),'(MAIN BRANCH)','GEORGETOWN',(SELECT id FROM countries WHERE name = 'Guyana'),'GUTIGYGE'),
  (6,(SELECT id FROM financial.banks WHERE name = 'REPUBLIC BANK (GUYANA) LIMITED'),'REPUBLIC BANK (GUYANA) LIMITED, GEORGETOWN','GEORGETOWN',(SELECT id FROM countries WHERE name = 'Guyana'),'RBGLGYGG');
