use crate::scribe::Scribe;
use crate::token::{Token, Span};

#[test]
fn test_let_statement() {
    let input = "let's five = 5";
    let mut scribe = Scribe::new(input);
    let tokens = vec![
        Token::Let(Span { start: 0, end: 5 }),
        Token::Identifier("five".to_string(), Span { start: 6, end: 10 }),
        Token::Assign(Span { start: 11, end: 12 }),
        Token::Number("5".to_string(), Span { start: 13, end: 14 }),
        Token::Eof(Span { start: 14, end: 14 }),
    ];
    for expected in tokens {
        let received = scribe.next_token();
        assert_eq!(received, expected);
    }
}

#[test]
fn test_all_tokens() {
    let input = r#"
        let's x = 10;
        app MyApp:
            for item in items:
                when item.type == "button":
                    # Do something
    "#;
    // A real test would have a long vector of expected tokens to assert against.
    // This just ensures it doesn't crash.
    let mut scribe = Scribe::new(input);
    loop {
        let token = scribe.next_token();
        if let Token::Eof(_) = token {
            break;
        }
    }
}
