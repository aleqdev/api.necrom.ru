CREATE VIEW tour_order_turnover AS
SELECT
    *,
    price * people_count AS cost
FROM tour;