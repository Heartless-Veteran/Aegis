//! Tests for generic contract definitions and type checking

use aegis_compiler::{Scribe, Architect, Guardian};

#[test]
fn test_simple_generic_contract_parsing() {
    let input = r#"contract Option<T>:
    value: T
    is_some: boolean"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    // Should parse without errors
    assert!(architect.errors.is_empty(), "Expected no parse errors, but got: {:?}", architect.errors);
    
    // Check that the contract was parsed correctly
    assert_eq!(program.definitions.len(), 1);
    if let aegis_compiler::ast::Definition::Contract(contract_def) = &program.definitions[0] {
        assert_eq!(contract_def.name, "Option");
        assert_eq!(contract_def.generic_params, vec!["T"]);
        assert_eq!(contract_def.fields.len(), 2);
        
        // Check first field
        assert_eq!(contract_def.fields[0].name, "value");
        if let aegis_compiler::ast::TypeIdentifier::Simple { name, .. } = &contract_def.fields[0].type_ann {
            assert_eq!(name, "T");
        } else {
            panic!("Expected Simple type identifier for field 'value'");
        }
        
        // Check second field  
        assert_eq!(contract_def.fields[1].name, "is_some");
        if let aegis_compiler::ast::TypeIdentifier::Simple { name, .. } = &contract_def.fields[1].type_ann {
            assert_eq!(name, "boolean");
        } else {
            panic!("Expected Simple type identifier for field 'is_some'");
        }
    } else {
        panic!("Expected contract definition");
    }
}

#[test]
fn test_generic_contract_semantic_analysis() {
    let input = r#"contract Option<T>:
    value: T
    is_some: boolean"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should complete semantic analysis without errors
    assert!(guardian.errors.is_empty(), "Expected no semantic errors, but got: {:?}", guardian.errors);
}

#[test]
fn test_multiple_generic_parameters() {
    let input = r#"contract Map<K, V>:
    key: K
    value: V"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    // Should parse without errors
    assert!(architect.errors.is_empty(), "Expected no parse errors, but got: {:?}", architect.errors);
    
    // Check that the contract was parsed correctly
    assert_eq!(program.definitions.len(), 1);
    if let aegis_compiler::ast::Definition::Contract(contract_def) = &program.definitions[0] {
        assert_eq!(contract_def.name, "Map");
        assert_eq!(contract_def.generic_params, vec!["K", "V"]);
        assert_eq!(contract_def.fields.len(), 2);
    } else {
        panic!("Expected contract definition");
    }
}

#[test]
fn test_generic_with_builtin_list_type() {
    let input = r#"contract Container<T>:
    items: List<T>
    count: number"#;
    
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();
    
    // Should parse without errors
    assert!(architect.errors.is_empty(), "Expected no parse errors, but got: {:?}", architect.errors);
    
    // Check that the contract was parsed correctly
    assert_eq!(program.definitions.len(), 1);
    if let aegis_compiler::ast::Definition::Contract(contract_def) = &program.definitions[0] {
        assert_eq!(contract_def.name, "Container");
        assert_eq!(contract_def.generic_params, vec!["T"]);
        assert_eq!(contract_def.fields.len(), 2);
        
        // Check first field (should be a generic type)
        assert_eq!(contract_def.fields[0].name, "items");
        // For now, we only have simple type parsing, but this validates the structure
        
        // Check second field
        assert_eq!(contract_def.fields[1].name, "count");
        if let aegis_compiler::ast::TypeIdentifier::Simple { name, .. } = &contract_def.fields[1].type_ann {
            assert_eq!(name, "number");
        } else {
            panic!("Expected Simple type identifier for field 'count'");
        }
    } else {
        panic!("Expected contract definition");
    }
    
    let mut guardian = Guardian::new();
    guardian.check_program(&program);
    
    // Should complete semantic analysis without errors
    assert!(guardian.errors.is_empty(), "Expected no semantic errors, but got: {:?}", guardian.errors);
}