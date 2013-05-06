all: compile test

compile:
	rustc --lib -o build/string-match string-match.rc

build-test:
	rustc --test -o build/string-match-test test/string-match.rs

test : build-test
	./build/string-match-test
