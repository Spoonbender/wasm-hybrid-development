import * as wasm from '../wasm/demo_app.js';

export function run() {
    const greeting = wasm.greet("CodeValue");
    console.log(greeting);
}