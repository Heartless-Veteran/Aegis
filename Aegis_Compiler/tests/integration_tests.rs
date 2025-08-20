//! Integration tests for the complete Aegis compiler pipeline

use aegis_compiler::{Scribe, Architect, Guardian};
use crate::test_utils::{TestFixtures, PerformanceTimer};

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test the complete compilation pipeline from source to validated AST
    fn compile_program(input: &str) -> CompilationResult {
        let timer = PerformanceTimer::new();
        
        // Lexical analysis
        let scribe = Scribe::new(input);
        let lexing_time = timer.elapsed_ms();
        
        // Parsing
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        let parsing_time = timer.elapsed_ms() - lexing_time;
        
        // Semantic analysis
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        let semantic_time = timer.elapsed_ms() - parsing_time - lexing_time;
        
        CompilationResult {
            program,
            parse_errors: architect.errors,
            semantic_errors: guardian.errors,
            lexing_time,
            parsing_time,
            semantic_time,
            total_time: timer.elapsed_ms(),
        }
    }

    struct CompilationResult {
        program: aegis_compiler::ast::Program,
        parse_errors: Vec<aegis_compiler::error::ParseError>,
        semantic_errors: Vec<aegis_compiler::error::SemanticError>,
        lexing_time: u128,
        parsing_time: u128,
        semantic_time: u128,
        total_time: u128,
    }

    impl CompilationResult {
        fn is_successful(&self) -> bool {
            self.parse_errors.is_empty() && self.semantic_errors.is_empty()
        }

        fn has_errors(&self) -> bool {
            !self.parse_errors.is_empty() || !self.semantic_errors.is_empty()
        }
    }

    #[test]
    fn test_complete_simple_program() {
        let result = compile_program(TestFixtures::simple_let());
        
        assert!(result.is_successful(), "Simple program should compile successfully");
        assert_eq!(result.program.definitions.len(), 1);
        assert!(result.total_time < 100, "Should compile quickly");
    }

    #[test]
    fn test_complete_function_program() {
        let result = compile_program(TestFixtures::simple_function());
        
        assert!(result.is_successful(), "Function program should compile successfully");
        assert_eq!(result.program.definitions.len(), 1);
    }

    #[test]
    fn test_complete_async_program() {
        let result = compile_program(TestFixtures::async_function());
        
        assert!(result.is_successful(), "Async program should compile successfully");
        assert_eq!(result.program.definitions.len(), 1);
    }

    #[test]
    fn test_complete_contract_program() {
        let result = compile_program(TestFixtures::simple_contract());
        
        assert!(result.is_successful(), "Contract program should compile successfully");
        assert_eq!(result.program.definitions.len(), 1);
    }

    #[test]
    fn test_complete_app_program() {
        let result = compile_program(TestFixtures::simple_app());
        
        assert!(result.is_successful(), "App program should compile successfully");
        assert_eq!(result.program.definitions.len(), 1);
    }

    #[test]
    fn test_complete_complex_program() {
        let result = compile_program(TestFixtures::complex_app());
        
        assert!(result.is_successful(), "Complex program should compile successfully");
        assert_eq!(result.program.definitions.len(), 2); // Contract + App
    }

    #[test]
    fn test_error_recovery_lexical() {
        let input = "let's x = @ invalid";
        let result = compile_program(input);
        
        assert!(result.has_errors(), "Should have compilation errors");
        // The compiler should still produce some AST even with lexical errors
    }

    #[test]
    fn test_error_recovery_syntax() {
        let input = "let's x = 5 + + 3"; // Invalid syntax
        let result = compile_program(input);
        
        assert!(result.has_errors(), "Should have compilation errors");
        assert!(!result.parse_errors.is_empty(), "Should have parse errors");
    }

    #[test]
    fn test_error_recovery_semantic() {
        let input = "let's x = undefined_variable + 5";
        let result = compile_program(input);
        
        assert!(result.has_errors(), "Should have compilation errors");
        assert!(!result.semantic_errors.is_empty(), "Should have semantic errors");
    }

    #[test]
    fn test_multiple_definitions() {
        let input = r#"
contract User:
    id: number
    name: string

let's create_user(id: number, name: string) -> User:
    return { id: id, name: name }

app UserApp:
    let's track current_user: User = create_user(1, "John")
    
    show:
        text "User: {current_user.name}"
"#;
        let result = compile_program(input);
        
        assert!(result.is_successful(), "Multi-definition program should compile successfully");
        assert_eq!(result.program.definitions.len(), 3); // Contract, Function, App
    }

    #[test]
    fn test_nested_expressions() {
        let input = "let's result = ((1 + 2) * 3) - (4 / 2)";
        let result = compile_program(input);
        
        assert!(result.is_successful(), "Nested expressions should compile successfully");
    }

    #[test]
    fn test_function_calls_chain() {
        let input = r#"
let's add(a: number, b: number) -> number:
    return a + b

let's multiply(a: number, b: number) -> number:
    return a * b

let's result = multiply(add(1, 2), add(3, 4))
"#;
        let result = compile_program(input);
        
        assert!(result.is_successful(), "Function call chains should compile successfully");
    }

    #[test]
    fn test_ui_with_events() {
        let input = r#"
app EventApp:
    let's track counter = 0
    
    show:
        column:
            text "Counter: {counter}"
            button "Increment" when_clicked:
                counter = counter + 1
            button "Reset" when_clicked:
                counter = 0
"#;
        let result = compile_program(input);
        
        assert!(result.is_successful(), "UI with events should compile successfully");
    }

    #[test]
    fn test_for_loop_in_ui() {
        let input = r#"
app ListApp:
    let's track items = ["Apple", "Banana", "Cherry"]
    
    show:
        column:
            for item in items:
                text item
"#;
        let result = compile_program(input);
        
        assert!(result.is_successful(), "For loop in UI should compile successfully");
    }

    #[test]
    fn test_conditional_ui() {
        let input = r#"
app ConditionalApp:
    let's track show_message = true
    
    show:
        column:
            if show_message:
                text "Hello, World!"
            else:
                text "Message hidden"
"#;
        let result = compile_program(input);
        
        assert!(result.is_successful(), "Conditional UI should compile successfully");
    }

    #[test]
    fn test_compilation_performance() {
        let large_program = format!("{}\n{}\n{}\n{}", 
            TestFixtures::simple_contract(),
            TestFixtures::simple_function(),
            TestFixtures::async_function(),
            TestFixtures::complex_app()
        );
        
        let result = compile_program(&large_program);
        
        assert!(result.is_successful(), "Large program should compile successfully");
        assert!(result.total_time < 1000, "Should compile within reasonable time (1s)");
        
        println!("Compilation performance:");
        println!("  Lexing: {}ms", result.lexing_time);
        println!("  Parsing: {}ms", result.parsing_time);
        println!("  Semantic: {}ms", result.semantic_time);
        println!("  Total: {}ms", result.total_time);
    }

    #[test]
    fn test_empty_program() {
        let result = compile_program("");
        
        assert!(result.is_successful(), "Empty program should compile successfully");
        assert_eq!(result.program.definitions.len(), 0);
    }

    #[test]
    fn test_comments_only_program() {
        let input = r#"
# This is a comment
# Another comment
# Yet another comment
"#;
        let result = compile_program(input);
        
        assert!(result.is_successful(), "Comments-only program should compile successfully");
        assert_eq!(result.program.definitions.len(), 0);
    }
}
