run:
	cargo run --release

memory:
	cargo build --release --features alloc
	valgrind target/release/smart_pointers memory

performance:
	cargo run --release --features alloc -- performance

test:
	cargo test --release

test_alloc:
	cargo test --release --features alloc
