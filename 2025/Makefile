.PHONY: init
init:
	mkdir -p build

build/1: 1.rs
	rustc 1.rs -o build/1

.PHONY: 1
1: build/1
	build/1 1.txt

.PHONY: clean
clean:
	rm -r build/*
