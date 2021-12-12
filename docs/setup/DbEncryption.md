# Database Encryption

This file describes the process of encrypting MariaDB.

## Encryption at rest

[Documentation](https://mariadb.com/kb/en/file-key-management-encryption-plugin/)

To encrypt data at rest, we load the default integrated mariadb plugin `file_key_management`, which reads the encryption key from a file that we will create now

The file has the following format:

```
<encryption_key_id1>;<hex-encoded_encryption_key1>
```

The encryption key support AES 128-bit, 192-bit and 256-bit.

To generate a keyfile for a key with the key id 1, we can seimply execute

```sh
echo "1;$(openssl rand -hex 32)" > keyfile
```

This generates a 256-bit (32 byte) encryption key.


### Protect the encryption key

The keyfile contains our encryption key in plain text. In the next step, we password protect our keyfile.

To do so, we need to generate and store a password first:

```sh
openssl rand -hex 128 > keyfile.key
```

We can now encrypt our keyfile with the given key using the following command:

```sh
openssl enc -aes-256-cbc -md sha1 \
   -pass file:keyfile.key \
   -in keyfile \
   -out keyfile.enc
```

The keyfile can now be removed safely, as we only want to store the encrypted file and the corresponding password key.

To validate that the encryption worked properly for all defined tables, execute the following SQL command
as the SQL root user:

```sql
SELECT * FROM information_schema.INNODB_TABLESPACES_ENCRYPTION;
```

## Encryption in transit
[Documentation](https://mariadb.com/kb/en/securing-connections-for-client-and-server/)

To enable encryption in transit, we first need to generate ssl certificates for our mariadb server.
In addition to the normal TLS certificates, we also need a CA certificate.

### Generate server certificates
[Documentation](https://mariadb.com/docs/security/encryption/in-transit/create-self-signed-certificates-keys-openssl/)

We will start by generating a CA private key like this:

```sh
openssl genrsa 2048 > ca-key.pem
```

We will now generate a CA X.509 certificate from the private key:

```sh
openssl req -new -x509 -nodes -days 365 \
   -subj "/CN=ca.sys.skyrocket.projectexchange.org/C=DE/L=BW" \
   -key ca-key.pem \
   -out ca-cert.pem
```

We also need a private server key and signing request:

```sh
openssl req -newkey rsa:2048 -nodes -days 365 \
   -subj "/CN=mariadb-server.sys.skyrocket.projectexchange.org/C=DE/L=BW" \
   -keyout server-key.pem \
   -out server-req.pem
```

Those can be used to create our server certificate

```sh
openssl x509 -req -days 365 -set_serial 01 \
   -in server-req.pem \
   -out server-cert.pem \
   -CA ca-cert.pem \
   -CAkey ca-key.pem
```

### Generate client certificates

To authenticate our clients, we need to create a new client key and certificate.

The private key and signing request are generated using

```sh
openssl req -newkey rsa:2048 -nodes -days 365 \
   -subj "/CN=mariadb-client.sys.skyrocket.projectexchange.org/C=DE/L=BW" \
   -keyout client-key.pem \
   -out client-req.pem
```

and can be used to create a new X.509 certificate

```sh
openssl x509 -req -days 365 -set_serial 01 \
   -in client-req.pem \
   -out client-cert.pem \
   -CA ca-cert.pem \
   -CAkey ca-key.pem
```

### Validation

To validate our new certificates, you can run

```sh
openssl verify -CAfile ca-cert.pem \
   ca-cert.pem \
   server-cert.pem
```

to verify the server certificate or

```sh
openssl verify -CAfile ca-cert.pem \
   ca-cert.pem \
   client-cert.pem
```

to verify the client certificate.

If the client and server are configured with the given config files in the `docker/config` folder, a session can be verified to use TLS using the following SQL command:

```sql
SHOW SESSION STATUS LIKE 'Ssl_cipher';
```
