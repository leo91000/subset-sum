.PHONY: build

build:
	rm -rf pkg
	wasm-pack build --target nodejs --release
	sed -i 's/Int32Array/number[]/g' pkg/wasm_subset_sum.d.ts
	sed -i 's/@param {Int32Array/@param {number[]/g' pkg/wasm_subset_sum.js
	sed -i 's/@returns {Int32Array/@returns {number[]/g' pkg/wasm_subset_sum.js
	sed -i 's/return v1/return v1 ? Array.from(v1) : undefined/g' pkg/wasm_subset_sum.js
	wasm-opt -O3 -o pkg/wasm_subset_sum_bg.wasm pkg/wasm_subset_sum_bg.wasm

publish:
	wasm-pack publish

