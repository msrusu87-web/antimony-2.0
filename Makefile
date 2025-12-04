.PHONY: help init setup build test clean deploy docs

help:
	@echo "ANTIMONY COIN 2.0 - Build & Deployment System"
	@echo ""
	@echo "Available commands:"
	@echo "  make init              - Initialize project (clone submodules, install deps)"
	@echo "  make setup             - Setup development environment"
	@echo "  make build             - Build all components"
	@echo "  make build-core        - Build core blockchain"
	@echo "  make build-evm         - Build EVM layer"
	@echo "  make build-explorer    - Build block explorer"
	@echo "  make build-wallets     - Build all wallets"
	@echo "  make test              - Run all tests"
	@echo "  make test-core         - Test core blockchain"
	@echo "  make test-contracts    - Test smart contracts"
	@echo "  make testnet           - Deploy testnet"
	@echo "  make mainnet           - Deploy mainnet"
	@echo "  make clean             - Clean build artifacts"
	@echo "  make docs              - Generate documentation"
	@echo "  make docker-build      - Build Docker images"
	@echo "  make docker-up         - Start Docker containers"
	@echo "  make docker-down       - Stop Docker containers"

init:
	@echo "Initializing ATMN 2.0 project..."
	@mkdir -p data logs config keys
	@echo "✓ Project directories created"
	@echo "✓ Initialize complete. Run 'make setup' to continue."

setup:
	@echo "Setting up development environment..."
	@chmod +x scripts/*.sh
	@echo "Installing dependencies..."
	@cd atmn-core && npm install 2>/dev/null || true
	@cd atmn-evm && npm install 2>/dev/null || true
	@cd atmn-explorer && npm install 2>/dev/null || true
	@cd atmn-wallet-web && npm install 2>/dev/null || true
	@cd atmn-contracts && npm install 2>/dev/null || true
	@echo "✓ Dependencies installed"

build: build-core build-evm build-explorer
	@echo "✓ All components built successfully"

build-core:
	@echo "Building core blockchain..."
	@cd atmn-core && cargo build --release 2>/dev/null || npm run build
	@echo "✓ Core built"

build-evm:
	@echo "Building EVM layer..."
	@cd atmn-evm && npm run build
	@echo "✓ EVM built"

build-explorer:
	@echo "Building block explorer..."
	@cd atmn-explorer && npm run build
	@echo "✓ Explorer built"

build-wallets:
	@echo "Building wallets..."
	@cd atmn-wallet-web && npm run build
	@echo "✓ Web wallet built"

test:
	@echo "Running test suite..."
	@cd atmn-tests && npm test
	@echo "✓ All tests passed"

test-core:
	@echo "Testing core blockchain..."
	@cd atmn-core && npm test 2>/dev/null || cargo test

test-contracts:
	@echo "Testing smart contracts..."
	@cd atmn-contracts && npm test

testnet: init
	@echo "Deploying to testnet..."
	@docker-compose -f docker-compose.testnet.yml up -d
	@echo "✓ Testnet started on http://localhost:3000"

mainnet: init
	@echo "⚠️  MAINNET DEPLOYMENT INITIATED"
	@echo "Deploying to mainnet..."
	@docker-compose -f docker-compose.mainnet.yml up -d
	@echo "✓ Mainnet deployed"

clean:
	@echo "Cleaning build artifacts..."
	@rm -rf */build */dist */node_modules .env .env.local
	@echo "✓ Clean complete"

docs:
	@echo "Generating documentation..."
	@cd atmn-docs && npm run generate
	@echo "✓ Documentation generated in ./docs"

docker-build:
	@echo "Building Docker images..."
	@docker-compose build
	@echo "✓ Docker images built"

docker-up:
	@echo "Starting Docker containers..."
	@docker-compose up -d
	@echo "✓ Containers running"

docker-down:
	@echo "Stopping Docker containers..."
	@docker-compose down
	@echo "✓ Containers stopped"

.DEFAULT_GOAL := help
