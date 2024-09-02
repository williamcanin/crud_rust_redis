# File type: Makefile
# Author: William C. Canin <https://williamcanin.github.io>

# Install Make in Windows:
# >>> choco install make

.DEFAULT_GOAL := commands

commands:
	@echo Commands:
	@echo     make fmt ---------------- Format code
	@echo     make tests -------------- Test the entire project
	@echo     make test-connection ---- Test connection
	@echo     make test-create-user --- Test the creation of a user
	@echo     make test-read-user ----- Test reading an existing user
	@echo     make test-update-user --- Testing an existing user update
	@echo     make test-delete-user --- Testing the removal of an existing user
	@echo     make build -------------- Compile code
	@echo     make release ------------ Compile code for release
	@echo     make run ---------------- Run program
	@echo     make clean -------------- Delete entire build directory
	@echo ----------------------------------------------------------
	@echo     (c) 2024 - William C. Canin - Makefile commands

fmt:
	@cargo +nightly fmt

tests:
	@cargo test --color=always tests::database::connection -- --show-output
	@cargo test --color=always tests::user::create -- --show-output
	@cargo test --color=always tests::user::read -- --show-output
	@cargo test --color=always tests::user::update -- --show-output
	@cargo test --color=always tests::user::delete -- --show-output

test-connection:
	@cargo test --color=always tests::database::connection -- --show-output

test-create-user:
	@cargo test --color=always tests::user::create -- --show-output

test-read-user:
	@cargo test --color=always tests::user::read -- --show-output

test-update-user:
	@cargo test --color=always tests::user::update -- --show-output

test-delete-user:
	@cargo test --color=always tests::user::delete -- --show-output

build:
	@cargo build

release:
	@cargo build --release

run:
	@cargo run

clean:
	@cargo clean
