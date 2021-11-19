build:
	rm -rf pkg
	wasm-pack build --target nodejs --release
	wasm-opt -O3 -o pkg/subset_sum_bg.wasm pkg/subset_sum_bg.wasm