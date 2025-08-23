//! Language feature tests for the Aegis compiler

use aegis_compiler::{Scribe, Architect, Guardian, Token};

#[test]
fn test_variable_declaration_tokens() {
    let input = "let's x = 42";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[0], Token::Let(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "x"));
    assert!(matches!(tokens[2], Token::Assign(_)));
    assert!(matches!(tokens[3], Token::Number(ref s, _) if s == "42"));
}

#[test]
fn test_tracked_variable_tokens() {
    let input = "let's track counter = 0";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[0], Token::Let(_)));
    assert!(matches!(tokens[1], Token::Track(_)));
    assert!(matches!(tokens[2], Token::Identifier(ref s, _) if s == "counter"));
    assert!(matches!(tokens[3], Token::Assign(_)));
    assert!(matches!(tokens[4], Token::Number(ref s, _) if s == "0"));
}

#[test]
fn test_function_definition_tokens() {
    let input = "let's add(a: number, b: number) -> number:";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    // Should contain let's, add, (, a, :, number, ,, b, :, number, ), ->, number, :
    assert!(tokens.len() >= 10);
    assert!(matches!(tokens[0], Token::Let(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "add"));
    assert!(matches!(tokens[2], Token::LParen(_)));
}

#[test]
fn test_async_function_tokens() {
    let input = "async let's fetch_data():";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert!(tokens.len() >= 5);
    assert!(matches!(tokens[0], Token::Async(_)));
    assert!(matches!(tokens[1], Token::Let(_)));
    assert!(matches!(tokens[2], Token::Identifier(ref s, _) if s == "fetch_data"));
    assert!(matches!(tokens[3], Token::LParen(_)));
    assert!(matches!(tokens[4], Token::RParen(_)));
}

#[test]
fn test_contract_definition_tokens() {
    let input = "contract User:";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0], Token::Contract(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "User"));
    assert!(matches!(tokens[2], Token::Colon(_)));
}

#[test]
fn test_app_definition_tokens() {
    let input = "app MyApp:";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0], Token::App(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "MyApp"));
    assert!(matches!(tokens[2], Token::Colon(_)));
}

#[test]
fn test_expression_tokens() {
    let test_cases = vec![
        ("1 + 2", vec!["1", "+", "2"]),
        ("x == y", vec!["x", "==", "y"]),
        ("!true", vec!["!", "true"]),
        ("-42", vec!["-", "42"]),
    ];
    
    for (input, expected_count) in test_cases {
        let mut scribe = Scribe::new(input);
        let tokens: Vec<Token> = std::iter::from_fn(|| {
            let token = scribe.next_token();
            match token {
                Token::Eof(_) => None,
                _ => Some(token),
            }
        }).collect();
        
        assert_eq!(tokens.len(), expected_count.len(), "Failed for input: {}", input);
    }
}

#[test]
fn test_control_flow_tokens() {
    let input = "if condition: result else: other";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    // Should contain if, condition, :, result, else, :, other
    assert!(tokens.len() >= 7);
    assert!(matches!(tokens[0], Token::If(_)));
    assert!(matches!(tokens[2], Token::Colon(_)));
    
    // Find else token
    let has_else = tokens.iter().any(|token| matches!(token, Token::Else(_)));
    assert!(has_else);
}

#[test]
fn test_for_loop_tokens() {
    let input = "for item in items:";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[0], Token::For(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "item"));
    assert!(matches!(tokens[2], Token::In(_)));
    assert!(matches!(tokens[3], Token::Identifier(ref s, _) if s == "items"));
    assert!(matches!(tokens[4], Token::Colon(_)));
}

#[test]
fn test_when_expression_tokens() {
    let input = "when value is 1:";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[0], Token::When(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "value"));
    assert!(matches!(tokens[2], Token::Is(_)));
    assert!(matches!(tokens[3], Token::Number(ref s, _) if s == "1"));
    assert!(matches!(tokens[4], Token::Colon(_)));
}

#[test]
fn test_ui_show_tokens() {
    let input = "show:";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0], Token::Show(_)));
    assert!(matches!(tokens[1], Token::Colon(_)));
}

#[test]
fn test_enum_definition_tokens() {
    let input = "enum Status:";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0], Token::Enum(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "Status"));
    assert!(matches!(tokens[2], Token::Colon(_)));
}

#[test]
fn test_enum_with_variants_tokens() {
    let input = "enum Result: Success Failed";
    let mut scribe = Scribe::new(input);
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    }).collect();
    
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[0], Token::Enum(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "Result"));
    assert!(matches!(tokens[2], Token::Colon(_)));
    assert!(matches!(tokens[3], Token::Identifier(ref s, _) if s == "Success"));
    assert!(matches!(tokens[4], Token::Identifier(ref s, _) if s == "Failed"));
}

#[test]
fn test_complete_language_constructs() {
    // Test that all major language constructs can be tokenized
    let constructs = vec![
        "let's x = 42",
        "let's track counter = 0",
        "async let's fetch():",
        "contract User:",
        "enum Status:",
        "app MyApp:",
        "if condition:",
        "for item in items:",
        "when value is 1:",
        "show:",
        "return result",
        "await call()",
        "true",
        "false",
        "nothing",
    ];
    
    for construct in constructs {
        let scribe = Scribe::new(construct);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        
        // Should not crash on any language construct
        // Stub implementations should handle all gracefully
        assert!(architect.errors.is_empty());
        assert!(guardian.errors.is_empty());
    }
}