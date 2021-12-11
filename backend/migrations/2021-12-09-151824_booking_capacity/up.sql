CREATE VIEW flights_offers_with_occupancy AS
SELECT
    fo.*, SUM(bookings.seats) as occupied,
    (SELECT flights.departure_icao FROM flights WHERE flights.offer_id=fo.id ORDER BY departure_time ASC LIMIT 1) AS departure_icao,
    (SELECT flights.arrival_icao FROM flights WHERE flights.offer_id=fo.id ORDER BY arrival_time DESC LIMIT 1) AS arrival_icao
FROM
    flights_offers AS fo, bookings
WHERE bookings.offer_id = fo.id
GROUP BY fo.id
