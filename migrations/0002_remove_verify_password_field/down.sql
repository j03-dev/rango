CREATE TABLE IF NOT EXISTS __rusql_alchemy_tmp_User_ (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, username VARCHAR(255) UNIQUE NOT NULL, email VARCHAR(100) UNIQUE NOT NULL, password VARCHAR(255) NOT NULL, verify_password VARCHAR(255) NOT NULL, created_at VARCHAR(40) DEFAULT current_timestamp NOT NULL, is_admin BOOL DEFAULT 0 NOT NULL);
INSERT INTO __rusql_alchemy_tmp_User_ (id, username, email, password, created_at, is_admin) SELECT id, username, email, password, created_at, is_admin FROM User_;
DROP TABLE User_;
ALTER TABLE __rusql_alchemy_tmp_User_ RENAME TO User_;