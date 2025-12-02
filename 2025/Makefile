.PHONY: init
init:
	mkdir -p build

build/1: 1.rs
	rustc 1.rs -o build/1

.PHONY: 1
1: build/1
	build/1 1.txt

build/2: 2.rs
	rustc 2.rs -o build/2

.PHONY: 2
2: build/2
	build/2 2.txt

.PHONY: clean
clean:
	rm -r build/*
