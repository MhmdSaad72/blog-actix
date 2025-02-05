-- Your SQL goes here
CREATE SEQUENCE IF NOT EXISTS users_id_seq;
CREATE TABLE users (
    id int8 NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    user_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);
CREATE UNIQUE INDEX email_unique_idx ON users (email);
CREATE UNIQUE INDEX user_name_unique_idx ON users (user_name);
