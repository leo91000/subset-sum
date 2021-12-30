.PHONY: build

build:
	rm -rf dist
	wasm-pack build --target bundler --release  --out-dir dist/import
	sed -i 's/Int32Array/number[]/g' dist/import/wasm_subset_sum.d.ts
	sed -i 's/@param {Int32Array/@param {number[]/g' dist/import/wasm_subset_sum.js
	sed -i 's/@returns {Int32Array/@returns {number[]/g' dist/import/wasm_subset_sum.js
	sed -i 's/return v1/return v1 ? Array.from(v1) : undefined/g' dist/import/wasm_subset_sum.js
	wasm-opt -O3 -o dist/import/wasm_subset_sum_bg.wasm dist/import/wasm_subset_sum_bg.wasm
	rm -rf dist/import/package.json
	rm -rf dist/import/.gitignore
	mv dist/import/wasm_subset_sum_bg.js dist/import/wasm_subset_sum_bg.mjs
	mv dist/import/wasm_subset_sum.js dist/import/wasm_subset_sum.mjs
	sed -i 's/wasm_subset_sum_bg.js/wasm_subset_sum_bg.mjs/g' dist/import/wasm_subset_sum.mjs

	wasm-pack build --target nodejs --release  --out-dir dist/cjs
	sed -i 's/Int32Array/number[]/g' dist/cjs/wasm_subset_sum.d.ts
	sed -i 's/@param {Int32Array/@param {number[]/g' dist/cjs/wasm_subset_sum.js
	sed -i 's/@returns {Int32Array/@returns {number[]/g' dist/cjs/wasm_subset_sum.js
	sed -i 's/return v1/return v1 ? Array.from(v1) : undefined/g' dist/cjs/wasm_subset_sum.js
	wasm-opt -O3 -o dist/cjs/wasm_subset_sum_bg.wasm dist/cjs/wasm_subset_sum_bg.wasm
	rm -rf dist/cjs/package.json
	rm -rf dist/cjs/.gitignore
	mv dist/cjs/wasm_subset_sum.js dist/cjs/wasm_subset_sum.cjs

publish:
	wasm-pack publish

