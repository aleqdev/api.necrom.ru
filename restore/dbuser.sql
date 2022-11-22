CREATE SCHEMA internal
AUTHORIZATION postgres;

CREATE TABLE internal.database_user (
    id SERIAL PRIMARY KEY,
    email VARCHAR(254) NOT NULL,
    password_hash VARCHAR(60) NOT NULL
);
