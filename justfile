set dotenv-load := false
just_home := justfile_directory()

# for Windows
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[doc("📋 Show all recipes")]
[group("Help")]
help:
    @just --list

[group("Help")]
default: help

[group("Dev")]
fmt:
	clear && cargo fmt

[group("Dev")]
check: fmt
	cargo check

[group("Dev")]
clippy: fmt
	cargo clippy --no-deps

[group("Dev")]
fix: check
	cargo clippy --no-deps

[group("Dev")]
fix-all: fmt
	cargo fix --allow-dirty --allow-no-vcs

run: fmt
	RUST_BACKTRACE=full cargo run

[group("Dev")]
[unix]
run-u: fmt
	RUST_BACKTRACE=full cargo run

[group("Dev")]
[windows]
run-w: fmt
	$env:RUST_BACKTRACE = "full"; cargo run

[group("Build")]
build: fmt
	cargo build

[group("Build")]
release: fmt
	cargo build --release

[group("Watch")]
watch: fmt
	# watchexec -c -w assets -w src -r cargo run
	watchexec -c -w src -r cargo run

[group("Watch")]
c: fmt
	watchexec -c -w src -r cargo check

[group("Maintenance")]
clean:
	cargo clean && rm -rf ./target && rm ./Cargo.lock

[group("Maintenance")]
[doc('Git check repo')]
gitc:
	git fsck
	git gc --prune=now --aggressive
	git count-objects -vH
