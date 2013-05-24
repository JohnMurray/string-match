RUSTC ?= rustc

#-------------------
# Internal Variables
dummy1 := $(shell mkdir bin 2> /dev/null)

all: string_match

string_match: lib

check: bin/test-string_match
	./bin/test-string_match

# Run unit tests with optimizations enabled (which is how we build the lib)
check-release: bin/test-string_match-release
	./bin/test-string_match-release

install:
	install `find bin -maxdepth 1 -name "libstring_match*" -type f` /usr/local/lib/rust/

clean:
	rm -rf bin

dist: lib
	tar --create --compress \
		--exclude \*/.git \
		--exclude \*/.git/\* \
		--file=string_match-0.0.1.tar.gz \
		README.md src Makefile manifest.json

#-------------------
# Binary Targts
# We always build the lib because:
# 1. We don't do it that often
# 2. It's fast.
# 3. The compiler gives it some crazy name like "libstring_match-da45653350eb4f90-0.1.dylib"
#    which is depenedent on some hash(?) as well as the current platform. (And -o works when
#    setting an executable's name, but not libraries)
.PHONY : lib
lib:
	$(RUSTC) --lib --out-dir bin -O src/string_match.rc

bin/test-string_match: src/string_match.rc src/*.rs src/test/string_match_test.rs
	$(RUSTC) --test -o $@ $<

bin/test-string_match-release: src/string_match.rc src/*.rs src/test/string_match_test.rs
	$(RUSTC) --test -O -o $@ $<



