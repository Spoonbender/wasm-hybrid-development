import { run as runHelloWorld } from './demos/hello_world.js';
import { run as runCallingJavascript } from './demos/calling_javascript.js';
import { run as runTypescripting } from './demos/typescripting.js';
import { run as runUntypedValues } from './demos/untyped_values.js';
import { run as runDomManipulation } from './demos/dom_manipulation.js';
import { run as runErrorHandling } from './demos/error_handling.js';
import { run as runAsyncFunctions } from './demos/async_functions.js';
import { run as runDuckTyping } from './demos/duck_typing.js';
import { run as runJsClosures } from './demos/js_closures.js';
import { run as runRustClosures } from './demos/rust_closures.js';

let helloWorld = {
    title: "The one with the console.log at the end",
    description: "Simple call from JS to a WASM function, passing a string as input and getting another string as output, then printing it to console",
    run: runHelloWorld,
    sources: ["static/hello_world.js", "static/hello_world.rs"]
}

let callingJavascript = {
    title: "The one with the alert()",
    description: "WASM calls the alert() function of the browser",
    run: runCallingJavascript,
    sources: ["static/calling_javascript.js", "static/calling_javascript.rs"]
}

let typescripting = {
    title: "The one with the types",
    description: "Types defined in Rust are mapped to TypeScript types with auto-generated code",
    run: runTypescripting,
    sources: ["static/typescripting.ts", "static/typescripting.rs", "static/demo_app.d.ts"]
}

let untypedValues = {
    title: "The one without the types",
    description: "Iterate over the keys of a JavaScript object",
    run: runUntypedValues,
    sources: ["static/untyped_values.js", "static/untyped_values.rs"]
}

let domManipulation = {
    title: "The one with the DOM",
    description: "Rust manipulates the DOM by finding a specific element and adding a new element to it",
    run: runDomManipulation,
    sources: ["static/dom_manipulation.js", "static/dom_manipulation.rs"]
}

let errorHandling = {
    title: "The one where an exception is thrown",
    description: "Rust invokes a JS function and catches an exception",
    run: runErrorHandling,
    sources: ["static/error_handling.js", "static/error_handling.rs"]
}

let asyncFunctions = {
    title: "The one with the Future as Promise",
    description: "JS invokes a Rust async function, awaiting the Future as if it was a Promise",
    run: runAsyncFunctions,
    sources: ["static/async_functions.js", "static/async_functions.rs"]
}

let duckTyping = {
    title: "The one with the ducks",
    description: "Duck typing: binding anything that quacks to a Rust Quack trait",
    run: runDuckTyping,
    sources: ["static/duck_typing.js", "static/duck_typing.rs"]
}

let jsClosures = {
    title: "The one with JavaScript Closures",
    description: "JavaScript passes a closure to Rust, which invokes it without knowing much about it!",
    run: runJsClosures,
    sources: ["static/js_closures.js", "static/js_closures.rs"]
}

let rustClosures = {
    title: "The one with Rust Closures",
    description: "Rust passes a closure to JavaScript",
    run: runRustClosures,
    sources: ["static/rust_closures.js", "static/rust_closures.rs"]
}

export let demos = [
    helloWorld,
    callingJavascript,
    typescripting,
    untypedValues,
    duckTyping,
    errorHandling,
    jsClosures,
    rustClosures,
    asyncFunctions,
    domManipulation,
]