.PHONY:build run rd rr save trash resize sampler_test
release:
	cargo build -r
debug:
	cargo build
rd:
	./target/debug/main 4 RUST_BACKTRACE=1
	convert ./output/out.ppm ./output/out.jpg
rr:
	./target/release/main 4
	convert ./output/out.ppm ./output/out.jpg
save:
	cp ./output/out.jpg ./output/$(FILENAME).jpg
trash:
	cp ./output/out.jpg ./trash/$(FILENAME).jpg
resize:
	python3 resize.py ./output/out.jpg ./output/out.jpg
sampler_test:
	./target/release/main 1
	convert ./output/out.ppm ./output/sampling/white_16spp.jpg
	./target/release/main 2
	convert ./output/out.ppm ./output/sampling/uniform_16spp.jpg
	./target/release/main 3
	convert ./output/out.ppm ./output/sampling/jigger_16spp.jpg
	./target/release/main 4
	convert ./output/out.ppm ./output/sampling/blue_16spp.jpg

