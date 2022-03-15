.PHONY:build run
release:
	cargo build -r
debug:
	cargo build
rd:
	./target/debug/main | convert - output/out.jpg
rr:
	./target/release/main | convert - output/out.jpg
