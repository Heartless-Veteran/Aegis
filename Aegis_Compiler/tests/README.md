# Aegis Compiler Test Suite

This is a comprehensive test suite for the Aegis programming language compiler. It evaluates the performance and correctness of all compiler components through various test categories.

## Test Categories

### 1. Lexer Tests (`lexer_tests.rs`)
Tests the tokenization and lexical analysis phase of compilation:
- **Keywords**: Tests recognition of all Aegis keywords (`let's`, `app`, `track`, etc.)
- **Operators**: Tests all operators (`+`, `-`, `==`, `=>`, etc.)
- **Delimiters**: Tests parentheses, braces, commas, colons
- **Identifiers**: Tests variable and function name recognition
- **Numbers**: Tests numeric literal parsing
- **Strings**: Tests string literal parsing with proper escaping
- **Comments**: Tests comment handling and whitespace skipping
- **Error Cases**: Tests handling of illegal characters and malformed input

### 2. Parser Tests (`parser_tests.rs`)
Tests the parsing and AST generation phase:
- **Variable Declarations**: Tests `let's` statements with and without tracking
- **Function Definitions**: Tests function parsing with parameters and return types
- **Async Functions**: Tests `async`/`await` syntax parsing
- **Contract Definitions**: Tests custom type definitions
- **App Definitions**: Tests application structure parsing
- **Expressions**: Tests all expression types (literals, infix, prefix, calls)
- **Control Flow**: Tests `if`, `for`, `when` constructs
- **UI Components**: Tests UI element parsing in `show` blocks

### 3. Semantic Tests (`semantic_tests.rs`)
Tests the semantic analysis and type checking phase:
- **Type Checking**: Tests type compatibility and inference
- **Symbol Resolution**: Tests variable and function lookup
- **Function Validation**: Tests parameter/argument matching
- **Async Validation**: Tests `await` usage in async contexts
- **Contract Validation**: Tests custom type usage
- **Scope Rules**: Tests variable visibility and scoping
- **Error Detection**: Tests detection of semantic errors

### 4. Integration Tests (`integration_tests.rs`)
Tests the complete compilation pipeline:
- **End-to-End Compilation**: Tests full source-to-AST pipeline
- **Error Recovery**: Tests compiler behavior with various error types
- **Multi-Definition Programs**: Tests complex programs with multiple components
- **Performance Tracking**: Tests compilation speed for different program sizes

### 5. Performance Tests (`performance_tests.rs`)
Tests compilation speed and efficiency:
- **Lexer Performance**: Tests tokenization speed
- **Parser Performance**: Tests parsing speed
- **Semantic Analysis Performance**: Tests type checking speed
- **Large Program Handling**: Tests scalability with large codebases
- **Memory Usage**: Tests for memory leaks and efficient resource usage
- **Throughput Measurement**: Tests processing speed in bytes/second

### 6. Error Handling Tests (`error_handling_tests.rs`)
Tests error detection, reporting, and recovery:
- **Lexical Errors**: Tests handling of illegal characters and malformed tokens
- **Syntax Errors**: Tests handling of malformed syntax
- **Semantic Errors**: Tests detection of type errors, undefined variables, etc.
- **Error Recovery**: Tests compiler's ability to continue after errors
- **Error Messages**: Tests quality and helpfulness of error messages
- **Error Limits**: Tests reasonable limits on error reporting

### 7. Language Feature Tests (`language_feature_tests.rs`)
Tests specific Aegis language constructs:
- **Variable Declarations**: All forms of variable declarations
- **Function Definitions**: Regular and async functions
- **Contract Definitions**: Custom type definitions
- **App Definitions**: Application structure
- **Control Flow**: Conditionals, loops, pattern matching
- **UI Components**: User interface elements and styling
- **Event Handling**: UI event bindings
- **Data Structures**: Lists, maps, sets
- **String Interpolation**: Template strings
- **Type Annotations**: Explicit type specifications
- **Reactive Variables**: Tracked state variables

## Running Tests

### Run All Tests
```bash
cd Aegis_Compiler
cargo test
```

### Run Specific Test Suite
```bash
cargo test lexer_tests
cargo test parser_tests
cargo test semantic_tests
cargo test integration_tests
cargo test performance_tests
cargo test error_handling_tests
cargo test language_feature_tests
```

### Run with Detailed Output
```bash
cargo test -- --nocapture
```

### Run Performance Tests Only
```bash
cargo test performance_tests -- --nocapture
```

### Run Tests in Release Mode (for accurate performance measurements)
```bash
cargo test --release
```

## Test Utilities

The `test_utils.rs` module provides:
- **TestFixtures**: Pre-defined code samples for testing
- **PerformanceTimer**: Utilities for measuring execution time
- **Helper Functions**: Common test operations like tokenization

## Test Report Generation

The test suite includes a comprehensive reporting system (`test_runner.rs`) that provides:
- **Success/Failure Rates**: Overall test statistics
- **Performance Metrics**: Timing information for each test suite
- **Error Analysis**: Detailed information about failed tests
- **Recommendations**: Suggestions for improving test performance

## Expected Outcomes

A successful test run should demonstrate:

1. **High Success Rate**: >95% of tests should pass
2. **Fast Compilation**: Most tests should complete in <100ms
3. **Comprehensive Coverage**: All language features should be tested
4. **Robust Error Handling**: Compiler should handle malformed input gracefully
5. **Scalable Performance**: Large programs should compile in reasonable time

## Continuous Integration

The test suite is designed to work with CI/CD systems:
- Tests provide clear pass/fail status
- Performance regressions are detected
- Detailed reports help with debugging
- Test results can be integrated into build pipelines

## Contributing to Tests

When adding new language features:
1. Add test fixtures to `test_utils.rs`
2. Add lexer tests for new tokens
3. Add parser tests for new syntax
4. Add semantic tests for new validation rules
5. Add integration tests for complete features
6. Add performance tests for potentially slow operations
7. Add error handling tests for new error conditions
8. Add language feature tests for user-facing functionality

This comprehensive test suite ensures the Aegis compiler is reliable, performant, and user-friendly.