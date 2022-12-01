CREATE SCHEMA internal;

GRANT ALL PRIVILEGES ON SCHEMA internal TO postgres;

CREATE TABLE internal.database_user (
    id SERIAL PRIMARY KEY,
    email email NOT NULL,
    password_hash VARCHAR(60) NOT NULL
);
