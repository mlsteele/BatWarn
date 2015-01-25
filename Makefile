.PHONY: all run pkill

all:
	rustc batwarn.rs

run:
	./batwarn

pkill:
	pkill batwarn
