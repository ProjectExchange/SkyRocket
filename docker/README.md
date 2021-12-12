# Dockerfiles used within this project

## Development setup

To easily get started with the project, you can simply create a `.env` file with the following content inside this directory:

```env
MYSQL_ROOT_PASSWORD=superSecretPassword
MYSQL_DATABASE=skyrocket
MYSQL_USER=skyrocket
MYSQL_PASSWORD=skyrocket
```

Since this application encrypts its database by default, some configuration has to be done before
the development can start. Inside the `config/mysql/encryption` folder execute the commands documented
[here](../docs/setup/DbEncryption.md#encryption-at-rest).

In the end, your folder should have the following content:

```sh
$ ls config/mysql/encryption/
keyfile.enc  keyfile.key
```

Make sure to adapt the variables inside `docker-compose.dev.yml` to your needs. Afterwards, simply run

```sh
docker-compose -f docker-compose.dev.yml up -d
```

This will spawn a clean mariadb instance with adminer webinterface.
