-- Your SQL goes here
CREATE SEQUENCE IF NOT EXISTS posts_id_seq;
CREATE TABLE posts (
    id int8 NOT NULL DEFAULT nextval('posts_id_seq'::regclass),
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    PRIMARY KEY (id)
);
