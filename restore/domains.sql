CREATE DOMAIN email AS VARCHAR(254) CHECK (VALUE ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$');
CREATE DOMAIN phone_number AS VARCHAR(16) CHECK (VALUE ~* '^[0-9]+$');
CREATE TYPE tour_order_status AS ENUM ('active', 'canceled', 'completed');
