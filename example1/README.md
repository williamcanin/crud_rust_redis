
# :crab: CRUD example1 between [Rust](https://www.rust-lang.org/) and [Redis](https://redis.io)

## Usage

1 - Create a `.env` file in this directory with the following variables:

```
PRODUCTION=false
TLS=true
DEVELOPMENT_URL=redis://127.0.0.1:6379
REDIS_USERNAME=
REDIS_HOSTNAME=
REDIS_PASSWORD=
REDIS_PORT=
```

In `PRODUCTION` you must set between `true` and `false`. If it is `true`, the connection to Redis will be made on your remote server, if it is `false`, the connection will be with the local Redis (localhost). In other words, if `PRODUCTION=true` it is for production, if `PRODUCTION=false` it is for development.

If `PRODUCTION` is `false` in the `.env` file, `TLS` will be automatically disabled, that is, even if `TLS=true` in the `.env` file, it will have no effect because there is no need for TLS in a local connection (development).

In `DEVELOPMENT_URL`, which will be the development url, you can leave the same specified, unless you have changed the default Redis port.

In `REDIS_USERNAME`, `REDIS_HOSTNAME`, `REDIS_PASSWORD`, and `REDIS_PORT`, these are Redis settings for your server, i.e. settings for `PRODUCTION=true` mode.

### Commands

2 - Run the tests:

```
make tests
````

3 - For more command, run:

```
make
````


