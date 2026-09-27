.PHONY: all dev release test bench clean install docker

all: release

dev:
	@bash scripts/dev.sh

release:
	@bash scripts/release.sh

test:
	@echo "==> Running Unit & Integration Tests..."
	cargo test --workspace -- --nocapture
	@echo "==> Running Chaos Fault Injection..."
	@bash scripts/chaos.sh

bench:
	@bash scripts/bench.sh

docker:
	docker compose -f docker/docker-compose.yml up --build

clean:
	cargo clean
	rm -rf target/

install: release
	@bash scripts/install.sh
