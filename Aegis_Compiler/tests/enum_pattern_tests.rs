//! Tests for enum pattern matching and instantiation functionality

use aegis_compiler::{Guardian, ast::*};
use aegis_compiler::guardian_types::Type;

#[test]
fn test_enum_pattern_matching_integration() {
    // This tests the basic integration without actually parsing enum syntax
    // (since the parser is currently a stub)
    
    let mut guardian = Guardian::new();
    
    // Create an enum definition manually to test semantic analysis
    let enum_def = EnumDefinition {
        name: "LoadState".to_string(),
        variants: vec![
            EnumVariant { name: "Success".to_string(), types: vec![], span: Default::default() },
            EnumVariant { name: "Loading".to_string(), types: vec![], span: Default::default() },
            EnumVariant { name: "Failed".to_string(), types: vec![], span: Default::default() },
        ],
        span: Default::default(),
    };
    
    // Test that enum definition is properly handled
    guardian.check_enum_definition(&enum_def);
    
    // The Guardian should not have any errors
    assert!(guardian.errors.is_empty());
}

#[test]
fn test_enum_pattern_ast_structure() {
    // Test that the new EnumVariant pattern can be constructed
    let enum_pattern = WhenPattern::EnumVariant {
        enum_name: "MyEnum".to_string(),
        variant_name: "MyVariant".to_string(),
        span: Default::default(),
    };
    
    // Verify the structure
    match enum_pattern {
        WhenPattern::EnumVariant { enum_name, variant_name, .. } => {
            assert_eq!(enum_name, "MyEnum");
            assert_eq!(variant_name, "MyVariant");
        }
        _ => panic!("Expected EnumVariant pattern"),
    }
}

#[test]
fn test_enum_instantiation_via_member_access() {
    let mut guardian = Guardian::new();
    
    // Define an enum
    let enum_def = EnumDefinition {
        name: "Status".to_string(),
        variants: vec![
            EnumVariant { name: "Active".to_string(), types: vec![], span: Default::default() },
            EnumVariant { name: "Inactive".to_string(), types: vec![], span: Default::default() },
        ],
        span: Default::default(),
    };
    
    guardian.check_enum_definition(&enum_def);
    
    // Test valid enum instantiation (Status::Active)
    let valid_instantiation = Expression::MemberAccess(Box::new(MemberAccessExpression {
        object: Expression::Identifier("Status".to_string(), Default::default()),
        property: "Active".to_string(),
        span: Default::default(),
    }));
    
    let result_type = guardian.infer_expression_type(&valid_instantiation);
    // The enum type should match with the expected structure
    if let Type::Enum { name, variants } = result_type {
        assert_eq!(name, "Status");
        assert_eq!(variants.len(), 2);
        assert_eq!(variants.get("Active"), Some(&vec![]));
        assert_eq!(variants.get("Inactive"), Some(&vec![]));
    } else {
        panic!("Expected Type::Enum but got {:?}", result_type);
    }
    
    // Test invalid enum instantiation (Status::NonExistent)
    let invalid_instantiation = Expression::MemberAccess(Box::new(MemberAccessExpression {
        object: Expression::Identifier("Status".to_string(), Default::default()),
        property: "NonExistent".to_string(),
        span: Default::default(),
    }));
    
    let error_type = guardian.infer_expression_type(&invalid_instantiation);
    assert_eq!(error_type, Type::Error);
}

#[test]
fn test_enum_with_associated_data() {
    let mut guardian = Guardian::new();
    
    // Define an enum with associated data
    let enum_def = EnumDefinition {
        name: "LoadState".to_string(),
        variants: vec![
            EnumVariant { name: "Loading".to_string(), types: vec![], span: Default::default() },
            EnumVariant { name: "Success".to_string(), types: vec!["Data".to_string()], span: Default::default() },
            EnumVariant { name: "Failure".to_string(), types: vec!["Error".to_string()], span: Default::default() },
        ],
        span: Default::default(),
    };
    
    guardian.check_enum_definition(&enum_def);
    
    // Test that enum was properly defined
    assert!(guardian.errors.is_empty());
    
    // Test valid enum instantiation with data (LoadState::Success(data))
    let call_instantiation = Expression::Call(Box::new(CallExpression {
        function: Expression::MemberAccess(Box::new(MemberAccessExpression {
            object: Expression::Identifier("LoadState".to_string(), Default::default()),
            property: "Success".to_string(),
            span: Default::default(),
        })),
        arguments: vec![
            Expression::Identifier("data".to_string(), Default::default()),
        ],
        span: Default::default(),
    }));
    
    // This should work if we had proper type checking, but for now, the argument
    // type checking will fail because "data" is not defined. The structure is correct though.
    let result_type = guardian.infer_expression_type(&call_instantiation);
    // We expect Type::Error because the argument "data" is not defined
    assert_eq!(result_type, Type::Error);
    
    // Test invalid enum instantiation with wrong number of arguments
    let invalid_call = Expression::Call(Box::new(CallExpression {
        function: Expression::MemberAccess(Box::new(MemberAccessExpression {
            object: Expression::Identifier("LoadState".to_string(), Default::default()),
            property: "Success".to_string(),
            span: Default::default(),
        })),
        arguments: vec![], // Wrong number of arguments (should be 1)
        span: Default::default(),
    }));
    
    let error_type = guardian.infer_expression_type(&invalid_call);
    assert_eq!(error_type, Type::Error);
}

#[test]
fn test_when_expression_with_enum_variants() {
    let mut guardian = Guardian::new();
    
    // Define an enum
    let enum_def = EnumDefinition {
        name: "Color".to_string(),
        variants: vec![
            EnumVariant { name: "Red".to_string(), types: vec![], span: Default::default() },
            EnumVariant { name: "Green".to_string(), types: vec![], span: Default::default() },
            EnumVariant { name: "Blue".to_string(), types: vec![], span: Default::default() },
        ],
        span: Default::default(),
    };
    
    guardian.check_enum_definition(&enum_def);
    
    // Create a when expression with enum variant patterns
    // Note: For this test to work properly, we need to define the subject variable
    // Let's use a simpler approach and focus on testing the pattern matching logic
    
    // First, let's create a valid enum instantiation expression that we can use as the subject
    let enum_instantiation = Expression::MemberAccess(Box::new(MemberAccessExpression {
        object: Expression::Identifier("Color".to_string(), Default::default()),
        property: "Red".to_string(),
        span: Default::default(),
    }));
    
    let when_expr = Expression::When(Box::new(WhenExpression {
        value: enum_instantiation,
        cases: vec![
            WhenCase {
                pattern: WhenPattern::EnumVariant {
                    enum_name: "Color".to_string(),
                    variant_name: "Red".to_string(),
                    span: Default::default(),
                },
                body: Expression::Literal(Literal::String("It's red!".to_string()), Default::default()),
                span: Default::default(),
            },
            WhenCase {
                pattern: WhenPattern::EnumVariant {
                    enum_name: "Color".to_string(),
                    variant_name: "Green".to_string(),
                    span: Default::default(),
                },
                body: Expression::Literal(Literal::String("It's green!".to_string()), Default::default()),
                span: Default::default(),
            },
        ],
        span: Default::default(),
    }));
    
    // The when expression should return String type
    let result_type = guardian.infer_expression_type(&when_expr);
    
    // Since all cases return String, the result should be String
    assert_eq!(result_type, Type::String);
}