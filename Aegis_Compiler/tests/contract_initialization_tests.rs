//! Tests for contract initialization type checking

use aegis_compiler::{Scribe, Architect, Guardian};

#[test]
fn test_simple_contract_initialization() {
    let input = r#"contract User:
    id: number
    name: string

let's user: User = {
    id: 42,
    name: "Alice"
}"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should pass - correct contract initialization
    assert!(guardian.errors.is_empty(), "Expected no errors, but got: {:?}", guardian.errors);
}

#[test]
fn test_contract_initialization_missing_field() {
    let input = r#"contract User:
    id: number
    name: string

let's user: User = {
    id: 42
    # Missing 'name' field
}"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should fail - missing required field
    assert!(!guardian.errors.is_empty(), "Expected errors for missing field");
}

#[test]
fn test_contract_initialization_wrong_type() {
    let input = r#"contract User:
    id: number
    name: string

let's user: User = {
    id: "not_a_number",
    name: "Alice"
}"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should fail - wrong field type
    assert!(!guardian.errors.is_empty(), "Expected errors for wrong field type");
}

#[test]
fn test_contract_initialization_extra_field() {
    let input = r#"contract User:
    id: number
    name: string

let's user: User = {
    id: 42,
    name: "Alice",
    extra_field: "not_allowed"
}"#;
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should fail - extra field not in contract
    assert!(!guardian.errors.is_empty(), "Expected errors for extra field");
}