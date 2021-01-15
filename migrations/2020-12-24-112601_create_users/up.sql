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
  is_admin BOOLEAN NOT NULL DEFAULT 'f',
  verified BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE groups (
                        gid SERIAL PRIMARY KEY,
                        title VARCHAR NOT NULL
);

CREATE TABLE belongs (
                         gid INTEGER NOT NULL,
                         uid INTEGER NOT NULL,
                         PRIMARY KEY (gid, uid),
                         FOREIGN KEY (gid)
                             REFERENCES groups(gid)
                             ON DELETE CASCADE,
                         FOREIGN KEY (uid)
                             REFERENCES users(id)
                             ON DELETE CASCADE
);