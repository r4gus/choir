-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  street VARCHAR NOT NULL,
  house_number VARCHAR NOT NULL,
  zip VARCHAR NOT NULL,
  city VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  is_admin BOOLEAN NOT NULL DEFAULT 'f'
);
