-- Your SQL goes here
CREATE TABLE `users` (
    `id` INT(255) NOT NULL AUTO_INCREMENT,
    `firstname` VARCHAR(255) NOT NULL,
    `lastname` VARCHAR(255) NOT NULL,
    `email` VARCHAR(255) UNIQUE NOT NULL,
    `birthday` DATE NOT NULL,
    `gender` enum('male', 'female', 'diverse') NOT NULL,
    PRIMARY KEY (`id`)
);

CREATE TABLE `users_oauth_github` (
    `user_id` INT(255) NOT NULL UNIQUE,
    `github_id` INT(255) NOT NULL UNIQUE,
    PRIMARY KEY (`github_id`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);

CREATE TABLE `users_roles` (
    `user_id` INT(255) NOT NULL,
    `role` enum('admin') NOT NULL, -- enum values must be written in snake_case
    PRIMARY KEY (`user_id`, `role`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);

CREATE TABLE `addresses` (
    `id` INT(255) NOT NULL,
    `user_id` INT(255) NOT NULL,
    `country` VARCHAR(255) NOT NULL,
    `postal_code` INT(255) NOT NULL,
    `town` VARCHAR(255) NOT NULL,
    `street` VARCHAR(255) NOT NULL,
    `house_number` INT(255) NOT NULL,
    PRIMARY KEY (`id`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);

CREATE TABLE `flights_offers` (
    `id` INT(255) NOT NULL,
    `seats` INT(255) NOT NULL,
    `price` FLOAT(7,2) NOT NULL,
    `currency` enum('dollar', 'euro') NOT NULL,
    PRIMARY KEY (`id`)
);

CREATE TABLE `flights` (
    `id` INT(255) NOT NULL,
    `offer_id` INT(255) NOT NULL,
    `departure_icao` VARCHAR(4) NOT NULL,
    `departure_time` DATETIME NOT NULL,
    `arrival_icao` VARCHAR(4) NOT NULL,
    `arrival_time` DATETIME NOT NULL,
    PRIMARY KEY (`id`),
    FOREIGN KEY (`offer_id`) REFERENCES `flights_offers` (`id`)
);

CREATE TABLE `bookings`(
    `user_id` INT(255) NOT NULL,
    `offer_id` INT(255) NOT NULL,
    `seats` INT(255) NOT NULL,
    PRIMARY KEY (`user_id`, `offer_id`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
    FOREIGN KEY (`offer_id`) REFERENCES `flights_offers` (`id`)
);

CREATE TABLE `sessions`(
    `user_id` INT(255) NOT NULL,
    `redis_key` VARCHAR(255) NOT NULL,
    `established` DATETIME NOT NULL,
    `data` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`redis_key`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);
