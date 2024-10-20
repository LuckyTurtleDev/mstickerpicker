CREATE TABLE files (
  hash BYTEA UNIQUE NOT NULL,
  mxc TEXT UNIQUE NOT NULL,
  id BIGSERIAL UNIQUE,
  PRIMARY KEY (id)
);

CREATE INDEX
ON files(hash);

CREATE TABLE file_owner (
  user_id INTEGER NOT NULL,
  file_id BIGINT NOT NULL,
  FOREIGN KEY(file_id)
    REFERENCES files(id)
    ON DELETE CASCADE,
  FOREIGN KEY(user_id)
    REFERENCES users(id)
    ON DELETE CASCADE
);

CREATE INDEX
ON file_owner(user_id);

CREATE INDEX
ON file_owner(file_id);