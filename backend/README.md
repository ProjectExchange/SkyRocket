# SkyRocket backend

To run the project you need a `.env` file with the following content in the `src` folder

```toml
DATABASE_URL=mysql://<user>:<password>@<host>/<database>
ROCKET_DATABASES={skyrocket={url="mysql://<user>:<password>@<host>/<database>"}}
OAUTH_GITHUB_CLIENT_ID=<YOUR_CLIENT_ID>
OAUTH_GITHUB_CLIENT_SECRET=<YOUR_CLIENT_SECRET>
```

### GitHub OAuth Credentials
To generate the GitHub client credentials, go to your `GitHub Account > Settings > Developer
Settings > OAuth Apps` and create a new application.
