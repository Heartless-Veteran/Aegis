//! Error handling and recovery tests for the Aegis compiler

use aegis_compiler::{Scribe, Architect, Guardian};

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    fn compile_and_get_errors(input: &str) -> (Vec<String>, Vec<String>) {
        let scribe = Scribe::new(input);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        
        let parse_errors: Vec<String> = architect.errors.into_iter()
            .map(|e| e.message)
            .collect();
        
        let semantic_errors: Vec<String> = guardian.errors.into_iter()
            .map(|e| e.message)
            .collect();
        
        (parse_errors, semantic_errors)
    }

    #[test]
    fn test_lexical_error_illegal_character() {
        let input = "let's x = @ + 5";
        let (parse_errors, _) = compile_and_get_errors(input);
        
        assert!(!parse_errors.is_empty(), "Should have parse errors for illegal character");
        
        let has_illegal_char_error = parse_errors.iter().any(|err| 
            err.contains("illegal") || err.contains("unexpected")
        );
        assert!(has_illegal_char_error, "Should report illegal character error");
    }

    #[test]
    fn test_lexical_error_unterminated_string() {
        let input = r#"let's message = "unterminated string"#;
        let (parse_errors, _) = compile_and_get_errors(input);
        
        assert!(!parse_errors.is_empty(), "Should have parse errors for unterminated string");
        
        let has_string_error = parse_errors.iter().any(|err| 
            err.contains("string") || err.contains("unterminated")
        );
        assert!(has_string_error, "Should report unterminated string error");
    }

    #[test]
    fn test_syntax_error_missing_assignment() {
        let input = "let's x 42";
        let (parse_errors, _) = compile_and_get_errors(input);
        
        assert!(!parse_errors.is_empty(), "Should have parse errors for missing assignment");
        
        let has_assignment_error = parse_errors.iter().any(|err| 
            err.contains("expected") && (err.contains("=") || err.contains("assignment"))
        );
        assert!(has_assignment_error, "Should report missing assignment error");
    }

    #[test]
    fn test_syntax_error_missing_colon() {
        let input = "let's add(a: number, b: number) -> number return a + b";
        let (parse_errors, _) = compile_and_get_errors(input);
        
        assert!(!parse_errors.is_empty(), "Should have parse errors for missing colon");
        
        let has_colon_error = parse_errors.iter().any(|err| 
            err.contains("expected") && err.contains(":")
        );
        assert!(has_colon_error, "Should report missing colon error");
    }

    #[test]
    fn test_syntax_error_unmatched_parentheses() {
        let input = "let's result = add(1, 2";
        let (parse_errors, _) = compile_and_get_errors(input);
        
        assert!(!parse_errors.is_empty(), "Should have parse errors for unmatched parentheses");
        
        let has_paren_error = parse_errors.iter().any(|err| 
            err.contains("parenthes") || err.contains(")")
        );
        assert!(has_paren_error, "Should report unmatched parentheses error");
    }

    #[test]
    fn test_syntax_error_unmatched_braces() {
        let input = r#"app MyApp:
    let's x = 5
    show:
        column:
            text "hello"
        # Missing closing brace"#;
        let (parse_errors, _) = compile_and_get_errors(input);
        
        assert!(!parse_errors.is_empty(), "Should have parse errors for unmatched braces");
    }

    #[test]
    fn test_semantic_error_undeclared_variable() {
        let input = "let's result = undefined_var + 5";
        let (_, semantic_errors) = compile_and_get_errors(input);
        
        assert!(!semantic_errors.is_empty(), "Should have semantic errors for undeclared variable");
        
        let has_undeclared_error = semantic_errors.iter().any(|err| 
            err.contains("undefined") || err.contains("undeclared") || err.contains("not found")
        );
        assert!(has_undeclared_error, "Should report undeclared variable error");
    }

    #[test]
    fn test_semantic_error_type_mismatch() {
        let input = r#"let's x: number = "string""#;
        let (_, semantic_errors) = compile_and_get_errors(input);
        
        assert!(!semantic_errors.is_empty(), "Should have semantic errors for type mismatch");
        
        let has_type_error = semantic_errors.iter().any(|err| 
            err.contains("type") && (err.contains("mismatch") || err.contains("expected"))
        );
        assert!(has_type_error, "Should report type mismatch error");
    }

    #[test]
    fn test_semantic_error_function_arity() {
        let input = r#"let's add(a: number, b: number) -> number:
    return a + b

let's result = add(1)"#;
        let (_, semantic_errors) = compile_and_get_errors(input);
        
        assert!(!semantic_errors.is_empty(), "Should have semantic errors for wrong arity");
        
        let has_arity_error = semantic_errors.iter().any(|err| 
            err.contains("argument") || err.contains("parameter") || err.contains("arity")
        );
        assert!(has_arity_error, "Should report function arity error");
    }

    #[test]
    fn test_semantic_error_return_type_mismatch() {
        let input = r#"let's get_number() -> number:
    return "not a number""#;
        let (_, semantic_errors) = compile_and_get_errors(input);
        
        assert!(!semantic_errors.is_empty(), "Should have semantic errors for return type mismatch");
        
        let has_return_error = semantic_errors.iter().any(|err| 
            err.contains("return") && err.contains("type")
        );
        assert!(has_return_error, "Should report return type mismatch error");
    }

    #[test]
    fn test_semantic_error_await_outside_async() {
        let input = r#"let's regular_function():
    let's result = await some_call()
    return result"#;
        let (_, semantic_errors) = compile_and_get_errors(input);
        
        assert!(!semantic_errors.is_empty(), "Should have semantic errors for await outside async");
        
        let has_await_error = semantic_errors.iter().any(|err| 
            err.contains("await") && err.contains("async")
        );
        assert!(has_await_error, "Should report await outside async error");
    }

    #[test]
    fn test_semantic_error_duplicate_declaration() {
        let input = r#"let's x = 5
let's x = 10"#;
        let (_, semantic_errors) = compile_and_get_errors(input);
        
        assert!(!semantic_errors.is_empty(), "Should have semantic errors for duplicate declaration");
        
        let has_duplicate_error = semantic_errors.iter().any(|err| 
            err.contains("duplicate") || err.contains("already declared") || err.contains("redefinition")
        );
        assert!(has_duplicate_error, "Should report duplicate declaration error");
    }

    #[test]
    fn test_error_recovery_continues_parsing() {
        let input = r#"let's x = @  # Illegal character
let's y = 42  # This should still be parsed"#;
        let (parse_errors, _) = compile_and_get_errors(input);
        
        // Should have errors but continue parsing
        assert!(!parse_errors.is_empty(), "Should have parse errors");
        
        // The parser should recover and continue parsing the rest
        // This is implementation-dependent, but ideally it should not crash
    }

    #[test]
    fn test_multiple_errors_reported() {
        let input = r#"let's x = @  # Error 1: illegal character
let's y  # Error 2: missing assignment
let's z: number = "string"  # Error 3: type mismatch"#;
        let (parse_errors, semantic_errors) = compile_and_get_errors(input);
        
        let total_errors = parse_errors.len() + semantic_errors.len();
        assert!(total_errors >= 2, "Should report multiple errors, got {}", total_errors);
    }

    #[test]
    fn test_error_spans_accuracy() {
        let input = "let's x = undefined_variable";
        let scribe = Scribe::new(input);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        
        if !guardian.errors.is_empty() {
            let error = &guardian.errors[0];
            // The error span should point to the undefined variable
            let error_text = &input[error.span.start..error.span.end];
            assert_eq!(error_text, "undefined_variable", "Error span should point to the problematic token");
        }
    }

    #[test]
    fn test_cascading_errors_limited() {
        let input = r#"let's undefined_func() -> SomeUndefinedType:
    return undefined_var + another_undefined"#;
        let (_, semantic_errors) = compile_and_get_errors(input);
        
        // Should report errors but not an excessive number due to cascading
        assert!(!semantic_errors.is_empty(), "Should have semantic errors");
        assert!(semantic_errors.len() < 10, "Should not have excessive cascading errors");
    }

    #[test]
    fn test_error_messages_helpful() {
        let test_cases = vec![
            ("let's x = @", "illegal character"),
            ("let's x", "missing assignment"),
            ("let's result = undefined_var", "undefined variable"),
            (r#"let's x: number = "string""#, "type mismatch"),
        ];
        
        for (input, expected_error_type) in test_cases {
            let (parse_errors, semantic_errors) = compile_and_get_errors(input);
            let all_errors: Vec<String> = parse_errors.into_iter()
                .chain(semantic_errors.into_iter())
                .collect();
            
            assert!(!all_errors.is_empty(), "Should have errors for: {}", input);
            
            let has_expected_error = all_errors.iter().any(|err| 
                err.to_lowercase().contains(&expected_error_type.to_lowercase())
            );
            assert!(has_expected_error, 
                "Should have {} error for input: {}\nActual errors: {:?}", 
                expected_error_type, input, all_errors);
        }
    }

    #[test]
    fn test_error_recovery_in_functions() {
        let input = r#"let's broken_func(a: number, b: @) -> number:  # Error in parameter
    return a + b

let's good_func(x: number) -> number:  # This should still be parsed
    return x * 2"#;
        let (parse_errors, _) = compile_and_get_errors(input);
        
        assert!(!parse_errors.is_empty(), "Should have parse errors");
        // The parser should recover and parse the second function
    }

    #[test]
    fn test_error_recovery_in_apps() {
        let input = r#"app BrokenApp:
    let's x = @  # Error here
    
    show:
        text "This should still be parsed"

app GoodApp:  # This should still be parsed
    show:
        text "Hello""#;
        let (parse_errors, _) = compile_and_get_errors(input);
        
        assert!(!parse_errors.is_empty(), "Should have parse errors");
        // The parser should recover and parse the second app
    }

    #[test]
    fn test_no_crash_on_malformed_input() {
        let malformed_inputs = vec![
            "",
            "@@@@@@",
            "let's",
            "(((((",
            ")))))",
            "\"\"\"\"\"",
            "# Only comments",
            "let's x = 1 + + + 2",
            "app MyApp: show: column: text",
        ];
        
        for input in malformed_inputs {
            // Should not crash, even with malformed input
            let _ = compile_and_get_errors(input);
        }
    }

    #[test]
    fn test_error_limits() {
        // Generate input with many errors
        let mut input = String::new();
        for i in 0..100 {
            input.push_str(&format!("let's var_{} = @\n", i));
        }
        
        let (parse_errors, semantic_errors) = compile_and_get_errors(&input);
        let total_errors = parse_errors.len() + semantic_errors.len();
        
        // Should have errors but not necessarily report all 100
        // (compilers often limit error reporting to avoid overwhelming users)
        assert!(total_errors > 0, "Should have some errors");
        assert!(total_errors <= 50, "Should limit error reporting to reasonable number");
    }
}
