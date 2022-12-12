CREATE FUNCTION tour_order_turnover_ordered(tour, date_begin TIMESTAMP with time zone DEFAULT '-infinity', date_end TIMESTAMP with time zone DEFAULT 'infinity')
RETURNS BIGINT AS $$
    SELECT SUM(tour_order.people_count) 
    FROM tour_order
    WHERE (tour_order.crt_date BETWEEN date_begin AND date_end) AND tour_order.tour_id = $1.id;
$$ LANGUAGE SQL IMMUTABLE;
