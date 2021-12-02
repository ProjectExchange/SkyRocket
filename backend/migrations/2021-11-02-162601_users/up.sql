-- Your SQL goes here
CREATE TABLE `users` (
    `id` INT(255) NOT NULL AUTO_INCREMENT,
    `firstname` VARCHAR(255) NOT NULL,
    `lastname` VARCHAR(255) NOT NULL,
    `email` VARCHAR(255) UNIQUE NOT NULL,
    PRIMARY KEY (`id`)
);

CREATE TABLE `users_oauth_github` (
    `user_id` INT(255) NOT NULL UNIQUE,
    `github_id` INT(255) NOT NULL UNIQUE,
    PRIMARY KEY (`github_id`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);
