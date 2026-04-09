PUBLIC_URL ?= /

.PHONY: build serve dev clean

build:
	trunk build subxt.html --release --public-url $(PUBLIC_URL)
	mv dist/index.html dist/subxt.html
	cp index.html dist/index.html

serve: build
	@echo "Serving at http://127.0.0.1:8081/"
	cd dist && python3 -m http.server 8081

dev:
	trunk serve subxt.html --open

clean:
	rm -rf dist
