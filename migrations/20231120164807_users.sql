-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    username VARCHAR(25) NOT NULL,
    password VARCHAR(255) NOT NULL
);

INSERT INTO users (username, password) VALUES ('john', 'password211');
INSERT INTO users (username, password) VALUES ('smith', '112drowssap');