#!/bin/bash

echo "Configure MySQL encryption at rest"

scripts_dir="$PWD"
dir="../docker/config/mysql/encryption"
[[ -d $dir ]] || mkdir $dir
cd $dir

echo "1;$(openssl rand -hex 32)" > keyfile

openssl rand -hex 128 > keyfile.key

openssl enc -aes-256-cbc -md sha1 \
   -pass file:keyfile.key \
   -in keyfile \
   -out keyfile.enc

shred keyfile && rm keyfile

cd $scripts_dir

echo "Configure MySQL encryption in transit"

./certs.sh

echo "Generate configuration"

cd ../docker

echo "MYSQL_ROOT_PASSWORD=$(openssl rand -base64 128 | tr -d '\n')" >> .env
echo "Enter MySQL database name: "
read mysql_database
echo "MYSQL_DATABASE=${mysql_database}" >> .env
echo "Enter MySQL database user: "
read mysql_user
echo "MYSQL_USER=${mysql_user}" >> .env
echo "Enter MySQL database password: "
read -s mysql_password
echo "MYSQL_PASSWORD=${mysql_password}" >> .env

echo "ROCKET_SECRET_KEY=$(openssl rand -base64 32)" >> .env
echo "ROCKET_DATABASES={skyrocket={url=\"mysql://${mysql_user}:${mysql_password}@db/${mysql_database}\"}}" >> .env
echo "Enter redis database password: "
read -s redis_password
echo "REDIS_URL=redis://:${redis_password}@redis/" >> .env
echo "requirepass ${redis_password}" >> config/redis/redis.conf

echo "Enter GitHub Client ID: "
read -s github_client_id
echo "OAUTH_GITHUB_CLIENT_ID=${github_client_id}" >> .env
echo "Enter GitHub Client secret: "
read -s github_client_secret
echo "OAUTH_GITHUB_CLIENT_SECRET=${github_client_secret}" >> .env

echo -e "Now you are good to go, enter \u001b[34mdocker-compose up -d\u001b[0m to start."
