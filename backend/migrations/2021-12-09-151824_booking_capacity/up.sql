CREATE VIEW flights_offers_with_occupancy AS
SELECT
    fo.*,
    COALESCE(sum(bookings.seats), 0) AS occupied,
    COALESCE((SELECT flights.departure_icao FROM flights WHERE flights.offer_id = fo.id ORDER BY flights.departure_time LIMIT 1), '') AS departure_icao,
    COALESCE((SELECT flights.arrival_icao FROM flights WHERE flights.offer_id = fo.id ORDER BY flights.arrival_time DESC LIMIT 1), '') AS arrival_icao
FROM flights_offers AS fo    
LEFT JOIN bookings ON bookings.offer_id = fo.id
GROUP BY fo.id
