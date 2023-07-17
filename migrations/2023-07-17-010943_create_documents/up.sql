CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES persons(id),
    document_type_id INTEGER NOT NULL REFERENCES document_types(id),
    file_path TEXT NOT NULL,
    description TEXT,
    uploaded_at TIMESTAMP NOT NULL
);
