
# :crab: CRUD example1 between [Rust](https://www.rust-lang.org/) and [Redis](https://redis.io)

## Usage

1 - Create a `.env` file in this directory with the following variables:

```
PRODUCTION=false
DEVELOPMENT_URL=redis://127.0.0.1:6379
PRODUCTION_URL=rediss://<USERNAME>:<PASSWORD>@<HOST>:<PORT>
```

In `PRODUCTION` you must set between `true` and `false`. If it is `true`, the connection to Redis will be made on your remote server, if it is `false`, the connection will be to the local Redis (localhost). In other words, if `PRODUCTION=true` it is for production, if `PRODUCTION=false` it is for development. If `PRODUCTION=true` it will use the connection url from `PRODUCTION_URL`, otherwise it will use `DEVELOPMENT_URL`.

Remember, if the connection is TLS, the schema must be `rediss://`, without TLS, it will be `redis://`.

### Commands

2 - Run the tests:

```
make tests
````

3 - For more command, run:

```
make
````


