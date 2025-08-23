//! Parser tests for the Aegis compiler

use aegis_compiler::{Scribe, Architect};

#[test]
fn test_parse_simple_program() {
    let input = "let's x = 42";
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    
    let program = architect.parse_program();
    
    // The real implementation should not produce errors
    assert!(architect.errors.is_empty());
    
    // The real implementation now parses let statements
    assert_eq!(program.definitions.len(), 1);
}

#[test]
fn test_parse_empty_program() {
    let input = "";
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    
    let program = architect.parse_program();
    
    assert!(architect.errors.is_empty());
    assert_eq!(program.definitions.len(), 0);
}

#[test]
fn test_parse_function_stub() {
    let input = r#"let's add(a: number, b: number) -> number:
    return a + b"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    
    let program = architect.parse_program();
    
    // Function parsing is not fully implemented yet, so it will generate errors
    // when trying to parse as let statement but finding function syntax
    println!("Errors: {:?}", architect.errors);
    assert_eq!(program.definitions.len(), 0);
}

#[test]
fn test_parse_contract_stub() {
    let input = r#"contract User:
    id: number
    name: string"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    
    let program = architect.parse_program();
    
    // The real implementation should parse contracts correctly
    assert!(architect.errors.is_empty());
    assert_eq!(program.definitions.len(), 1);
}

#[test]
fn test_parse_app_stub() {
    let input = r#"app MyApp:
    show:
        text "Hello""#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    
    let program = architect.parse_program();
    
    // The stub implementation should handle this gracefully
    assert!(architect.errors.is_empty());
    assert_eq!(program.definitions.len(), 0);
}