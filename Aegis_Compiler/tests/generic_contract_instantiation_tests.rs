//! Tests for generic contract instantiation and type checking

use aegis_compiler::{Scribe, Architect, Guardian};

#[test]
fn test_simple_generic_contract_definition() {
    // Test that we can define and parse a generic contract
    let input = r#"contract Option<T>:
    value: T
    is_some: boolean"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    // Should parse without errors
    assert!(architect.errors.is_empty(), "Expected no parse errors, but got: {:?}", architect.errors);
    
    // Check that the generic parameters were parsed
    if let Some(aegis_compiler::ast::Definition::Contract(contract_def)) = program.definitions.get(0) {
        assert_eq!(contract_def.name, "Option");
        assert_eq!(contract_def.generic_params, vec!["T"]);
        assert_eq!(contract_def.fields.len(), 2);
    } else {
        panic!("Expected contract definition");
    }
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should pass semantic analysis
    assert!(guardian.errors.is_empty(), "Expected no semantic errors, but got: {:?}", guardian.errors);
}

#[test]
fn test_multiple_generic_parameters_parsing() {
    let input = r#"contract Map<K, V>:
    key: K
    value: V"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    assert!(architect.errors.is_empty(), "Expected no parse errors, but got: {:?}", architect.errors);
    
    // Check that both generic parameters were parsed
    if let Some(aegis_compiler::ast::Definition::Contract(contract_def)) = program.definitions.get(0) {
        assert_eq!(contract_def.name, "Map");
        assert_eq!(contract_def.generic_params, vec!["K", "V"]);
        assert_eq!(contract_def.fields.len(), 2);
    } else {
        panic!("Expected contract definition");
    }
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should pass semantic analysis
    assert!(guardian.errors.is_empty(), "Expected no semantic errors, but got: {:?}", guardian.errors);
}

#[test]
fn test_regular_contract_still_works() {
    // Test that regular (non-generic) contracts still work
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
    
    assert!(architect.errors.is_empty(), "Expected no parse errors, but got: {:?}", architect.errors);
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should pass semantic analysis
    assert!(guardian.errors.is_empty(), "Expected no semantic errors, but got: {:?}", guardian.errors);
}

#[test] 
fn test_contract_field_type_resolution() {
    // Test that generic type parameters are properly resolved in contract fields
    let input = r#"contract Option<T>:
    value: T
    is_some: boolean"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    assert!(architect.errors.is_empty(), "Expected no parse errors, but got: {:?}", architect.errors);
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should pass semantic analysis - the Guardian should handle generic type parameters
    assert!(guardian.errors.is_empty(), "Expected no semantic errors, but got: {:?}", guardian.errors);
    
    // Verify that the contract was registered as a GenericContract
    if let Some(symbol) = guardian.symbol_table.resolve("Option") {
        if let aegis_compiler::guardian_symbol_table::SymbolKind::GenericContract { params, fields } = &symbol.kind {
            assert_eq!(params, &vec!["T"]);
            assert_eq!(fields.len(), 2);
            assert!(fields.contains_key("value"));
            assert!(fields.contains_key("is_some"));
            
            // The value field should be a generic type parameter
            if let Some(value_type) = fields.get("value") {
                if let aegis_compiler::guardian_types::Type::Generic(param_name) = value_type {
                    assert_eq!(param_name, "T");
                } else {
                    panic!("Expected value field to be Generic type, but got: {:?}", value_type);
                }
            }
            
            // The is_some field should be Boolean
            if let Some(is_some_type) = fields.get("is_some") {
                assert_eq!(is_some_type, &aegis_compiler::guardian_types::Type::Boolean);
            } else {
                panic!("Expected is_some field");
            }
        } else {
            panic!("Expected GenericContract symbol kind, but got: {:?}", symbol.kind);
        }
    } else {
        panic!("Option contract not found in symbol table");
    }
}