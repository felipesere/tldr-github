.PHONY: test

test:
	cargo test

test-watch:
	cargo watch --clear -i tldr-github-svelte -x check -x test -s 'touch .trigger'

run:
	cargo run

run-watch:
	RUST_LOG=info cargo watch --no-gitignore --clear -i tldr-github-svelte -w .trigger -x run

migrate:
	diesel migration run --database-url repos.db
