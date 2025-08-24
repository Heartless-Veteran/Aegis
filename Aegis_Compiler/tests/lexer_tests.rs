//! Lexer tests for the Aegis compiler

use aegis_compiler::{Scribe, Token};

#[test]
fn test_simple_tokens() {
    let input = "let's x = 42";
    let mut scribe = Scribe::new(input);

    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    })
    .collect();

    assert_eq!(tokens.len(), 4); // let's, x, =, 42
    assert!(matches!(tokens[0], Token::Let(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "x"));
    assert!(matches!(tokens[2], Token::Assign(_)));
    assert!(matches!(tokens[3], Token::Number(ref s, _) if s == "42"));
}

#[test]
fn test_keywords() {
    let input = "let's app track when show enum";
    let mut scribe = Scribe::new(input);

    let token1 = scribe.next_token();
    let token2 = scribe.next_token();
    let token3 = scribe.next_token();
    let token4 = scribe.next_token();
    let token5 = scribe.next_token();
    let token6 = scribe.next_token();

    assert!(matches!(token1, Token::Let(_)));
    assert!(matches!(token2, Token::App(_)));
    assert!(matches!(token3, Token::Track(_)));
    assert!(matches!(token4, Token::When(_)));
    assert!(matches!(token5, Token::Show(_)));
    assert!(matches!(token6, Token::Enum(_)));
}

#[test]
fn test_operators() {
    let input = "= == != + - * /";
    let mut scribe = Scribe::new(input);

    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    })
    .collect();

    assert_eq!(tokens.len(), 7);
    assert!(matches!(tokens[0], Token::Assign(_)));
    assert!(matches!(tokens[1], Token::Equals(_)));
    assert!(matches!(tokens[2], Token::NotEquals(_)));
    assert!(matches!(tokens[3], Token::Plus(_)));
    assert!(matches!(tokens[4], Token::Minus(_)));
    assert!(matches!(tokens[5], Token::Asterisk(_)));
    assert!(matches!(tokens[6], Token::Slash(_)));
}

#[test]
fn test_string_literals() {
    let input = r#""hello world""#;
    let mut scribe = Scribe::new(input);

    let token = scribe.next_token();
    assert!(matches!(token, Token::String(ref s, _) if s == "hello world"));
}

#[test]
fn test_numbers() {
    let input = "42 0 999";
    let mut scribe = Scribe::new(input);

    let token1 = scribe.next_token();
    let token2 = scribe.next_token();
    let token3 = scribe.next_token();

    assert!(matches!(token1, Token::Number(ref s, _) if s == "42"));
    assert!(matches!(token2, Token::Number(ref s, _) if s == "0"));
    assert!(matches!(token3, Token::Number(ref s, _) if s == "999"));
}

#[test]
fn test_identifiers() {
    let input = "my_var MyClass _private";
    let mut scribe = Scribe::new(input);

    let token1 = scribe.next_token();
    let token2 = scribe.next_token();
    let token3 = scribe.next_token();

    assert!(matches!(token1, Token::Identifier(ref s, _) if s == "my_var"));
    assert!(matches!(token2, Token::Identifier(ref s, _) if s == "MyClass"));
    assert!(matches!(token3, Token::Identifier(ref s, _) if s == "_private"));
}

#[test]
fn test_delimiters() {
    let input = "( ) { } , :";
    let mut scribe = Scribe::new(input);

    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    })
    .collect();

    assert_eq!(tokens.len(), 6);
    assert!(matches!(tokens[0], Token::LParen(_)));
    assert!(matches!(tokens[1], Token::RParen(_)));
    assert!(matches!(tokens[2], Token::LBrace(_)));
    assert!(matches!(tokens[3], Token::RBrace(_)));
    assert!(matches!(tokens[4], Token::Comma(_)));
    assert!(matches!(tokens[5], Token::Colon(_)));
}

#[test]
fn test_whitespace_handling() {
    let input = "  let's   x   =   42  ";
    let mut scribe = Scribe::new(input);

    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    })
    .collect();

    assert_eq!(tokens.len(), 4); // Whitespace should be ignored
    assert!(matches!(tokens[0], Token::Let(_)));
    assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "x"));
    assert!(matches!(tokens[2], Token::Assign(_)));
    assert!(matches!(tokens[3], Token::Number(ref s, _) if s == "42"));
}

#[test]
fn test_empty_input() {
    let input = "";
    let mut scribe = Scribe::new(input);

    let token = scribe.next_token();
    assert!(matches!(token, Token::Eof(_)));
}

#[test]
fn test_comments() {
    let input = "let's x = 5 # This is a comment\nlet's y = 10";
    let mut scribe = Scribe::new(input);

    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let token = scribe.next_token();
        match token {
            Token::Eof(_) => None,
            _ => Some(token),
        }
    })
    .collect();

    // Should skip the comment and parse both let statements
    assert_eq!(tokens.len(), 8); // let's x = 5 let's y = 10
    assert!(matches!(tokens[0], Token::Let(_)));
    assert!(matches!(tokens[4], Token::Let(_))); // Second let's after comment
}
