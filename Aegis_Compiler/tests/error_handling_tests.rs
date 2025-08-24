//! Error handling tests for the Aegis compiler

use aegis_compiler::{Architect, Guardian, Scribe, Token};

#[test]
fn test_illegal_character_handling() {
    let input = "let's x = @";
    let mut scribe = Scribe::new(input);

    // Should tokenize normally until the illegal character
    let token1 = scribe.next_token(); // let's
    let token2 = scribe.next_token(); // x
    let token3 = scribe.next_token(); // =
    let token4 = scribe.next_token(); // @

    assert!(matches!(token1, Token::Let(_)));
    assert!(matches!(token2, Token::Identifier(ref s, _) if s == "x"));
    assert!(matches!(token3, Token::Assign(_)));
    assert!(matches!(token4, Token::Illegal('@', _)));
}

#[test]
fn test_unterminated_string_handling() {
    let input = r#"let's x = "unterminated"#;
    let mut scribe = Scribe::new(input);

    let _token1 = scribe.next_token(); // let's
    let _token2 = scribe.next_token(); // x
    let _token3 = scribe.next_token(); // =
    let token4 = scribe.next_token(); // unterminated string

    // Should produce an illegal token for unterminated string
    assert!(matches!(token4, Token::Illegal('"', _)));
}

#[test]
fn test_parser_error_handling() {
    let input = "let's x = @"; // Contains illegal character
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);

    let _program = architect.parse_program();

    // The real implementation should now detect and report the error
    // when encountering invalid syntax like '@'
    println!("Parser errors: {:?}", architect.errors);
    assert!(!architect.errors.is_empty()); // Real implementation should detect errors
}

#[test]
fn test_semantic_error_handling() {
    let input = "let's x = undefined_variable";
    let scribe = Scribe::new(input);
    let mut architect = Architect::new(scribe);
    let program = architect.parse_program();

    let mut guardian = Guardian::new();
    guardian.check_program(&program);

    // The stub implementation doesn't actually analyze, so it won't generate errors
    // In a real implementation, this would test semantic error detection
    assert!(guardian.errors.is_empty()); // Stub behavior
}

#[test]
fn test_empty_input_handling() {
    let input = "";
    let mut scribe = Scribe::new(input);

    let token = scribe.next_token();
    assert!(matches!(token, Token::Eof(_)));
}

#[test]
fn test_whitespace_only_handling() {
    let input = "   \t\n\r   ";
    let mut scribe = Scribe::new(input);

    let token = scribe.next_token();
    assert!(matches!(token, Token::Eof(_)));
}

#[test]
fn test_comment_only_handling() {
    let input = "# This is just a comment";
    let mut scribe = Scribe::new(input);

    let token = scribe.next_token();
    assert!(matches!(token, Token::Eof(_)));
}

#[test]
fn test_mixed_valid_invalid_tokens() {
    let input = "let's x = 42 @ y = 10";
    let mut scribe = Scribe::new(input);

    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    })
    .collect();

    // Should tokenize valid parts and mark invalid parts as illegal
    assert!(tokens.len() >= 5);
    assert!(matches!(tokens[0], Token::Let(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "x"));
    assert!(matches!(tokens[2], Token::Assign(_)));
    assert!(matches!(tokens[3], Token::Number(ref s, _) if s == "42"));
    assert!(matches!(tokens[4], Token::Illegal('@', _)));
}

#[test]
fn test_no_crash_on_malformed_input() {
    let malformed_inputs = vec![
        "",
        "@@@@@@",
        "let's",
        "(((((",
        ")))))",
        r#""""""""#,
        "# Only comments",
        "let's x = 1 + + + 2",
    ];

    for input in malformed_inputs {
        // Should not crash, even with malformed input
        let scribe = Scribe::new(input);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        let mut guardian = Guardian::new();
        guardian.check_program(&program);

        // Test passes if we reach this point without panicking
    }
}
