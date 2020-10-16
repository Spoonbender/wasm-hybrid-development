import { treat_like_a_duck } from "../wasm/demo_app.js";

const domestic = {
    quack: () => "Quack!"
};

const donald = {
    quack: () => "Oh boy oh boy oh boy!"
};

const daffy = {
    quack: () => "Youuu're deththpicable!"
};

export function run() {
    treat_like_a_duck(domestic);
    treat_like_a_duck(donald);
    treat_like_a_duck(daffy);
}