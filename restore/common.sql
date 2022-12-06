
CREATE TABLE person (
    id SERIAL PRIMARY KEY, 

    name VARCHAR(50) NOT NULL,
    surname VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,

    phone_number phone_number,
    email email
);

CREATE TABLE employee_role (
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL
);

CREATE TABLE employee (
    id SERIAL PRIMARY KEY, 
    person_id INTEGER NOT NULL REFERENCES person (id),
    role_id INTEGER NOT NULL REFERENCES employee_role (id)
);

CREATE TABLE client_type (
    id SERIAL PRIMARY KEY, 
    name VARCHAR(50) NOT NULL
);

CREATE TABLE client (
    id SERIAL PRIMARY KEY, 
    type_id INTEGER NOT NULL REFERENCES client_type (id),
    person_id INTEGER NOT NULL REFERENCES person (id)
);

CREATE TABLE hotel (
    id SERIAL PRIMARY KEY, 
    name VARCHAR(200) NOT NULL,
    city_id BIGINT NOT NULL REFERENCES city (id),
    owner_id INTEGER NOT NULL REFERENCES person (id),
    description VARCHAR(500)
);

CREATE TABLE tour_feeding_type (
    id SERIAL PRIMARY KEY, 
    name VARCHAR(50) NOT NULL
);

CREATE TABLE tour (
    id SERIAL PRIMARY KEY, 
    hotel_id INTEGER NOT NULL REFERENCES hotel (id),
    arrival_date TIMESTAMP NOT NULL,
    departure_date TIMESTAMP NOT NULL,
    feeding_type_id INTEGER NOT NULL REFERENCES tour_feeding_type (id),
    cost DECIMAL(12, 2) NOT NULL,
    description VARCHAR(500)
);

CREATE TABLE tour_order_payment_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50)
);

CREATE TABLE tour_order_group (
  id SERIAL PRIMARY KEY
);

CREATE TABLE tour_order (
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES client (id),
    payment_type_id INTEGER NOT NULL REFERENCES tour_order_payment_type (id),
    tour_id INTEGER NOT NULL REFERENCES tour (id),
    price DECIMAL(12, 2) NOT NULL,
    people_count INTEGER NOT NULL,
    group_id INTEGER NOT NULL REFERENCES tour_order_group (id),
    status tour_order_status NOT NULL DEFAULT 'active'
);

CREATE VIEW tour_order_view AS
SELECT
    *,
    price * people_count AS cost
FROM tour_order;

CREATE TABLE tour_order_payment (
    tour_order_id INTEGER NOT NULL REFERENCES tour_order (id),
    money_received DECIMAL(12, 2) NOT NULL
);

CREATE TABLE tour_order_purchase (
    tour_order_id INTEGER NOT NULL REFERENCES tour_order (id),
    reservations_confirmed BOOLEAN NOT NULL DEFAULT FALSE
);
