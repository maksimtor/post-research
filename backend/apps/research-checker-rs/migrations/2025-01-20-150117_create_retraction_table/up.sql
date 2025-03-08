-- Your SQL goes here
CREATE TABLE retraction (
  id SERIAL PRIMARY KEY,
  title TEXT,
  affiliations TEXT[] NOT NULL,
  journal TEXT,
  countries TEXT[] NOT NULL,
  authors TEXT[] NOT NULL,
  link TEXT,
  doi VARCHAR,
  pubmed_id VARCHAR,
  issues TEXT[] NOT NULL
)