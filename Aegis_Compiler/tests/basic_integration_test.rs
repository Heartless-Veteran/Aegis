//! Basic integration test for the Aegis compiler

use aegis_compiler::{Architect, Guardian, Scribe};

#[test]
fn test_basic_lexer_functionality() {
    let input = "let's x = 42";
    let mut scribe = Scribe::new(input);

    let token1 = scribe.next_token();
    let token2 = scribe.next_token();
    let token3 = scribe.next_token();
    let token4 = scribe.next_token();
    let token5 = scribe.next_token();

    // Should tokenize correctly
    assert!(matches!(token1, aegis_compiler::Token::Let(_)));
    assert!(matches!(token2, aegis_compiler::Token::Identifier(ref s, _) if s == "x"));
    assert!(matches!(token3, aegis_compiler::Token::Assign(_)));
    assert!(matches!(token4, aegis_compiler::Token::Number(ref s, _) if s == "42"));
    assert!(matches!(token5, aegis_compiler::Token::Eof(_)));
}

#[test]
fn test_basic_parser_functionality() {
    let input = "let's x = 42";
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);

    let program = architect.parse_program();

    // Should parse without errors and now actually parse the let statement
    assert!(architect.errors.is_empty());
    assert_eq!(program.definitions.len(), 1); // Real implementation now parses let statements
}

#[test]
fn test_basic_semantic_analysis() {
    let input = "let's x = 42";
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();

    let mut guardian = Guardian::new();
    guardian.check_program(&program);

    // Should analyze without errors (stub implementation)
    assert!(guardian.errors.is_empty());
}
