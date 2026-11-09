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
	cargo clippy --no-deps -- -D clippy::style -D clippy::correctness

[group("Dev")]
fix: fmt
    cargo clippy --no-deps

[group("Dev")]
fix-all: fmt
	cargo fix --allow-dirty --allow-no-vcs

[group("Dev")]
[unix]
run: fmt
	RUST_BACKTRACE=full cargo run

[group("Dev")]
[windows]
run: fmt
	$env:RUST_BACKTRACE = "full"; cargo run

[group("Build")]
build: fmt
	cargo build

[group("Build")]
release: fmt
	cargo build --release

[group("Watch")]
[unix]
watch:
	watchexec -c -w src -e rs -r -- "cargo fmt && cargo run"

[group("Watch")]
[windows]
watch:
	#watchexec -c -w src -e rs -r cargo run
	watchexec -c -w src -e rs -r -- "cargo fmt ; cargo run"

[group("Watch")]
c: fmt
	watchexec -c -w src -e rs -r cargo check

[group("Maintenance")]
clean-all:
	cargo clean && rm -rf ./target && rm ./Cargo.lock

[group("Maintenance")]
show-b:
	cat ./target/build_version.conf

[group("Maintenance")]
[doc('Git check repo')]
gitc:
	git fsck
	git gc --prune=now --aggressive
	git count-objects -vH

[windows]
db-rm:
	rm -v $env:LOCALAPPDATA\Thekes\data\thekes.db

[unix]
db-rm:
	rm -v ~/.local/share/thekes/thekes.db

bundle-v:
	cg-bundler --minify --keep-docs --no-expand-modules --validate

bundle-g:
	cg-bundler --keep-docs --no-expand-modules -o bundle.rs

bundle-m:
	cg-bundler --minify --keep-docs --no-expand-modules -o bundle-m.rs

bundle-rm:
	rm .\bundle.rs ; rm .\bundle-m.rs

find-a:
    rg -n --glob '*.rs' '#\s*\[\s*allow\s*\(' .

find-t:
    clear && rg -n --glob '*.rs' 'TODO|FIX|TO FIX|CHECK|TO CHECK|TEST|TO TEST|REMOVE|TO REMOVE|FIXME|HACK|todo!\(|unimplemented!\(|unreachable!\(' .

find-c:
    rg -n --glob '*.rs' '#\s*\[\s*allow\s*\(\s*clippy::' .
