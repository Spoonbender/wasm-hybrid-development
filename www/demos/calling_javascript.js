import * as wasm from '../wasm/demo_app.js';

export function run() {
    wasm.greet_redux("CodeValue");

    // Override the alert function
    let originalAlert = alert;
    alert = (msg) => {
        msg = "This is an overidden function! " + msg;
        originalAlert(msg);
    }

    // This will call the overriding function
    wasm.greet_redux("Web Devs");

    // Restore original alert function
    alert = originalAlert;
}