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
	#cargo clippy --all-targets --all-features -- -D warnings
	cargo clippy --no-deps

[group("Dev")]
fix: check
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
watch:
	#watchexec -c -w assets -w src -r cargo run
	#watchexec -c -w src -e rs -r -- "./rust-reload.ps1"
	watchexec -c -w src -e rs -r -- "cargo fmt ; cargo run"
	# watchexec -c -w src -e rs -r -- "pwsh -NoProfile -ExecutionPolicy Bypass -File .\rust-reload.ps1"

[group("Watch")]
c: fmt
	watchexec -c -w src -r cargo check


[group("Maintenance")]
[windows]
clean-all:
	cargo clean ; rm .\target ; rm .\Cargo.lock

[group("Maintenance")]
[unix]
clean-all:
	cargo clean && rm -rf ./target && rm ./Cargo.lock

[windows]
show-b:
	more .\target\build_version.conf

[unix]
show-b:
	cat .\target\build_version.conf

[group("Maintenance")]
[doc('Git check repo')]
gitc:
	git fsck
	git gc --prune=now --aggressive
	git count-objects -vH

db-rm:
	rm $env:LOCALAPPDATA\Thekes\data\thekes.db

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
