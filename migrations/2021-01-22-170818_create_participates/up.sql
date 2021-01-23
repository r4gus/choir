-- Your SQL goes here
CREATE TABLE participates (
    aid INTEGER NOT NULL,
    gid INTEGER NOT NULL,
    uid INTEGER NOT NULL,
    PRIMARY KEY (aid, uid),
    FOREIGN KEY (aid) REFERENCES appointments(id) ON DELETE CASCADE,
    FOREIGN KEY (gid) REFERENCES groups(gid) ON DELETE CASCADE,
    FOREIGN KEY (uid) REFERENCES users(id) ON DELETE CASCADE
);