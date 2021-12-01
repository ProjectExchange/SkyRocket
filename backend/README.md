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
or a length of 64 if hex-encoded. On linux, simply executed `pwgen -y 88 | head -1 | base64 | tail -1`

### GitHub OAuth Credentials
To generate the GitHub client credentials, go to your `GitHub Account > Settings > Developer
Settings > OAuth Apps` and create a new application.
