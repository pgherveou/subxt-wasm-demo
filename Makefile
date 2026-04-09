PUBLIC_URL ?= /

.PHONY: build serve dev clean

build:
	trunk build subxt.html --release --public-url $(PUBLIC_URL)
	mv dist/index.html dist/subxt.html
	npm run build
	cp index.html papi.html dist/

serve: build
	npx serve dist -l 8081

dev:
	trunk serve subxt.html --open

clean:
	rm -rf dist
