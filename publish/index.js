const {get_subset_sum} = require("./wasm_subset_sum");
module.exports.getSubsetSum = function ({ list, sum, timeout }) {
    const result = get_subset_sum(list, sum, timeout);
    if (result) {
        return Array.from(result);
    }
    return result;
}