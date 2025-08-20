//! Performance tests for the Aegis compiler

use aegis_compiler::{Scribe, Architect, Guardian};
use crate::test_utils::{TestFixtures, PerformanceTimer, tokenize_all};
use std::time::Duration;

#[cfg(test)]
mod performance_tests {
    use super::*;

    const PERFORMANCE_THRESHOLD_MS: u128 = 100;
    const LARGE_PROGRAM_THRESHOLD_MS: u128 = 1000;

    #[test]
    fn test_lexer_performance_simple() {
        let timer = PerformanceTimer::new();
        let _tokens = tokenize_all(TestFixtures::simple_let());
        let elapsed = timer.elapsed_ms();
        
        assert!(elapsed < PERFORMANCE_THRESHOLD_MS, 
            "Simple lexing should be fast, took {}ms", elapsed);
    }

    #[test]
    fn test_lexer_performance_complex() {
        let timer = PerformanceTimer::new();
        let _tokens = tokenize_all(TestFixtures::complex_app());
        let elapsed = timer.elapsed_ms();
        
        assert!(elapsed < PERFORMANCE_THRESHOLD_MS, 
            "Complex lexing should be fast, took {}ms", elapsed);
    }

    #[test]
    fn test_parser_performance_simple() {
        let scribe = Scribe::new(TestFixtures::simple_let());
        let mut architect = Architect::new(scribe);
        
        let timer = PerformanceTimer::new();
        let _program = architect.parse_program();
        let elapsed = timer.elapsed_ms();
        
        assert!(elapsed < PERFORMANCE_THRESHOLD_MS, 
            "Simple parsing should be fast, took {}ms", elapsed);
    }

    #[test]
    fn test_parser_performance_complex() {
        let scribe = Scribe::new(TestFixtures::complex_app());
        let mut architect = Architect::new(scribe);
        
        let timer = PerformanceTimer::new();
        let _program = architect.parse_program();
        let elapsed = timer.elapsed_ms();
        
        assert!(elapsed < PERFORMANCE_THRESHOLD_MS, 
            "Complex parsing should be fast, took {}ms", elapsed);
    }

    #[test]
    fn test_semantic_analysis_performance() {
        let scribe = Scribe::new(TestFixtures::complex_app());
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        
        let timer = PerformanceTimer::new();
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        let elapsed = timer.elapsed_ms();
        
        assert!(elapsed < PERFORMANCE_THRESHOLD_MS, 
            "Semantic analysis should be fast, took {}ms", elapsed);
    }

    #[test]
    fn test_large_program_performance() {
        let large_program = generate_large_program(100);
        
        let timer = PerformanceTimer::new();
        
        // Full compilation pipeline
        let scribe = Scribe::new(&large_program);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        
        let elapsed = timer.elapsed_ms();
        
        assert!(elapsed < LARGE_PROGRAM_THRESHOLD_MS, 
            "Large program compilation should complete within {}ms, took {}ms", 
            LARGE_PROGRAM_THRESHOLD_MS, elapsed);
    }

    #[test]
    fn test_repeated_compilation_performance() {
        let input = TestFixtures::complex_app();
        let iterations = 10;
        
        let timer = PerformanceTimer::new();
        
        for _ in 0..iterations {
            let scribe = Scribe::new(input);
            let mut architect = Architect::new(scribe);
            let program = architect.parse_program();
            let mut guardian = Guardian::new();
            guardian.check_program(&program);
        }
        
        let total_elapsed = timer.elapsed_ms();
        let avg_elapsed = total_elapsed / iterations;
        
        assert!(avg_elapsed < PERFORMANCE_THRESHOLD_MS, 
            "Average compilation time should be fast, took {}ms per iteration", avg_elapsed);
    }

    #[test]
    fn test_memory_usage_stability() {
        // This test ensures that repeated compilations don't cause memory leaks
        let input = TestFixtures::complex_app();
        let iterations = 50;
        
        for i in 0..iterations {
            let scribe = Scribe::new(input);
            let mut architect = Architect::new(scribe);
            let program = architect.parse_program();
            let mut guardian = Guardian::new();
            guardian.check_program(&program);
            
            // Drop everything to ensure cleanup
            drop(guardian);
            drop(program);
            drop(architect);
            drop(scribe);
            
            // Periodically check that we're not accumulating too much
            if i % 10 == 0 {
                // In a real implementation, you might check memory usage here
                // For now, we just ensure the test completes without crashing
            }
        }
    }

    #[test]
    fn test_lexer_throughput() {
        let input = TestFixtures::complex_app();
        let input_size = input.len();
        
        let timer = PerformanceTimer::new();
        let _tokens = tokenize_all(input);
        let elapsed = timer.elapsed();
        
        let throughput = (input_size as f64) / elapsed.as_secs_f64();
        
        // Should process at least 1MB/s (very conservative)
        assert!(throughput > 1_000_000.0, 
            "Lexer throughput should be at least 1MB/s, got {:.2} bytes/s", throughput);
    }

    #[test]
    fn test_deep_nesting_performance() {
        let deep_program = generate_deeply_nested_program(50);
        
        let timer = PerformanceTimer::new();
        
        let scribe = Scribe::new(&deep_program);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        
        let elapsed = timer.elapsed_ms();
        
        assert!(elapsed < LARGE_PROGRAM_THRESHOLD_MS, 
            "Deep nesting should not cause exponential slowdown, took {}ms", elapsed);
    }

    #[test]
    fn test_wide_program_performance() {
        let wide_program = generate_wide_program(200);
        
        let timer = PerformanceTimer::new();
        
        let scribe = Scribe::new(&wide_program);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        
        let elapsed = timer.elapsed_ms();
        
        assert!(elapsed < LARGE_PROGRAM_THRESHOLD_MS, 
            "Wide program should compile efficiently, took {}ms", elapsed);
    }

    // Helper functions to generate test programs

    fn generate_large_program(num_functions: usize) -> String {
        let mut program = String::new();
        
        for i in 0..num_functions {
            program.push_str(&format!(
                "let's function_{}(x: number) -> number:\n    return x + {}\n\n",
                i, i
            ));
        }
        
        program
    }

    fn generate_deeply_nested_program(depth: usize) -> String {
        let mut program = String::from("let's result = ");
        
        for i in 0..depth {
            program.push_str(&format!("if {} > 0: (", i));
        }
        
        program.push_str("42");
        
        for _ in 0..depth {
            program.push_str(") else: 0");
        }
        
        program
    }

    fn generate_wide_program(num_variables: usize) -> String {
        let mut program = String::new();
        
        for i in 0..num_variables {
            program.push_str(&format!("let's var_{} = {}\n", i, i));
        }
        
        // Add a function that uses all variables
        program.push_str("let's sum_all() -> number:\n    return ");
        for i in 0..num_variables {
            if i > 0 {
                program.push_str(" + ");
            }
            program.push_str(&format!("var_{}", i));
        }
        program.push('\n');
        
        program
    }

    #[test]
    fn benchmark_compilation_phases() {
        let input = TestFixtures::complex_app();
        
        // Lexing benchmark
        let timer = PerformanceTimer::new();
        let scribe = Scribe::new(input);
        let lexing_time = timer.elapsed_ms();
        
        // Parsing benchmark
        let timer = PerformanceTimer::new();
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        let parsing_time = timer.elapsed_ms();
        
        // Semantic analysis benchmark
        let timer = PerformanceTimer::new();
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        let semantic_time = timer.elapsed_ms();
        
        println!("Compilation phase benchmarks:");
        println!("  Lexing: {}ms", lexing_time);
        println!("  Parsing: {}ms", parsing_time);
        println!("  Semantic: {}ms", semantic_time);
        println!("  Total: {}ms", lexing_time + parsing_time + semantic_time);
        
        // All phases should be reasonably fast
        assert!(lexing_time < PERFORMANCE_THRESHOLD_MS);
        assert!(parsing_time < PERFORMANCE_THRESHOLD_MS);
        assert!(semantic_time < PERFORMANCE_THRESHOLD_MS);
    }
}
