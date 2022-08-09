JS_PATH := dist/asma/main.js
ORIGINAL_WASM_FILENAME := asma_bg.wasm
ORIGINAL_WASM_PATH := dist/asma/$(ORIGINAL_WASM_FILENAME)
NEW_WASM_FILENAME := main.wasm

build::
	@mkdir -p dist/asma
	@wasm-pack build --target web asma

# Copy distribution files
	@cp asma/pkg/asma.js $(JS_PATH)
	@cp asma/pkg/asma_bg.wasm dist/asma/asma_bg.wasm

# Display build information
# @echo "JS size:" $(shell ls -lh $(JS_PATH) |awk '{print $$5}')
	@echo "VM size:" $(shell ls -lh dist/asma/$(ORIGINAL_WASM_FILENAME) |awk '{print $$5}')

# Replace URL
	@sed -i '' 's/$(ORIGINAL_WASM_FILENAME)/$(NEW_WASM_FILENAME)/g' $(JS_PATH)

# npm install -g esbuild
	@esbuild --minify --sourcemap $(JS_PATH) --allow-overwrite --outfile=$(JS_PATH)

# If we want to optimize the WASM binary
# @wasm-opt -Oz -o dist/asma/asma_bg_optimized.wasm dist/asma/asma_bg.wasm
