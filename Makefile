default: all

all: deps build

deps:
	(cd lib/rustful && make deps && make)
	find lib -name \*.so -exec rm {} \; # Force static link.

build:
	rustc -L lib/rustful/lib/ -o lang-detect src/lang_detect.rs

clean:
	rm -rf lang-detect lib/rustful

.PHONY: all deps build clean

