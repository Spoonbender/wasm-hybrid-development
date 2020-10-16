import * as wasm from '../wasm/demo_app.js';

export function run() {
    wasm.printFibonacci();
}

export function printSequence(getNextItem) {
    for (let index = 0; index < 20; index++) {
        const item = getNextItem();
        console.log(`Got: ${item}`);
    }
}