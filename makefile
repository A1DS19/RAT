all: compile

# Target to compile the project in release mode
compile:
	cargo build --release

# Target to run the C2 binary
run-c2:
	cargo run --release --bin c2

# Target to run the Zombie binary
run-zombie:
	cargo run --release --bin zombie

# Target to clean the project
clean:
	cargo clean
