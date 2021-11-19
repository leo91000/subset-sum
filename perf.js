const { find_first_subset_sum } = require('./pkg/subset_sum')
const { performance } = require('perf_hooks')

function vanillaSubsetSum({
  array,
  target,
  timeout,
  now = Date.now,
}) {
    // if (now() > timeout) {
    //     throw new Error();
    // }

    if (array.includes(target)) {
        return [target];
    }

    if (array.reduce((a, c) => a + c, 0) === target) {
        return array;
    }

    for (let index = 0; index < array.length; index++) {
        const current = array[index];
        const withoutCurrent = [...array.slice(0, index), ...array.slice(index + 1)];
        const newTarget = target - current;
        const result = vanillaSubsetSum({ array: withoutCurrent, target: newTarget, timeout });

        if (result.length !== 0) {
            return [current, ...result];
        }
    }

    return [];
}

function wasmSubsetSum({ array, target }) {
    return find_first_subset_sum(array, target)
}

function perfTest({ array, target }) {
    let vanillaResult, wasmResult;

    const vanillaStart = performance.now()
    try { vanillaResult = vanillaSubsetSum({ array, target }) } catch (e) { vanillaResult = e }
    const vanillaEnd = performance.now()

    const wasmStart = performance.now()
    try { wasmResult = wasmSubsetSum({ array, target }) } catch (e) { wasmResult = e }
    const wasmEnd = performance.now()
    console.log(JSON.stringify({
        array,
        target,
        vanilla: {
            duration: vanillaEnd - vanillaStart,
            result: vanillaResult,
            sum: vanillaResult.reduce((a, c) => a + c, 0)
        },
        wasm: {
            duration: wasmEnd - wasmStart,
            result: typeof wasmResult === 'string' ? wasmResult : Array.from(wasmResult),
            sum: typeof wasmResult === 'string' ? wasmResult : wasmResult.reduce((a, c) => a + c, 0)
        }
    }))
    console.log('-------------------------------')
}

// Warmup
vanillaSubsetSum({ array: [1,2,3,4], target: 10 })
wasmSubsetSum({ array: [1,2,3,4], target: 10 })

// Perf test
perfTest({ array: [1,2,3,4], target: 10 })
perfTest({ array: [1,2,3,4], target: 7 })
perfTest({ array: [1,2,3,4,5,6,7,8,9,10,11], target: 60 })
perfTest({ array: [1,2,3,4,5,6,7,8,9], target: -1 })
perfTest({ array: [1,2,3,4,5,6,7,8,9,10], target: -1 })
perfTest({ array: [1,2,3,4,5,6,7,8,9,10,11], target: -1 })
perfTest({ array: [1,2,3,4,5,6,7,8,9,10,11,-1], target: 0 })