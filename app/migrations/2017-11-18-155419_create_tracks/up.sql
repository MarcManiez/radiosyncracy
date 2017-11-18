CREATE TABLE tracks (
  id SERIAL PRIMARY KEY,
  length integer,
  link VARCHAR NOT NULL,
  name VARCHAR,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
)
