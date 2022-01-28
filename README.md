# ⚡ Subset Sum ⚡

Subset sum algorithm, but very fast 🦀 (written in rust).

## Installation

You have two options for subset-sum :
- Node bindings
- WASM

If you want to use this library in the browser, you must pick WASM.
Otherwise, I recommend the node bindings one, because it is slightly faster.

### Node bindings

```bash
npm i @leo91000/subset-sum
# or
yarn add @leo91000/subset-sum
# or
pnpm add @leo91000/subset-sum
```

### WASM

```bash
npm i @leo91000/wasm-subset-sum
# or
yarn add @leo91000/wasm-subset-sum
# or
pnpm add @leo91000/wasm-subset-sum
```

## Usage

```ts
import { getSubsetSum } from '@leo91000/subset-sum' // or @leo91000/wasm-subset-sum

const results = getSubsetSum(
    [5, -7, 3, 11], // List to get the subset sum from
    8, // Target sum
    1000, // Timeout in ms
)
```
