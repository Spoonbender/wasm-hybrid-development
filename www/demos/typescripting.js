"use strict";
exports.__esModule = true;
var demo_app_1 = require("../wasm/demo_app");
function run() {
    var scores = new demo_app_1.TestScores(4);
    scores.addGrade(new demo_app_1.GradedTest("Alice", 49));
    scores.addGrade(new demo_app_1.GradedTest("Bob", 56));
    scores.addGrade(new demo_app_1.GradedTest("Carol", 90));
    scores.addGrade(new demo_app_1.GradedTest("Dave", 88));
    console.log("Average score: " + scores.average);
    var highestScorer = scores.getHighScore();
    console.log("Highest scorer: " + highestScorer.student + " scored " + highestScorer.grade);
}
exports.run = run;
