fmt:
	cargo fmt

check: fmt
	cargo check

run: fmt
	cargo run

watch: fmt
	watchexec -c -w src -r cargo run

c: fmt
	watchexec -c -w src -r cargo check

gitc:
    git fsck && git gc --prune=now --aggressive && git count-objects -vH
