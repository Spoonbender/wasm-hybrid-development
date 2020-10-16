import * as wasm from '../wasm/demo_app.js';

export function run() {
    const someObj = {
        "key1": "value1",
        "key2": 1234,
        "key3": true,
        "key4": {
            "nested1": "hello",
            "nested2": "world"
        }
    };
    const result = wasm.print_my_keys(someObj);
    console.log(result);
}