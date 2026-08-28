set dotenv-load := false
just_home := justfile_directory()

# for Windows
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

# globals vars
PROJECT_NAME := "macro-background"
#
CARGO_TERM_COLOR := "always"

[doc("📋 Show all recipes")]
[group("Help")]
help:
    @just --list

[group("Help")]
default: help

[group("Dev")]
fmt:
	cargo fmt -p {{PROJECT_NAME}}

[group("Dev")]
check: fmt
	cargo check -p {{PROJECT_NAME}}

[group("Dev")]
clippy: fmt
	clear && cargo clippy -p {{PROJECT_NAME}} --no-deps

[group("Dev")]
fix: fmt check
	clear && cargo clippy -p {{PROJECT_NAME}} --no-deps

[group("Dev")]
fix-all: fmt check
	clear && cargo fix -p {{PROJECT_NAME}} --allow-dirty

[group("Dev")]
run: fmt
	clear && cargo run -p {{PROJECT_NAME}}

[group("Dev")]
[unix]
rund:
	clear && ./target/debug/{{PROJECT_NAME}}

[group("Dev")]
[windows]
rund:
	clear && ./target/debug/{{PROJECT_NAME}}.exe

[group("Dev")]
[group("Build")]
build: fmt
	clear && cargo build -p {{PROJECT_NAME}}

[group("Build")]
release: fmt
	clear && cargo build -p {{PROJECT_NAME}} --release

[group("Watch")]
wr: fmt
	# watchexec -c -w assets -w src -r cargo run
	watchexec -c -w {{PROJECT_NAME}}/src -r cargo run -p {{PROJECT_NAME}}

[group("Watch")]
wc: fmt
	watchexec -c -w {{PROJECT_NAME}}/src -r cargo check -p {{PROJECT_NAME}}

[group("Maintenance")]
clean:
	cargo clean && rm -rf ./target && rm ./Cargo.lock

[group("Maintenance")]
clean-all:
	cargo clean -p {{PROJECT_NAME}} && rm -rf {{PROJECT_NAME}}/target && rm {{PROJECT_NAME}}/Cargo.lock

[group("Maintenance")]
[doc('Git check repo')]
gitc:
	git fsck
	git gc --prune=now --aggressive
	git count-objects -vH
