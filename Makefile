run:
	cargo run --release

memory:
	cargo build --release
	valgrind target/release/smart_pointers memory

performance:
	cargo run --release -- performance

test:
	cargo test --release
