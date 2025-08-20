//! Integration tests for the Aegis compiler

use aegis_compiler::{Scribe, Architect, Guardian};

#[test]
fn test_complete_compilation_pipeline() {
    let input = "let's x = 42";
    
    // Lexical analysis
    let scribe = Scribe::new(input);
    
    // Parsing
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    // Semantic analysis
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // All phases should complete without errors
    assert!(architect.errors.is_empty());
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_pipeline_with_function() {
    let input = r#"let's add(a: number, b: number) -> number:
    return a + b"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    assert!(architect.errors.is_empty());
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_pipeline_with_contract() {
    let input = r#"contract User:
    id: number
    name: string"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    assert!(architect.errors.is_empty());
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_pipeline_with_app() {
    let input = r#"app MyApp:
    show:
        text "Hello""#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    assert!(architect.errors.is_empty());
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_empty_program_pipeline() {
    let input = "";
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    assert!(architect.errors.is_empty());
    assert!(guardian.errors.is_empty());
}