CREATE TABLE document_types (
    id SERIAL PRIMARY KEY,
    type VARCHAR(50) NOT NULL
);

INSERT INTO document_types (type) VALUES
  ('Receipt'),
  ('Invoice'),
  ('Contract'),
  ('Identification'),
  ('Bank Statement'),
  ('Tax Document'),
  ('Insurance Policy'),
  ('Medical Record'),
  ('Certificate'),
  ('Legal Document'),
  ('Vehicle Ownership'),
  ('Cryptocurrency'),
  ('Other');
