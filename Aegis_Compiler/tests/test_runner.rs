//! Test runner utilities for the Aegis compiler test suite

use std::time::{Duration, Instant};

/// Simple test result structure
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration: Duration,
    pub error_message: Option<String>,
}

/// Test suite results
#[derive(Debug)]
pub struct TestSuiteResult {
    pub suite_name: String,
    pub tests: Vec<TestResult>,
    pub total_duration: Duration,
}

impl TestSuiteResult {
    pub fn new(suite_name: String) -> Self {
        Self {
            suite_name,
            tests: Vec::new(),
            total_duration: Duration::new(0, 0),
        }
    }

    pub fn add_test(&mut self, result: TestResult) {
        self.total_duration += result.duration;
        self.tests.push(result);
    }

    pub fn passed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.passed).count()
    }

    pub fn failed_count(&self) -> usize {
        self.tests.iter().filter(|t| !t.passed).count()
    }

    pub fn total_count(&self) -> usize {
        self.tests.len()
    }

    pub fn success_rate(&self) -> f64 {
        if self.tests.is_empty() {
            0.0
        } else {
            self.passed_count() as f64 / self.total_count() as f64
        }
    }
}

/// Overall test report
#[derive(Debug)]
pub struct TestReport {
    pub suites: Vec<TestSuiteResult>,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

impl TestReport {
    pub fn new() -> Self {
        Self {
            suites: Vec::new(),
            start_time: Instant::now(),
            end_time: None,
        }
    }

    pub fn add_suite(&mut self, suite: TestSuiteResult) {
        self.suites.push(suite);
    }

    pub fn finish(&mut self) {
        self.end_time = Some(Instant::now());
    }

    pub fn total_duration(&self) -> Duration {
        if let Some(end_time) = self.end_time {
            end_time.duration_since(self.start_time)
        } else {
            self.start_time.elapsed()
        }
    }

    pub fn total_tests(&self) -> usize {
        self.suites.iter().map(|s| s.total_count()).sum()
    }

    pub fn total_passed(&self) -> usize {
        self.suites.iter().map(|s| s.passed_count()).sum()
    }

    pub fn total_failed(&self) -> usize {
        self.suites.iter().map(|s| s.failed_count()).sum()
    }

    pub fn overall_success_rate(&self) -> f64 {
        if self.total_tests() == 0 {
            0.0
        } else {
            self.total_passed() as f64 / self.total_tests() as f64
        }
    }

    /// Generate a simple summary
    pub fn generate_summary(&self) -> String {
        format!(
            "Tests: {} passed, {} failed, {} total ({:.1}% success rate) in {:.2}s",
            self.total_passed(),
            self.total_failed(),
            self.total_tests(),
            self.overall_success_rate() * 100.0,
            self.total_duration().as_secs_f64()
        )
    }

    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.total_failed() == 0
    }
}
