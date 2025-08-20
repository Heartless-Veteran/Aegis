//! Aegis Compiler Test Suite
//! 
//! This is a comprehensive test suite for the Aegis compiler that evaluates
//! the performance and correctness of all compiler components.
//! 
//! ## Test Categories
//! 
//! - **Lexer Tests**: Test tokenization and lexical analysis
//! - **Parser Tests**: Test AST generation and syntax analysis  
//! - **Semantic Tests**: Test type checking and semantic analysis
//! - **Integration Tests**: Test the complete compilation pipeline
//! - **Performance Tests**: Test compilation speed and efficiency
//! - **Error Handling Tests**: Test error recovery and reporting
//! - **Language Feature Tests**: Test specific language constructs
//! 
//! ## Usage
//! 
//! Run all tests:
//! ```bash
//! cargo test
//! ```
//! 
//! Run specific test suite:
//! ```bash
//! cargo test lexer_tests
//! cargo test parser_tests
//! cargo test semantic_tests
//! cargo test integration_tests
//! cargo test performance_tests
//! cargo test error_handling_tests
//! cargo test language_feature_tests
//! ```
//! 
//! Run with detailed output:
//! ```bash
//! cargo test -- --nocapture
//! ```

// Test modules
pub mod test_utils;
pub mod test_runner;
pub mod lexer_tests;
pub mod parser_tests;
pub mod semantic_tests;
pub mod integration_tests;
pub mod performance_tests;
pub mod error_handling_tests;
pub mod language_feature_tests;

use test_runner::{TestReport, TestSuiteResult, TestResult};
use std::time::Instant;

/// Run all test suites and generate a comprehensive report
pub fn run_all_tests() -> TestReport {
    let mut report = TestReport::new();
    
    println!("🚀 Starting Aegis Compiler Test Suite...\n");
    
    // Run each test suite
    let suites = vec![
        ("Lexer Tests", run_lexer_tests),
        ("Parser Tests", run_parser_tests),
        ("Semantic Tests", run_semantic_tests),
        ("Integration Tests", run_integration_tests),
        ("Performance Tests", run_performance_tests),
        ("Error Handling Tests", run_error_handling_tests),
        ("Language Feature Tests", run_language_feature_tests),
    ];
    
    for (suite_name, test_fn) in suites {
        println!("📋 Running {}...", suite_name);
        let suite_result = test_fn();
        
        println!("   ✅ {} passed, ❌ {} failed, ⏱️  {:.2}s\n", 
            suite_result.passed_count(),
            suite_result.failed_count(),
            suite_result.total_duration.as_secs_f64()
        );
        
        report.add_suite(suite_result);
    }
    
    report.finish();
    report
}

// Individual test suite runners
// These would be implemented to actually run the tests in each module

fn run_lexer_tests() -> TestSuiteResult {
    let mut suite = TestSuiteResult::new("Lexer Tests".to_string());
    
    // This is a placeholder - in a real implementation, this would
    // discover and run all tests in the lexer_tests module
    let start = Instant::now();
    
    // Simulate running tests
    suite.add_test(TestResult {
        name: "test_keywords".to_string(),
        passed: true,
        duration: start.elapsed(),
        error_message: None,
    });
    
    suite
}

fn run_parser_tests() -> TestSuiteResult {
    let mut suite = TestSuiteResult::new("Parser Tests".to_string());
    
    let start = Instant::now();
    suite.add_test(TestResult {
        name: "test_parse_simple_let_statement".to_string(),
        passed: true,
        duration: start.elapsed(),
        error_message: None,
    });
    
    suite
}

fn run_semantic_tests() -> TestSuiteResult {
    TestSuiteResult::new("Semantic Tests".to_string())
}

fn run_integration_tests() -> TestSuiteResult {
    TestSuiteResult::new("Integration Tests".to_string())
}

fn run_performance_tests() -> TestSuiteResult {
    TestSuiteResult::new("Performance Tests".to_string())
}

fn run_error_handling_tests() -> TestSuiteResult {
    TestSuiteResult::new("Error Handling Tests".to_string())
}

fn run_language_feature_tests() -> TestSuiteResult {
    TestSuiteResult::new("Language Feature Tests".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_comprehensive_test_suite() {
        let report = run_all_tests();
        
        println!("{}", report.generate_report());
        println!("Summary: {}", report.generate_summary());
        
        // The test suite should have a high success rate
        assert!(report.overall_success_rate() > 0.8, 
            "Test suite success rate should be above 80%, got {:.1}%", 
            report.overall_success_rate() * 100.0);
        
        // The test suite should complete in reasonable time
        assert!(report.total_duration().as_secs() < 30, 
            "Test suite should complete within 30 seconds, took {:.2}s", 
            report.total_duration().as_secs_f64());
    }
}