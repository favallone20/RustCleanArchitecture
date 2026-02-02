.PHONY: help run build clean fmt check test

help: ## Mostra questo messaggio di aiuto
	@echo "Comandi disponibili:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

run: ## Esegue il server
	cd backend && cargo run

build: ## Compila il progetto in release
	cargo build --release

clean: ## Pulisce i file di build e i dati
	cargo clean
	rm -rf backend/data/

fmt: ## Formatta il codice
	cargo fmt --all

check: ## Controlla il progetto senza compilare
	cargo check --all

test: ## Esegue i test
	cargo test --all

doc: ## Genera la documentazione
	cargo doc --no-deps --open
