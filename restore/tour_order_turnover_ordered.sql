CREATE FUNCTION tour_order_turnover_ordered(tour, date_begin TIMESTAMP with time zone DEFAULT '-infinity', date_end TIMESTAMP with time zone DEFAULT 'infinity')
RETURNS TABLE (
  crt_date TIMESTAMP with time zone,
  people_count INTEGER
) AS $$
    SELECT tour_order.crt_date, tour_order.people_count
    FROM tour_order
    WHERE (tour_order.crt_date BETWEEN date_begin AND date_end) AND tour_order.tour_id = $1.id;
$$ LANGUAGE SQL IMMUTABLE;
