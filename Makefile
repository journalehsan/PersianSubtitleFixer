APP=persian-subtitle-fixer
CARGO?=cargo

.PHONY: build run fmt lint check clean

build:
	$(CARGO) build --release

run:
	$(CARGO) run -- $(ARGS)

fmt:
	$(CARGO) fmt

lint:
	$(CARGO) clippy --all-targets -- -D warnings

check:
	$(CARGO) check

clean:
	$(CARGO) clean
