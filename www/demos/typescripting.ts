import { GradedTest, TestScores } from "../wasm/demo_app";

export function run(): void {
    let scores: TestScores = new TestScores(4);

    scores.addGrade(new GradedTest("Alice", 49));
    scores.addGrade(new GradedTest("Bob", 56));
    scores.addGrade(new GradedTest("Carol", 90));
    scores.addGrade(new GradedTest("Dave", 88));

    console.log(`Average score: ${scores.average}`);
    const highestScorer: GradedTest = scores.getHighScore();
    console.log(`Highest scorer: ${highestScorer.student} scored ${highestScorer.grade}`);
}