//! Test runner and reporting system for the Aegis compiler test suite

use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Test result for individual test cases
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

    /// Generate a detailed report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("=".repeat(80).as_str());
        report.push_str("\n");
        report.push_str("                    AEGIS COMPILER TEST SUITE REPORT\n");
        report.push_str("=".repeat(80).as_str());
        report.push_str("\n\n");
        
        // Overall summary
        report.push_str(&format!("Total Tests: {}\n", self.total_tests()));
        report.push_str(&format!("Passed: {} ({:.1}%)\n", 
            self.total_passed(), 
            self.overall_success_rate() * 100.0));
        report.push_str(&format!("Failed: {}\n", self.total_failed()));
        report.push_str(&format!("Total Duration: {:.2}s\n\n", 
            self.total_duration().as_secs_f64()));
        
        // Suite-by-suite breakdown
        report.push_str("TEST SUITE BREAKDOWN:\n");
        report.push_str("-".repeat(50).as_str());
        report.push_str("\n");
        
        for suite in &self.suites {
            report.push_str(&format!("\n{}: {} tests, {} passed, {} failed ({:.1}%) - {:.2}s\n",
                suite.suite_name,
                suite.total_count(),
                suite.passed_count(),
                suite.failed_count(),
                suite.success_rate() * 100.0,
                suite.total_duration.as_secs_f64()
            ));
            
            // Show failed tests
            let failed_tests: Vec<_> = suite.tests.iter()
                .filter(|t| !t.passed)
                .collect();
            
            if !failed_tests.is_empty() {
                report.push_str("  Failed tests:\n");
                for test in failed_tests {
                    report.push_str(&format!("    - {}: {}\n", 
                        test.name, 
                        test.error_message.as_ref().unwrap_or(&"Unknown error".to_string())
                    ));
                }
            }
        }
        
        // Performance analysis
        report.push_str("\nPERFORMANCE ANALYSIS:\n");
        report.push_str("-".repeat(50).as_str());
        report.push_str("\n");
        
        let mut suite_times: Vec<_> = self.suites.iter()
            .map(|s| (s.suite_name.clone(), s.total_duration))
            .collect();
        suite_times.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (name, duration) in suite_times {
            report.push_str(&format!("{}: {:.2}s\n", name, duration.as_secs_f64()));
        }
        
        // Recommendations
        report.push_str("\nRECOMMENDATIONS:\n");
        report.push_str("-".repeat(50).as_str());
        report.push_str("\n");
        
        if self.overall_success_rate() < 0.95 {
            report.push_str("⚠️  Test success rate is below 95%. Consider investigating failing tests.\n");
        }
        
        if self.total_duration().as_secs() > 10 {
            report.push_str("⚠️  Test suite takes longer than 10 seconds. Consider optimizing slow tests.\n");
        }
        
        let slowest_suite = self.suites.iter()
            .max_by_key(|s| s.total_duration);
        if let Some(suite) = slowest_suite {
            if suite.total_duration.as_secs() > 2 {
                report.push_str(&format!("⚠️  '{}' suite is particularly slow ({:.2}s). Consider optimization.\n",
                    suite.suite_name, suite.total_duration.as_secs_f64()));
            }
        }
        
        if self.overall_success_rate() >= 0.95 && self.total_duration().as_secs() <= 5 {
            report.push_str("✅ Test suite is performing well!\n");
        }
        
        report.push_str("\n");
        report.push_str("=".repeat(80).as_str());
        report.push_str("\n");
        
        report
    }

    /// Generate a summary for CI/CD systems
    pub fn generate_summary(&self) -> String {
        format!("Tests: {} passed, {} failed, {} total ({:.1}% success rate) in {:.2}s",
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

    /// Get failed test details for debugging
    pub fn get_failed_tests(&self) -> Vec<(String, String, String)> {
        let mut failed = Vec::new();
        
        for suite in &self.suites {
            for test in &suite.tests {
                if !test.passed {
                    failed.push((
                        suite.suite_name.clone(),
                        test.name.clone(),
                        test.error_message.clone().unwrap_or_else(|| "Unknown error".to_string())
                    ));
                }
            }
        }
        
        failed
    }
}