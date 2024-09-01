# :crab: CRUD examples between [Rust](https://www.rust-lang.org/) and [Redis](https://redis.io)

## Requirements

* Rust > 1.70

## Usage

1 - Create a `.env` file in the project root.

2 - Add the following variables:

```
REDIS_USERNAME=
REDIS_HOSTNAME=
REDIS_PASSWORD=
REDIS_PORT=
````

> Note: Fill in according to your bank details.

3 - Run the test to verify the connection:

```
cargo test tests::database::connection
````

---
[LICENSE](https://github.com/williamcanin/crud_rust_redis)