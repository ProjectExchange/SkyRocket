# Dockerfiles used within this project

## Development setup

To easily get started with the project, you can simply create a `.env` file with the following content inside this directory:

```env
MYSQL_ROOT_PASSWORD: superSecretPassword
MYSQL_DATABASE: skyrocket
MYSQL_USER: skyrocket
MYSQL_PASSWORD: skyrocket
```

Make sure to adapt to variables to your needs. Afterwards, simply run

```sh
docker-compose -f docker-compose.dev.yml up -d
```

This will spawn a clean mariadb instance with adminer webinterface.
