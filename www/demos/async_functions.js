import * as wasm from '../wasm/demo_app.js';

export async function run() {
    console.log("Calling an async WASM function...");
    const promise = wasm.getWebAssemblyLanguages("https://raw.githubusercontent.com/appcypher/awesome-wasm-langs/master/README.md");
    for (let index = 0; index < 5; index++) {
        console.log("Doing other things while the promise is running in the background... " + index);
    }

    console.log("Awaiting promise to resolve...");
    const parsedText = await promise;
    console.log("Promise resovled!");
    console.log(parsedText);
    return parsedText;
}