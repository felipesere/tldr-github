.PHONY: test

test:
	cargo test

test-watch:
	cargo watch --clear -x check -x test -s 'touch .trigger'

run:
	cargo run

run-watch:
	cargo watch --no-gitignore --clear -w .trigger -x run
