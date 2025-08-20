//! Semantic analysis (Guardian) tests for the Aegis compiler

use aegis_compiler::{Scribe, Architect, Guardian};
use crate::test_utils::TestFixtures;

#[cfg(test)]
mod semantic_tests {
    use super::*;

    fn analyze_program(input: &str) -> Guardian {
        let scribe = Scribe::new(input);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        guardian
    }

    #[test]
    fn test_valid_simple_let() {
        let guardian = analyze_program(TestFixtures::simple_let());
        assert!(guardian.errors.is_empty(), "Should have no semantic errors");
    }

    #[test]
    fn test_valid_tracked_variable() {
        let guardian = analyze_program(TestFixtures::tracked_variable());
        assert!(guardian.errors.is_empty(), "Should have no semantic errors");
    }

    #[test]
    fn test_valid_function_definition() {
        let guardian = analyze_program(TestFixtures::simple_function());
        assert!(guardian.errors.is_empty(), "Should have no semantic errors");
    }

    #[test]
    fn test_valid_async_function() {
        let guardian = analyze_program(TestFixtures::async_function());
        assert!(guardian.errors.is_empty(), "Should have no semantic errors");
    }

    #[test]
    fn test_valid_contract_definition() {
        let guardian = analyze_program(TestFixtures::simple_contract());
        assert!(guardian.errors.is_empty(), "Should have no semantic errors");
    }

    #[test]
    fn test_valid_app_definition() {
        let guardian = analyze_program(TestFixtures::simple_app());
        assert!(guardian.errors.is_empty(), "Should have no semantic errors");
    }

    #[test]
    fn test_undeclared_variable_error() {
        let input = "let's result = undefined_var + 5";
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_undeclared_error = guardian.errors.iter().any(|error| {
            error.message.contains("undefined") || error.message.contains("undeclared")
        });
        assert!(has_undeclared_error, "Should have undeclared variable error");
    }

    #[test]
    fn test_type_mismatch_error() {
        let input = r#"let's x: number = "string value""#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_type_error = guardian.errors.iter().any(|error| {
            error.message.contains("type") && (error.message.contains("mismatch") || error.message.contains("expected"))
        });
        assert!(has_type_error, "Should have type mismatch error");
    }

    #[test]
    fn test_function_call_wrong_arity() {
        let input = r#"let's add(a: number, b: number) -> number:
    return a + b

let's result = add(1, 2, 3)"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_arity_error = guardian.errors.iter().any(|error| {
            error.message.contains("argument") || error.message.contains("parameter")
        });
        assert!(has_arity_error, "Should have function arity error");
    }

    #[test]
    fn test_function_call_wrong_types() {
        let input = r#"let's add(a: number, b: number) -> number:
    return a + b

let's result = add("hello", "world")"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_type_error = guardian.errors.iter().any(|error| {
            error.message.contains("type") || error.message.contains("expected")
        });
        assert!(has_type_error, "Should have function argument type error");
    }

    #[test]
    fn test_return_type_mismatch() {
        let input = r#"let's get_number() -> number:
    return "not a number""#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_return_error = guardian.errors.iter().any(|error| {
            error.message.contains("return") && error.message.contains("type")
        });
        assert!(has_return_error, "Should have return type mismatch error");
    }

    #[test]
    fn test_await_outside_async_function() {
        let input = r#"let's regular_function():
    let's result = await some_async_call()
    return result"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_await_error = guardian.errors.iter().any(|error| {
            error.message.contains("await") && error.message.contains("async")
        });
        assert!(has_await_error, "Should have await outside async function error");
    }

    #[test]
    fn test_duplicate_variable_declaration() {
        let input = r#"let's x = 5
let's x = 10"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_duplicate_error = guardian.errors.iter().any(|error| {
            error.message.contains("duplicate") || error.message.contains("already declared")
        });
        assert!(has_duplicate_error, "Should have duplicate variable error");
    }

    #[test]
    fn test_duplicate_function_declaration() {
        let input = r#"let's add(a: number, b: number) -> number:
    return a + b

let's add(x: number, y: number) -> number:
    return x + y"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_duplicate_error = guardian.errors.iter().any(|error| {
            error.message.contains("duplicate") || error.message.contains("already declared")
        });
        assert!(has_duplicate_error, "Should have duplicate function error");
    }

    #[test]
    fn test_invalid_member_access() {
        let input = r#"let's x: number = 42
let's result = x.invalid_property"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_member_error = guardian.errors.iter().any(|error| {
            error.message.contains("property") || error.message.contains("member")
        });
        assert!(has_member_error, "Should have invalid member access error");
    }

    #[test]
    fn test_contract_field_validation() {
        let input = r#"contract User:
    id: number
    name: string

let's user: User = {
    id: 1,
    name: "John",
    extra_field: "invalid"
}"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_field_error = guardian.errors.iter().any(|error| {
            error.message.contains("field") || error.message.contains("property")
        });
        assert!(has_field_error, "Should have invalid contract field error");
    }

    #[test]
    fn test_missing_contract_field() {
        let input = r#"contract User:
    id: number
    name: string
    email: string

let's user: User = {
    id: 1,
    name: "John"
    # Missing email field
}"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_missing_field_error = guardian.errors.iter().any(|error| {
            error.message.contains("missing") || error.message.contains("required")
        });
        assert!(has_missing_field_error, "Should have missing contract field error");
    }

    #[test]
    fn test_invalid_binary_operation() {
        let input = r#"let's result = "hello" + true"#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_operation_error = guardian.errors.iter().any(|error| {
            error.message.contains("operation") || error.message.contains("operator")
        });
        assert!(has_operation_error, "Should have invalid binary operation error");
    }

    #[test]
    fn test_scope_resolution() {
        let input = r#"let's outer = 10

let's test_function():
    let's inner = 20
    return outer + inner

let's result = inner"#; // inner is not accessible here
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_scope_error = guardian.errors.iter().any(|error| {
            error.message.contains("scope") || error.message.contains("undefined")
        });
        assert!(has_scope_error, "Should have scope resolution error");
    }

    #[test]
    fn test_recursive_function_call() {
        let input = r#"let's factorial(n: number) -> number:
    if n <= 1:
        return 1
    else:
        return n * factorial(n - 1)"#;
        let guardian = analyze_program(input);
        
        // Recursive calls should be valid
        assert!(guardian.errors.is_empty(), "Recursive function should be valid");
    }

    #[test]
    fn test_ui_component_validation() {
        let input = r#"app TestApp:
    show:
        invalid_component:
            text "Hello""#;
        let guardian = analyze_program(input);
        
        assert!(!guardian.errors.is_empty(), "Should have semantic errors");
        
        let has_component_error = guardian.errors.iter().any(|error| {
            error.message.contains("component") || error.message.contains("element")
        });
        assert!(has_component_error, "Should have invalid UI component error");
    }

    #[test]
    fn test_track_variable_in_ui() {
        let input = r#"app TestApp:
    let's track counter = 0
    
    show:
        text "{counter}""#;
        let guardian = analyze_program(input);
        
        // This should be valid - tracked variables can be used in UI
        assert!(guardian.errors.is_empty(), "Tracked variable in UI should be valid");
    }

    #[test]
    fn test_non_tracked_variable_in_reactive_context() {
        let input = r#"app TestApp:
    let's regular_var = "hello"
    
    show:
        text "{regular_var}""#;
        let guardian = analyze_program(input);
        
        // This might generate a warning about non-reactive variables in UI
        // The exact behavior depends on the language design
        // For now, we'll just check that it doesn't crash
        // assert!(!guardian.errors.is_empty(), "Should warn about non-tracked variable in UI");
    }
}
