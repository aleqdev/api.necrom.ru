
CREATE TABLE worker_role (
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL
);

CREATE TABLE worker (
    id SERIAL PRIMARY KEY, 

    name VARCHAR(50) NOT NULL,
    surname VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,

    phone_number VARCHAR(16) NOT NULL,
    email VARCHAR(254),

    role_id INTEGER NOT NULL REFERENCES worker_role (id)
);

CREATE TABLE hotel (
    id SERIAL PRIMARY KEY, 
    name VARCHAR(200) NOT NULL,
    city_id BIGINT NOT NULL REFERENCES city (id),
    owner_id INTEGER NOT NULL REFERENCES worker (id),
    description TEXT
);
