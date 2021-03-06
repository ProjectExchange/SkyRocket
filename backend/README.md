# SkyRocket backend

To run the project you need a `.env` file with the following content in the `backend` folder

```toml
REDIS_URL=redis://127.0.0.1/
DATABASE_URL=mysql://<user>:<password>@<host>/<database>
ROCKET_SECRET_KEY=<GENERATE_YOUR_OWN>
ROCKET_DATABASES={skyrocket={url="mysql://<user>:<password>@<host>/<database>"}}
OAUTH_GITHUB_CLIENT_ID=<YOUR_CLIENT_ID>
OAUTH_GITHUB_CLIENT_SECRET=<YOUR_CLIENT_SECRET>
```

`ROCKET_SECRET_KEY` is either a base64 encoded string which has a raw length of 44 or 88 characters,
or a length of 64 if hex-encoded. On linux, simply executed `openssl rand -base64 32`

### Useful commands

Check for syntax error:

```sh
cargo fmt -- --check
```

Use clippy to check for issues:

```sh
cargo clippy -- -D warnings
```

Run unit tests

```sh
cargo test
```

### GitHub OAuth Credentials
To generate the GitHub client credentials, go to your `GitHub Account > Settings > Developer
Settings > OAuth Apps` and create a new application.
