.PHONY:build run rd rr save trash resize
release:
	cargo build -r
debug:
	cargo build
rd:
	./target/debug/main RUST_BACKTRACE=1
	convert ./output/out.ppm ./output/out.jpg
rr:
	./target/release/main
	convert ./output/out.ppm ./output/out.jpg
save:
	cp ./output/out.jpg ./output/$(FILENAME).jpg
trash:
	cp ./output/out.jpg ./trash/$(FILENAME).jpg
resize:
	python3 resize.py ./output/out.jpg ./output/out.jpg