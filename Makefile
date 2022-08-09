build::
	@mkdir -p dist/asma
	@wasm-pack build --target web asma
	@cp asma/pkg/asma.js dist/asma/main.js
	@cp asma/pkg/asma_bg.wasm dist/asma/asma_bg.wasm
