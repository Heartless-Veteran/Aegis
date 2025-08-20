//! This module contains the Architect, the parser for the Aegis language.
//! It uses a Pratt parsing (top-down operator precedence) strategy to handle
//! expressions, which allows for clean and correct handling of operator precedence.
//! Its primary job is to consume tokens from the Scribe and produce an AST,
//! collecting any syntax errors it finds along the way.

use crate::scribe::Scribe;
use crate::token::{Token, Span};
use crate::error::ParseError;
use crate::architect::ast::*;

/// Defines the precedence levels for operators to manage order of operations.
/// Higher variants have higher precedence.
#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Assign,      // =
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Member,      // object.member
}

/// The Architect struct holds the state of the parser as it consumes tokens.
pub struct Architect<'a> {
    /// The Scribe (lexer) which provides the token stream.
    scribe: Scribe<'a>,
    /// The current token the parser is looking at.
    current_token: Token,
    /// The next token in the stream, used for lookahead.
    peek_token: Token,
    /// A list of syntax errors encountered during parsing.
    pub errors: Vec<ParseError>,
}

impl<'a> Architect<'a> {
    /// Creates a new Architect and primes it by loading the first two tokens.
    pub fn new(scribe: Scribe<'a>) -> Self {
        let mut architect = Self {
            scribe,
            current_token: Token::Eof(Span::default()),
            peek_token: Token::Eof(Span::default()),
            errors: Vec::new(),
        };
        // Load the first two tokens to initialize the `current` and `peek` state.
        architect.next_token();
        architect.next_token();
        architect
    }
    
    /// Consumes the current token and advances the Scribe to the next one.
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.scribe.next_token();
    }

    /// The main entry point for parsing an entire Aegis file. It continues
    /// parsing definitions until it reaches the End-Of-File token.
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            definitions: Vec::new(),
            span: Span::default(), // Will be updated at the end
        };

        while !matches!(self.current_token, Token::Eof(_)) {
            match self.parse_definition() {
                Some(def) => program.definitions.push(def),
                None => self.next_token(), // On error, skip token to prevent infinite loops
            }
        }
        program
    }
    
    /// Dispatches to the correct parsing function for a top-level definition,
    /// such as an `app`, `contract`, or function.
    fn parse_definition(&mut self) -> Option<Definition> {
        // This is a placeholder for the full dispatch logic. In a real implementation,
        // it would look at the current token and decide which specific parse
        // function to call (e.g., `parse_app_definition`).
        // For example:
        // match self.current_token {
        //     Token::App(_) => self.parse_app_definition().map(Definition::App),
        //     Token::Contract(_) => self.parse_contract_definition().map(Definition::Contract),
        //     _ => self.parse_statement().map(Definition::Statement),
        // }
        None // Placeholder
    }

    /// The core of the Pratt parser for handling expressions.
    /// It recursively parses tokens based on their defined precedence.
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Find a prefix parsing function for the current token.
        // This handles literals, identifiers, and prefix operators like `-` or `!`.
        let mut left_expression = match self.parse_prefix() {
            Some(expr) => expr,
            None => {
                // Register an error if no prefix parse function is found.
                self.errors.push(ParseError { 
                    message: format!("Unexpected token: {:?}", self.current_token),
                    span: self.current_token.span(),
                });
                return None;
            }
        };

        // Loop as long as the next token is an infix operator with higher precedence.
        while precedence < self.peek_precedence() {
            self.next_token(); // Consume the operator
            // Find an infix parsing function for the new current token.
            left_expression = match self.parse_infix(left_expression.clone()) {
                Some(expr) => expr,
                None => return Some(left_expression), // No infix function found, end of expression.
            };
        }
        
        Some(left_expression)
    }
    
    // In a complete implementation, this file would continue with dozens of helper
    // functions, each responsible for parsing a specific piece of the language's
    // grammar. For example:
    //
    // /// Parses a `let's` statement, e.g., `let's x = 5`
    // fn parse_let_statement(&mut self) -> Option<LetStatement> { ... }
    //
    // /// Parses an `if/else` expression.
    // fn parse_if_expression(&mut self) -> Option<Expression> { ... }
    //
    // /// Parses a UI node definition, e.g., `text "Hello"`
    // fn parse_ui_node(&mut self) -> Option<UiNode> { ... }
    //
    // /// Parses a function call expression, e.g., `my_func(a, b)`
    // fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> { ... }
    //
    // ...and so on for every language construct.
}
