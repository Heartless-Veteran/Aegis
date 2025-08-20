//! Comprehensive lexer (Scribe) tests for the Aegis compiler

use aegis_compiler::{Scribe, token::{Token, Span}};
use crate::test_utils::{TestFixtures, tokenize_all, test_span};

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let input = "let's app track when show contract for in is return true false if else async await nothing";
        let mut scribe = Scribe::new(input);
        
        let expected_tokens = vec![
            Token::Let(test_span(0, 5)),
            Token::App(test_span(6, 9)),
            Token::Track(test_span(10, 15)),
            Token::When(test_span(16, 20)),
            Token::Show(test_span(21, 25)),
            Token::Contract(test_span(26, 34)),
            Token::For(test_span(35, 38)),
            Token::In(test_span(39, 41)),
            Token::Is(test_span(42, 44)),
            Token::Return(test_span(45, 51)),
            Token::True(test_span(52, 56)),
            Token::False(test_span(57, 62)),
            Token::If(test_span(63, 65)),
            Token::Else(test_span(66, 70)),
            Token::Async(test_span(71, 76)),
            Token::Await(test_span(77, 82)),
            Token::Nothing(test_span(83, 90)),
            Token::Eof(test_span(90, 90)),
        ];
        
        for expected in expected_tokens {
            let received = scribe.next_token();
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_operators() {
        let input = "= == != + - * / < > ! . =>";
        let mut scribe = Scribe::new(input);
        
        let expected_tokens = vec![
            Token::Assign(test_span(0, 1)),
            Token::Equals(test_span(2, 4)),
            Token::NotEquals(test_span(5, 7)),
            Token::Plus(test_span(8, 9)),
            Token::Minus(test_span(10, 11)),
            Token::Asterisk(test_span(12, 13)),
            Token::Slash(test_span(14, 15)),
            Token::LessThan(test_span(16, 17)),
            Token::GreaterThan(test_span(18, 19)),
            Token::Bang(test_span(20, 21)),
            Token::Dot(test_span(22, 23)),
            Token::FatArrow(test_span(24, 26)),
            Token::Eof(test_span(26, 26)),
        ];
        
        for expected in expected_tokens {
            let received = scribe.next_token();
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_delimiters() {
        let input = "( ) { } , :";
        let mut scribe = Scribe::new(input);
        
        let expected_tokens = vec![
            Token::LParen(test_span(0, 1)),
            Token::RParen(test_span(2, 3)),
            Token::LBrace(test_span(4, 5)),
            Token::RBrace(test_span(6, 7)),
            Token::Comma(test_span(8, 9)),
            Token::Colon(test_span(10, 11)),
            Token::Eof(test_span(11, 11)),
        ];
        
        for expected in expected_tokens {
            let received = scribe.next_token();
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_identifiers() {
        let input = "my_var MyClass _private variable123 user'";
        let mut scribe = Scribe::new(input);
        
        let expected_tokens = vec![
            Token::Identifier("my_var".to_string(), test_span(0, 6)),
            Token::Identifier("MyClass".to_string(), test_span(7, 14)),
            Token::Identifier("_private".to_string(), test_span(15, 23)),
            Token::Identifier("variable123".to_string(), test_span(24, 35)),
            Token::Identifier("user'".to_string(), test_span(36, 41)),
            Token::Eof(test_span(41, 41)),
        ];
        
        for expected in expected_tokens {
            let received = scribe.next_token();
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_numbers() {
        let input = "42 0 999 123456";
        let mut scribe = Scribe::new(input);
        
        let expected_tokens = vec![
            Token::Number("42".to_string(), test_span(0, 2)),
            Token::Number("0".to_string(), test_span(3, 4)),
            Token::Number("999".to_string(), test_span(5, 8)),
            Token::Number("123456".to_string(), test_span(9, 15)),
            Token::Eof(test_span(15, 15)),
        ];
        
        for expected in expected_tokens {
            let received = scribe.next_token();
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_strings() {
        let input = r#""hello" "world with spaces" "special chars: !@#$%""#;
        let mut scribe = Scribe::new(input);
        
        let expected_tokens = vec![
            Token::String("hello".to_string(), test_span(0, 7)),
            Token::String("world with spaces".to_string(), test_span(8, 27)),
            Token::String("special chars: !@#$%".to_string(), test_span(28, 50)),
            Token::Eof(test_span(50, 50)),
        ];
        
        for expected in expected_tokens {
            let received = scribe.next_token();
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_unterminated_string() {
        let input = r#""unterminated string"#;
        let mut scribe = Scribe::new(input);
        
        let token = scribe.next_token();
        assert!(matches!(token, Token::Illegal('"', _)));
    }

    #[test]
    fn test_comments() {
        let input = r#"let's x = 5 # This is a comment
let's y = 10"#;
        let mut scribe = Scribe::new(input);
        
        let expected_tokens = vec![
            Token::Let(test_span(0, 5)),
            Token::Identifier("x".to_string(), test_span(6, 7)),
            Token::Assign(test_span(8, 9)),
            Token::Number("5".to_string(), test_span(10, 11)),
            Token::Let(test_span(33, 38)),
            Token::Identifier("y".to_string(), test_span(39, 40)),
            Token::Assign(test_span(41, 42)),
            Token::Number("10".to_string(), test_span(43, 45)),
            Token::Eof(test_span(45, 45)),
        ];
        
        for expected in expected_tokens {
            let received = scribe.next_token();
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "  let's   x   =   42  ";
        let mut scribe = Scribe::new(input);
        
        let expected_tokens = vec![
            Token::Let(test_span(2, 7)),
            Token::Identifier("x".to_string(), test_span(10, 11)),
            Token::Assign(test_span(14, 15)),
            Token::Number("42".to_string(), test_span(18, 20)),
            Token::Eof(test_span(22, 22)),
        ];
        
        for expected in expected_tokens {
            let received = scribe.next_token();
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_illegal_characters() {
        let input = "let's x = @ # comment";
        let mut scribe = Scribe::new(input);
        
        let tokens = vec![
            scribe.next_token(), // let's
            scribe.next_token(), // x
            scribe.next_token(), // =
            scribe.next_token(), // @
        ];
        
        assert!(matches!(tokens[3], Token::Illegal('@', _)));
    }

    #[test]
    fn test_simple_let_statement() {
        let tokens = tokenize_all(TestFixtures::simple_let());
        
        assert_eq!(tokens.len(), 5); // let's, x, =, 42, EOF
        assert!(matches!(tokens[0], Token::Let(_)));
        assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "x"));
        assert!(matches!(tokens[2], Token::Assign(_)));
        assert!(matches!(tokens[3], Token::Number(ref s, _) if s == "42"));
        assert!(matches!(tokens[4], Token::Eof(_)));
    }

    #[test]
    fn test_tracked_variable() {
        let tokens = tokenize_all(TestFixtures::tracked_variable());
        
        assert_eq!(tokens.len(), 6); // let's, track, counter, =, 0, EOF
        assert!(matches!(tokens[0], Token::Let(_)));
        assert!(matches!(tokens[1], Token::Track(_)));
        assert!(matches!(tokens[2], Token::Identifier(ref s, _) if s == "counter"));
        assert!(matches!(tokens[3], Token::Assign(_)));
        assert!(matches!(tokens[4], Token::Number(ref s, _) if s == "0"));
        assert!(matches!(tokens[5], Token::Eof(_)));
    }

    #[test]
    fn test_function_definition() {
        let tokens = tokenize_all(TestFixtures::simple_function());
        
        // Should contain: let's, add, (, a, :, number, ,, b, :, number, ), ->, number, :, return, a, +, b
        assert!(tokens.len() > 10);
        assert!(matches!(tokens[0], Token::Let(_)));
        assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "add"));
        assert!(matches!(tokens[2], Token::LParen(_)));
    }

    #[test]
    fn test_async_function() {
        let tokens = tokenize_all(TestFixtures::async_function());
        
        assert!(matches!(tokens[0], Token::Async(_)));
        assert!(matches!(tokens[1], Token::Let(_)));
        
        // Should contain await keyword
        let has_await = tokens.iter().any(|token| matches!(token, Token::Await(_)));
        assert!(has_await);
    }

    #[test]
    fn test_contract_definition() {
        let tokens = tokenize_all(TestFixtures::simple_contract());
        
        assert!(matches!(tokens[0], Token::Contract(_)));
        assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "User"));
        assert!(matches!(tokens[2], Token::Colon(_)));
        
        // Should contain field names and types
        let has_id = tokens.iter().any(|token| matches!(token, Token::Identifier(ref s, _) if s == "id"));
        let has_name = tokens.iter().any(|token| matches!(token, Token::Identifier(ref s, _) if s == "name"));
        let has_active = tokens.iter().any(|token| matches!(token, Token::Identifier(ref s, _) if s == "active"));
        
        assert!(has_id);
        assert!(has_name);
        assert!(has_active);
    }

    #[test]
    fn test_app_definition() {
        let tokens = tokenize_all(TestFixtures::simple_app());
        
        assert!(matches!(tokens[0], Token::App(_)));
        assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "MyApp"));
        
        // Should contain show keyword
        let has_show = tokens.iter().any(|token| matches!(token, Token::Show(_)));
        assert!(has_show);
        
        // Should contain track keyword
        let has_track = tokens.iter().any(|token| matches!(token, Token::Track(_)));
        assert!(has_track);
    }

    #[test]
    fn test_complex_app() {
        let tokens = tokenize_all(TestFixtures::complex_app());
        
        // Should contain all major keywords
        let has_contract = tokens.iter().any(|token| matches!(token, Token::Contract(_)));
        let has_app = tokens.iter().any(|token| matches!(token, Token::App(_)));
        let has_for = tokens.iter().any(|token| matches!(token, Token::For(_)));
        let has_in = tokens.iter().any(|token| matches!(token, Token::In(_)));
        let has_show = tokens.iter().any(|token| matches!(token, Token::Show(_)));
        
        assert!(has_contract);
        assert!(has_app);
        assert!(has_for);
        assert!(has_in);
        assert!(has_show);
    }

    #[test]
    fn test_token_spans() {
        let input = "let's x = 42";
        let mut scribe = Scribe::new(input);
        
        let token1 = scribe.next_token();
        let token2 = scribe.next_token();
        let token3 = scribe.next_token();
        let token4 = scribe.next_token();
        
        assert_eq!(token1.span(), Span { start: 0, end: 5 });
        assert_eq!(token2.span(), Span { start: 6, end: 7 });
        assert_eq!(token3.span(), Span { start: 8, end: 9 });
        assert_eq!(token4.span(), Span { start: 10, end: 12 });
    }

    #[test]
    fn test_empty_input() {
        let mut scribe = Scribe::new("");
        let token = scribe.next_token();
        assert!(matches!(token, Token::Eof(_)));
    }

    #[test]
    fn test_only_whitespace() {
        let mut scribe = Scribe::new("   \t\n\r   ");
        let token = scribe.next_token();
        assert!(matches!(token, Token::Eof(_)));
    }

    #[test]
    fn test_only_comments() {
        let mut scribe = Scribe::new("# This is a comment\n# Another comment");
        let token = scribe.next_token();
        assert!(matches!(token, Token::Eof(_)));
    }

    #[test]
    fn test_mixed_line_endings() {
        let input = "let's\nx\r\n=\r42";
        let tokens = tokenize_all(input);
        
        assert_eq!(tokens.len(), 5); // let's, x, =, 42, EOF
        assert!(matches!(tokens[0], Token::Let(_)));
        assert!(matches!(tokens[1], Token::Identifier(ref s, _) if s == "x"));
        assert!(matches!(tokens[2], Token::Assign(_)));
        assert!(matches!(tokens[3], Token::Number(ref s, _) if s == "42"));
        assert!(matches!(tokens[4], Token::Eof(_)));
    }

    #[test]
    fn test_literal_string_method() {
        let identifier = Token::Identifier("test_var".to_string(), test_span(0, 8));
        let number = Token::Number("42".to_string(), test_span(0, 2));
        let keyword = Token::Let(test_span(0, 5));
        
        assert_eq!(identifier.literal_string(), "test_var");
        assert_eq!(number.literal_string(), "");
        assert_eq!(keyword.literal_string(), "");
    }
}