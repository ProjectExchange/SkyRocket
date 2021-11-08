-- Your SQL goes here
CREATE TABLE `users` (
    `id` INT(255) UNSIGNED NOT NULL AUTO_INCREMENT,
    `email` VARCHAR(255) NOT NULL,
    `firstname` VARCHAR(255) NOT NULL,
    `lastname` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`id`)
);
