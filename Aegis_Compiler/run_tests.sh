#!/bin/bash

# Aegis Compiler Test Suite Runner
# This script runs the comprehensive test suite and generates a detailed report

set -e

echo "🚀 Starting Aegis Compiler Test Suite..."
echo "========================================"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to run a test suite and capture results
run_test_suite() {
    local suite_name=$1
    local test_pattern=$2
    
    echo -e "\n📋 Running ${suite_name}..."
    
    if cargo test $test_pattern --release -- --nocapture; then
        echo -e "${GREEN}✅ ${suite_name} completed successfully${NC}"
    else
        echo -e "${RED}❌ ${suite_name} failed${NC}"
        return 1
    fi
}

# Run individual test suites
run_test_suite "Lexer Tests" "lexer_tests"
run_test_suite "Parser Tests" "parser_tests"
run_test_suite "Semantic Tests" "semantic_tests"
run_test_suite "Integration Tests" "integration_tests"
run_test_suite "Performance Tests" "performance_tests"
run_test_suite "Error Handling Tests" "error_handling_tests"
run_test_suite "Language Feature Tests" "language_feature_tests"

echo -e "\n🎯 Running comprehensive test suite..."
if cargo test run_comprehensive_test_suite --release -- --nocapture; then
    echo -e "${GREEN}✅ All tests completed successfully!${NC}"
    echo -e "\n📊 Check the detailed report above for performance metrics and recommendations."
else
    echo -e "${RED}❌ Some tests failed. Check the output above for details.${NC}"
    exit 1
fi

echo -e "\n🎉 Aegis Compiler Test Suite completed!"