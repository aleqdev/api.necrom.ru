CREATE FUNCTION tour_order_turnover_selled(tour, date_begin TIMESTAMP with time zone DEFAULT '-infinity', date_end TIMESTAMP with time zone DEFAULT 'infinity')
RETURNS BIGINT AS $$
    SELECT SUM(tour_order_purchase.people_count) 
    FROM tour_order_purchase
    WHERE (tour_order_purchase.crt_date BETWEEN date_begin AND date_end) AND tour_order_purchase.tour_id = $1.id;
$$ LANGUAGE SQL IMMUTABLE;
