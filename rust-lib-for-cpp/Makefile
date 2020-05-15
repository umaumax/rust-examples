ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/libdouble_input_lib.$(EXT)
	g++ src/main.cpp -L ./target/debug/ -ldouble_input_lib -o double_input_main
	LD_LIBRARY_PATH=./target/debug/ ./double_input_main

target/debug/libdouble_input_lib.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf double_input_main
