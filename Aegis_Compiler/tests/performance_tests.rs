//! Performance tests for the Aegis compiler

use aegis_compiler::{Scribe, Architect, Guardian};
use std::time::Instant;

#[test]
fn test_lexer_performance() {
    let input = "let's x = 42";
    
    let start = Instant::now();
    let mut scribe = Scribe::new(input);
    
    // Tokenize the entire input
    loop {
        let token = scribe.next_token();
        if matches!(token, aegis_compiler::Token::Eof(_)) {
            break;
        }
    }
    
    let elapsed = start.elapsed();
    
    // Should complete very quickly
    assert!(elapsed.as_millis() < 100, "Lexing should be fast, took {:?}", elapsed);
}

#[test]
fn test_parser_performance() {
    let input = "let's x = 42";
    
    let start = Instant::now();
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let _program = architect.parse_program();
    let elapsed = start.elapsed();
    
    // Should complete very quickly
    assert!(elapsed.as_millis() < 100, "Parsing should be fast, took {:?}", elapsed);
}

#[test]
fn test_semantic_analysis_performance() {
    let input = "let's x = 42";
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    let start = Instant::now();
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    let elapsed = start.elapsed();
    
    // Should complete very quickly
    assert!(elapsed.as_millis() < 100, "Semantic analysis should be fast, took {:?}", elapsed);
}

#[test]
fn test_complete_pipeline_performance() {
    let input = r#"let's add(a: number, b: number) -> number:
    return a + b

let's result = add(1, 2)"#;
    
    let start = Instant::now();
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    let elapsed = start.elapsed();
    
    // Complete pipeline should be fast
    assert!(elapsed.as_millis() < 200, "Complete pipeline should be fast, took {:?}", elapsed);
}

#[test]
fn test_large_input_performance() {
    // Generate a larger input
    let mut input = String::new();
    for i in 0..100 {
        input.push_str(&format!("let's var_{} = {}\n", i, i));
    }
    
    let start = Instant::now();
    
    let scribe = Scribe::new(&input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    let elapsed = start.elapsed();
    
    // Should still be reasonably fast even with larger input
    assert!(elapsed.as_millis() < 1000, "Large input processing should be reasonable, took {:?}", elapsed);
}