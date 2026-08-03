fmt:
	cargo fmt

check: fmt
	cargo check

run: fmt
	cargo run

watch: fmt
	watchexec -c -w src -r cargo run
