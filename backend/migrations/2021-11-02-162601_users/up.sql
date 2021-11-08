-- Your SQL goes here
CREATE TABLE `users` (
    `id` INT(255) NOT NULL AUTO_INCREMENT,
    `firstname` VARCHAR(255) NOT NULL,
    `lastname` VARCHAR(255) NOT NULL,
    `email` VARCHAR(255) NOT NULL,
    `password`, VARCHAR(255) NOT NULL,
    PRIMARY KEY (`id`)
);
