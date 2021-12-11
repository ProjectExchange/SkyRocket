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
