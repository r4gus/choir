-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  password_hash VARCHAR NOT NULL,
  first_name VARCHAR,
  last_name VARCHAR,
  street VARCHAR,
  house_number VARCHAR,
  zip VARCHAR,
  city VARCHAR,
  phone VARCHAR,
  is_admin BOOLEAN NOT NULL DEFAULT 'f'
);
