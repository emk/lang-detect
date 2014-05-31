default: all

all: deps build

deps:
	(cd lib/rustful && make deps && make)

build:
	rustc -L lib/rustful/lib/ -o lang-detect src/lang_detect.rs

test:
	rustc -L lib/rustful/lib/ --test -o test src/test.rs
	./test

clean:
	rm -rf lang-detect lib/rustful

.PHONY: all deps build test clean

