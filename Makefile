all: compile test

clean:
	rm -rf build

compile: init
	rustc --lib -o build/string_match string_match.rc

build-test: init
	rustc --test -o build/string_match-test test/string_match.rs

init:
	mkdir -p build

test : build-test
	./build/string_match-test
