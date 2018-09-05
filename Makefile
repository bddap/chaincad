
PHONY: watch

watch:
	ag -lr | entr -rc cargo run -- ./sample/snowman.clc

