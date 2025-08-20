@echo off
REM Aegis Compiler Test Suite Runner for Windows
REM This script runs the comprehensive test suite and generates a detailed report

echo 🚀 Starting Aegis Compiler Test Suite...
echo ========================================

echo.
echo 📋 Running Lexer Tests...
cargo test lexer_tests --release -- --nocapture
if errorlevel 1 goto :error

echo.
echo 📋 Running Parser Tests...
cargo test parser_tests --release -- --nocapture
if errorlevel 1 goto :error

echo.
echo 📋 Running Semantic Tests...
cargo test semantic_tests --release -- --nocapture
if errorlevel 1 goto :error

echo.
echo 📋 Running Integration Tests...
cargo test integration_tests --release -- --nocapture
if errorlevel 1 goto :error

echo.
echo 📋 Running Performance Tests...
cargo test performance_tests --release -- --nocapture
if errorlevel 1 goto :error

echo.
echo 📋 Running Error Handling Tests...
cargo test error_handling_tests --release -- --nocapture
if errorlevel 1 goto :error

echo.
echo 📋 Running Language Feature Tests...
cargo test language_feature_tests --release -- --nocapture
if errorlevel 1 goto :error

echo.
echo 🎯 Running comprehensive test suite...
cargo test run_comprehensive_test_suite --release -- --nocapture
if errorlevel 1 goto :error

echo.
echo ✅ All tests completed successfully!
echo 📊 Check the detailed report above for performance metrics and recommendations.
echo.
echo 🎉 Aegis Compiler Test Suite completed!
goto :end

:error
echo.
echo ❌ Some tests failed. Check the output above for details.
exit /b 1

:end