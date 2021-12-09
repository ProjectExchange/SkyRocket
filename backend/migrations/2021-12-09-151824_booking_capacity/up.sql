CREATE VIEW flights_offers_with_capacity AS
SELECT
    flights_offers.*, SUM(bookings.seats) as occupied
FROM
    flights_offers, bookings
WHERE bookings.offer_id = flights_offers.id
GROUP BY flights_offers.id;
