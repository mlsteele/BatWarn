.PHONY: all run pkill

all:
	rustc batwarn.rs

run: all
	./batwarn

pkill:
	pkill batwarn
