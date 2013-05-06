all: compile test

clean:
	rm -rf build

compile: init
	rustc --lib -o build/string-match string-match.rc

build-test: init
	rustc --test -o build/string-match-test test/string-match.rs

init:
	mkdir -p build

test : build-test
	./build/string-match-test
