CREATE TABLE IF NOT EXISTS documents (
    id SERIAL PRIMARY KEY,
    record_group INTEGER NOT NULL,
    is_current BOOLEAN NOT NULL DEFAULT TRUE,
    person_id INTEGER NOT NULL REFERENCES persons(id),
    document_type_id INTEGER NOT NULL REFERENCES document_types(id),
    file_path TEXT NOT NULL,
    description TEXT,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
