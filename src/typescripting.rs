use std::cmp::*;
use wasm_bindgen::prelude::*;

/// A student's test, including its grade
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct GradedTest {
    student: String,
    test_grade: u8,
}

#[wasm_bindgen]
impl GradedTest {
    /// Creates a new graded test
    #[wasm_bindgen(constructor)]
    pub fn new(student: String, test_grade: u8) -> GradedTest {
        GradedTest {
            student,
            test_grade,
        }
    }

    /// The name of the student
    #[wasm_bindgen(getter)]
    pub fn student(&self) -> String {
        self.student.clone()
    }

    /// The test grade
    #[wasm_bindgen(getter)]
    pub fn grade(&self) -> u8 {
        self.test_grade
    }
}

/// A collection of test scores
#[wasm_bindgen]
pub struct TestScores {
    data: Vec<GradedTest>,
}

#[wasm_bindgen]
impl TestScores {
    /// Creates a new TestScores collection with the specified capacity
    #[wasm_bindgen(constructor)]
    pub fn with_capacity(amount: usize) -> TestScores {
        TestScores {
            data: Vec::with_capacity(amount),
        }
    }

    /// Add a new graded test
    #[wasm_bindgen(js_name = addGrade)]
    pub fn add_grade(&mut self, test: GradedTest) {
        self.data.push(test);
    }

    /// Gets the average test score
    #[wasm_bindgen(getter = average)]
    pub fn get_average(&self) -> u8 {
        let mut count = 0;
        let mut sum: i32 = 0;
        for test in &self.data {
            sum += test.grade() as i32;
            count += 1;
        }
        return (sum / count) as u8;
    }

    /// Gets the highest scored test
    #[wasm_bindgen(js_name = getHighScore)]
    pub fn get_high_score(&self) -> GradedTest {
        self.data
            .iter()
            .max_by(|x, y| x.grade().cmp(&y.grade()))
            .unwrap()
            .clone()
    }
}
