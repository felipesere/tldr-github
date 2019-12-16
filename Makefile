.PHONY: test

test:
	cargo test

test-watch:
	cargo watch --clear -i tldr-github-svelte -x check -x test -s 'touch .trigger'

run:
	cargo run

run-watch:
	cargo watch --no-gitignore --clear -i tldr-github-svelte -w .trigger -x run
