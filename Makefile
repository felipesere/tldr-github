# Self documentation of targets.
# From: https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.PHONY: help
help: ## Displays a list of make targets.
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: migrate-test-db
migrate-test-db: ## Runs all migrations against a test db
	rm -rf test.db*
	for filename in migrations/*/up.sql; do  sqlite3 test.db < $$filename; done


.PHONY: test
test: ## Runs all tests.
	cargo test

.PHONY: test-watch
test-watch: ## Runs tests and watches for file changes.
	cargo watch --clear -i tldr-github-svelte -x check -x test -s 'touch .trigger'

.PHONY: run
run: ## Run the application.
	RUST_LOG=info cargo run

.PHONY: run-watch
run-watch: ## Run the application and reload/recompile on file changes.
	RUST_LOG=info cargo watch --no-gitignore --clear -i tldr-github-svelte -w .trigger -x run

.PHONY: migrate
migrate: ## Apply database migrations.
	diesel migration run --database-url repos.db

.PHONY: setup
setup: ## Install dependencies and build the application
	cargo build
	cd tldr-github-svelte && npm install
	cd tldr-github-parcel && npm install

.PHONY: build-svelte
build-svelte: ## Build the svelte side of the UI
	cd tldr-github-svelte && npm run build

.PHONY: build-parcel
build-parcel: ## Build the parcel side of the UI
	cd tldr-github-parcel && npm run release

build-parcel-watch: ## Build the parcel side of the UI and keep watching for changes
	cd tldr-github-parcel &&  watchexec --clear --exts css,js,ts,tsx,json --ignore dist npm run release

.PHONY: build-ui
build-ui: build-svelte build-parcel ## Build all UI components

.PHONY: build-backend
build-backend: ## Build the Rust backend side
	cargo build

.PHONY: build-all
build-all: build-ui build-backend ## Build the entire app
