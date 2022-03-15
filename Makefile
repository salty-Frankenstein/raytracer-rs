.PHONY:build run
release:
	cargo build -r
debug:
	cargo build
rd:
	./target/debug/main
run:
	./target/release/main
