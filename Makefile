all: example

example:
	cd rusqdrive
	cargo run --example usercrud

cargo-deny:
	cargo deny check -s
