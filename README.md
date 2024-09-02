# :crab: CRUD examples between [Rust](https://www.rust-lang.org/) and [Redis](https://redis.io)

## Requirements

* Rust > 1.70

## Usage

1 - Clone the project:

```
git clone https://github.com/williamcanin/crud_rust_redis.git
```

2 - Access the project folder, for example the `example1` project:

```
cd crud_rust_redis/example1
```

3 - Create a `.env` file in the project root and the following variables:

```
REDIS_USERNAME=
REDIS_HOSTNAME=
REDIS_PASSWORD=
REDIS_PORT=
````

> Note: Fill in according to the data in your database.

4 - Run the test to verify the connection:

```
cargo test tests::database::connection
````

---
[LICENSE](https://github.com/williamcanin/crud_rust_redis/blob/main/LICENSE)