//! Comprehensive test suite for the Aegis compiler
//! 
//! This module contains integration tests that evaluate the performance
//! and correctness of the entire compiler pipeline.

pub mod lexer_tests;
pub mod parser_tests;
pub mod semantic_tests;
pub mod integration_tests;
pub mod performance_tests;
pub mod error_handling_tests;
pub mod language_feature_tests;

// Test utilities and common fixtures
pub mod test_utils;