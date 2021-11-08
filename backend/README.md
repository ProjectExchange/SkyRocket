# SkyRocket backend

To run the project you need a `.env` file with the following content in the `src` folder

```toml
DATABASE_URL=mysql://<user>:<password>@<host>/<database>
ROCKET_DATABASES={skyrocket={url="mysql://<user>:<password>@<host>/<database>"}}
```
