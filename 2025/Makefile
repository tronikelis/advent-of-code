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

build/3: 3.rs
	rustc 3.rs -o build/3

.PHONY: 3
3: build/3
	build/3 3.txt

build/4: 4.rs
	rustc 4.rs -o build/4

.PHONY: 4
4: build/4
	build/4 4.txt

build/5: 5.rs
	rustc 5.rs -o build/5

.PHONY: 5
5: build/5
	build/5 5.txt

build/6: 6.rs
	rustc 6.rs -o build/6

.PHONY: 6
6: build/6
	build/6 6.txt

.PHONY: clean
clean:
	rm -r build/*
