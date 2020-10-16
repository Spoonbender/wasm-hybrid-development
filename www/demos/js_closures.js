import { doForPrimes } from "../wasm/demo_app.js";

export function run() {
    let primes = new Array();
    doForPrimes(100, (prime) => {
        console.log(`Found prime: ${prime}`);
        primes.push(prime);
    });
    console.warn(`Here are the first 100 primes:`);
    console.warn(primes);
}