.PHONY: help build run test clean check fmt lint docker-build docker-run

help: ## Mostra questo help
	@echo "Clean Architecture Rust - Comandi disponibili:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Compila il progetto
	cargo build

build-release: ## Compila il progetto in modalità release
	cargo build --release

run: ## Esegue l'applicazione
	cargo run

run-release: ## Esegue l'applicazione in modalità release
	cargo run --release

test: ## Esegue tutti i test
	cargo test

test-unit: ## Esegue solo gli unit test
	cargo test --lib

test-integration: ## Esegue solo i test di integrazione
	cargo test --test integration_test

test-verbose: ## Esegue i test con output dettagliato
	cargo test -- --nocapture

test-coverage: ## Genera report di coverage (richiede tarpaulin)
	cargo tarpaulin --out Html --output-dir coverage

check: ## Verifica il codice senza compilare
	cargo check

fmt: ## Formatta il codice
	cargo fmt

fmt-check: ## Verifica la formattazione del codice
	cargo fmt -- --check

lint: ## Esegue clippy per linting
	cargo clippy -- -D warnings

lint-fix: ## Corregge automaticamente i problemi di linting
	cargo clippy --fix

clean: ## Pulisce i file compilati
	cargo clean

doc: ## Genera la documentazione
	cargo doc --no-deps --open

watch: ## Ricompila automaticamente al cambio dei file (richiede cargo-watch)
	cargo watch -x run

watch-test: ## Esegue i test automaticamente al cambio dei file (richiede cargo-watch)
	cargo watch -x test

# API Testing
api-test: ## Testa l'API (richiede che il server sia in esecuzione)
	@echo "Testing API endpoints..."
	@bash examples/api_usage.sh

# Docker
docker-build: ## Crea l'immagine Docker
	docker build -t clean-architecture-rust .

docker-run: ## Esegue il container Docker
	docker run -p 3000:3000 clean-architecture-rust

# Development
dev: ## Modalità sviluppo con hot reload
	cargo watch -x 'run'

dev-check: fmt lint check test ## Esegue tutti i controlli di qualità del codice

# Install tools
install-tools: ## Installa gli strumenti di sviluppo necessari
	cargo install cargo-watch
	cargo install cargo-tarpaulin
	rustup component add clippy
	rustup component add rustfmt

# Audit
audit: ## Verifica le vulnerabilità nelle dipendenze
	cargo audit

update: ## Aggiorna le dipendenze
	cargo update

# Performance
bench: ## Esegue i benchmark (se presenti)
	cargo bench

# All-in-one
all: clean fmt lint check test build ## Esegue tutti i comandi in sequenza
