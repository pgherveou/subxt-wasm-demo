PUBLIC_URL ?= /

.PHONY: build serve dev clean

build:
	wasm-pack build --target web --release
	npm run build
	cp index.html polkadot.json asset_hub.json dist/

serve: build
	npx serve dist -l 8081

clean:
	rm -rf dist pkg target
