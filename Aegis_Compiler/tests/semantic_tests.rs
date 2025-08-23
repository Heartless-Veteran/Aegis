//! Semantic analysis tests for the Aegis compiler

use aegis_compiler::{Architect, Guardian, Scribe};

#[test]
fn test_semantic_analysis_simple() {
    let input = "let's x = 42";
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();

    let mut guardian = Guardian::new();
    guardian.check_program(&program);

    // The stub implementation should not produce errors
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_semantic_analysis_empty() {
    let input = "";
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();

    let mut guardian = Guardian::new();
    guardian.check_program(&program);

    assert!(guardian.errors.is_empty());
}

#[test]
fn test_semantic_analysis_function() {
    let input = r#"let's add(a: number, b: number) -> number:
    return a + b"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();

    let mut guardian = Guardian::new();
    guardian.check_program(&program);

    // The stub implementation should handle this gracefully
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_semantic_analysis_contract() {
    let input = r#"contract User:
    id: number
    name: string"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();

    let mut guardian = Guardian::new();
    guardian.check_program(&program);

    // The stub implementation should handle this gracefully
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_semantic_analysis_app() {
    let input = r#"app MyApp:
    show:
        text "Hello""#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();

    let mut guardian = Guardian::new();
    guardian.check_program(&program);

    // The stub implementation should handle this gracefully
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_semantic_analysis_enum() {
    let input = r#"enum Status:
    Active
    Inactive
    Pending"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();

    let mut guardian = Guardian::new();
    guardian.check_program(&program);

    // The stub implementation should handle this gracefully
    assert!(guardian.errors.is_empty());
}
