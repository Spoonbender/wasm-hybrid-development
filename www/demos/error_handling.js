import * as wasm from '../wasm/demo_app.js';

export function run() {
    wasm.meow();
}

export function openSchrodingersBox() {
    if (Math.random() > 0.5) {
        return "Meow!";
    } else {
        throw new Error("Schrödinger's cat is dead!");
    }
}