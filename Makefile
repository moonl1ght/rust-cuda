CARGO = cargo

build-dev:
	$(CARGO) build

build:
	$(CARGO) build --release

run:
	$(CARGO) run