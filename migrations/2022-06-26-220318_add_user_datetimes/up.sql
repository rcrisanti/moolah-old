ALTER TABLE users 
    ADD COLUMN created TIMESTAMP NOT NULL,
    ADD COLUMN last_login TIMESTAMP NOT NULL;