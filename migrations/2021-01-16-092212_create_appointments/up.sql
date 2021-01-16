-- Your SQL goes here
CREATE TABLE appointments (
    id SERIAL PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    place VARCHAR NOT NULL,
    begins TIMESTAMP NOT NULL,
    ends TIMESTAMP NOT NULL,
    description VARCHAR NOT NULL
);