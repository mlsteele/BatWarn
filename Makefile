.PHONY: all run pkill

all:
	cargo build

run: all pkill
	cargo run

pkill:
	-pkill batwarn
