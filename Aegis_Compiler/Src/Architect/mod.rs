//! This module contains the Architect, the parser for the Aegis language.
//! It uses a Pratt parsing (top-down operator precedence) strategy to handle
//! expressions, which allows for clean and correct handling of operator precedence.
//! Its primary job is to consume tokens from the Scribe and produce an AST,
//! collecting any syntax errors it finds along the way.

use crate::ast::*;
use crate::token::{Token, Span};
use crate::error::ParseError;
use crate::Scribe;

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
        match &self.current_token {
            Token::Contract(_) => self.parse_contract_definition().map(Definition::Contract),
            Token::Let(_) => self.parse_let_statement().map(Definition::Statement),
            Token::App(_) => self.parse_app_definition().map(Definition::App),
            Token::Enum(_) => self.parse_enum_definition().map(Definition::Enum),
            _ => {
                // For now, skip unknown tokens to prevent infinite loops
                self.next_token();
                None
            }
        }
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
    
    /// Parse a contract definition
    fn parse_contract_definition(&mut self) -> Option<ContractDefinition> {
        let start_span = self.current_token.span();
        
        // Consume 'contract' token
        if !matches!(self.current_token, Token::Contract(_)) {
            return None;
        }
        self.next_token();
        
        // Get contract name
        let name = if let Token::Identifier(name, _) = &self.current_token {
            let contract_name = name.clone();
            self.next_token();
            contract_name
        } else {
            self.errors.push(ParseError {
                message: "Expected contract name".to_string(),
                span: self.current_token.span(),
            });
            return None;
        };
        
        // Parse generic parameters if present: <T>, <T, U>, etc.
        let mut generic_params = Vec::new();
        if matches!(self.current_token, Token::LessThan(_)) {
            self.next_token(); // consume '<'
            
            // Parse first parameter
            if let Token::Identifier(param_name, _) = &self.current_token {
                generic_params.push(param_name.clone());
                self.next_token();
                
                // Parse additional parameters separated by commas
                while matches!(self.current_token, Token::Comma(_)) {
                    self.next_token(); // consume ','
                    
                    if let Token::Identifier(param_name, _) = &self.current_token {
                        generic_params.push(param_name.clone());
                        self.next_token();
                    } else {
                        self.errors.push(ParseError {
                            message: "Expected generic parameter name after comma".to_string(),
                            span: self.current_token.span(),
                        });
                        return None;
                    }
                }
                
                // Expect closing '>'
                if !matches!(self.current_token, Token::GreaterThan(_)) {
                    self.errors.push(ParseError {
                        message: "Expected '>' to close generic parameters".to_string(),
                        span: self.current_token.span(),
                    });
                    return None;
                }
                self.next_token(); // consume '>'
            } else {
                self.errors.push(ParseError {
                    message: "Expected generic parameter name after '<'".to_string(),
                    span: self.current_token.span(),
                });
                return None;
            }
        }
        
        // Expect colon
        if !matches!(self.current_token, Token::Colon(_)) {
            self.errors.push(ParseError {
                message: "Expected ':' after contract name".to_string(),
                span: self.current_token.span(),
            });
            return None;
        }
        self.next_token();
        
        // Parse fields (simplified - just parse lines with "name: type" format)
        let mut fields = Vec::new();
        while !matches!(self.current_token, Token::Eof(_)) && 
              !matches!(self.current_token, Token::Let(_)) &&
              !matches!(self.current_token, Token::Contract(_)) &&
              !matches!(self.current_token, Token::App(_)) {
            
            if let Token::Identifier(field_name, field_span) = &self.current_token {
                let field_name = field_name.clone();
                let field_start_span = *field_span;
                self.next_token();
                
                if matches!(self.current_token, Token::Colon(_)) {
                    self.next_token();
                    
                    if let Token::Identifier(type_name, _) = &self.current_token {
                        let type_name = type_name.clone();
                        self.next_token();
                        
                        fields.push(ContractField {
                            name: field_name,
                            type_ann: TypeIdentifier::Simple {
                                name: type_name,
                                span: self.current_token.span(),
                            },
                            span: field_start_span,
                        });
                    } else {
                        self.errors.push(ParseError {
                            message: "Expected type annotation".to_string(),
                            span: self.current_token.span(),
                        });
                    }
                } else {
                    // Skip this token and continue
                    self.next_token();
                }
            } else {
                // Skip this token and continue
                self.next_token();
            }
        }
        
        Some(ContractDefinition {
            name,
            generic_params,
            fields,
            span: start_span,
        })
    }
    
    /// Parse a let statement
    fn parse_let_statement(&mut self) -> Option<Statement> {
        let start_span = self.current_token.span();
        
        // Consume 'let' token  
        if !matches!(self.current_token, Token::Let(_)) {
            return None;
        }
        self.next_token();
        
        // Check for 's' (for "let's")
        let mut is_tracked = false;
        if let Token::Identifier(ident, _) = &self.current_token {
            if ident == "s" {
                self.next_token(); // Consume 's'
                is_tracked = false; // Regular variable
            }
        }
        
        // Check for 'track' keyword
        if let Token::Track(_) = &self.current_token {
            is_tracked = true;
            self.next_token();
        }
        
        // Get variable name
        let name = if let Token::Identifier(name, _) = &self.current_token {
            let var_name = name.clone();
            self.next_token();
            var_name
        } else {
            self.errors.push(ParseError {
                message: "Expected variable name".to_string(),
                span: self.current_token.span(),
            });
            return None;
        };
        
        // Check for type annotation
        let mut type_annotation = None;
        if matches!(self.current_token, Token::Colon(_)) {
            self.next_token();
            
            if let Token::Identifier(type_name, _) = &self.current_token {
                type_annotation = Some(type_name.clone());
                self.next_token();
            }
        }
        
        // Expect assignment
        if !matches!(self.current_token, Token::Assign(_)) {
            self.errors.push(ParseError {
                message: "Expected '=' in let statement".to_string(),
                span: self.current_token.span(),
            });
            return None;
        }
        self.next_token();
        
        // Parse value expression (simplified)
        let value = self.parse_simple_expression()?;
        
        Some(Statement::Let(LetStatement {
            name,
            is_tracked,
            type_annotation,
            value,
            span: start_span,
        }))
    }
    
    /// Parse a simple expression (number, string, identifier, or map literal)
    fn parse_simple_expression(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::Number(num, span) => {
                let expr = Expression::Literal(Literal::Number(num.clone()), *span);
                self.next_token();
                Some(expr)
            }
            Token::String(s, span) => {
                let expr = Expression::Literal(Literal::String(s.clone()), *span);
                self.next_token();
                Some(expr)
            }
            Token::Identifier(ident, span) => {
                let expr = Expression::Identifier(ident.clone(), *span);
                self.next_token();
                Some(expr)
            }
            Token::LBrace(_) => {
                // Parse map literal
                self.parse_map_literal()
            }
            _ => {
                self.errors.push(ParseError {
                    message: format!("Unexpected token in expression: {:?}", self.current_token),
                    span: self.current_token.span(),
                });
                None
            }
        }
    }
    
    /// Parse a map literal: { key: value, key: value }
    fn parse_map_literal(&mut self) -> Option<Expression> {
        let start_span = self.current_token.span();
        
        if !matches!(self.current_token, Token::LBrace(_)) {
            return None;
        }
        self.next_token(); // Consume '{'
        
        let mut pairs = Vec::new();
        
        while !matches!(self.current_token, Token::RBrace(_)) && !matches!(self.current_token, Token::Eof(_)) {
            // Parse key
            let key = self.parse_simple_expression()?;
            
            // Expect colon
            if !matches!(self.current_token, Token::Colon(_)) {
                self.errors.push(ParseError {
                    message: "Expected ':' in map literal".to_string(),
                    span: self.current_token.span(),
                });
                return None;
            }
            self.next_token();
            
            // Parse value
            let value = self.parse_simple_expression()?;
            
            pairs.push((key, value));
            
            // Skip comma if present
            if matches!(self.current_token, Token::Comma(_)) {
                self.next_token();
            }
        }
        
        // Consume '}'
        if matches!(self.current_token, Token::RBrace(_)) {
            self.next_token();
        } else {
            self.errors.push(ParseError {
                message: "Expected '}' to close map literal".to_string(),
                span: self.current_token.span(),
            });
        }
        
        Some(Expression::Literal(
            Literal::Map(MapLiteral {
                pairs,
                span: start_span,
            }),
            start_span,
        ))
    }
    
    /// Minimal implementation of parse_prefix for basic expressions
    fn parse_prefix(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::Number(num, span) => {
                let expr = Expression::Literal(Literal::Number(num.clone()), *span);
                self.next_token();
                Some(expr)
            }
            Token::String(s, span) => {
                let expr = Expression::Literal(Literal::String(s.clone()), *span);
                self.next_token();
                Some(expr)
            }
            Token::Identifier(ident, span) => {
                let expr = Expression::Identifier(ident.clone(), *span);
                self.next_token();
                Some(expr)
            }
            Token::LBrace(_) => {
                self.parse_map_literal()
            }
            _ => None
        }
    }
    
    /// Get precedence of the peek token
    fn peek_precedence(&self) -> Precedence {
        match &self.peek_token {
            Token::Plus(_) | Token::Minus(_) => Precedence::Sum,
            Token::Asterisk(_) | Token::Slash(_) => Precedence::Product,
            Token::Equals(_) => Precedence::Equals,
            _ => Precedence::Lowest
        }
    }
    
    /// Minimal implementation of parse_infix (stub for now)
    fn parse_infix(&mut self, _left: Expression) -> Option<Expression> {
        // For now, just return the left expression (no infix operations)
        None
    }
    
    /// Parse an app definition (stub implementation)
    fn parse_app_definition(&mut self) -> Option<AppDefinition> {
        let start_span = self.current_token.span();
        
        // Consume 'app' token
        if !matches!(self.current_token, Token::App(_)) {
            return None;
        }
        self.next_token();
        
        // Get app name
        let name = if let Token::Identifier(name, _) = &self.current_token {
            let app_name = name.clone();
            self.next_token();
            app_name
        } else {
            self.errors.push(ParseError {
                message: "Expected app name".to_string(),
                span: self.current_token.span(),
            });
            return None;
        };
        
        // Skip the rest for now (colon and body)
        while !matches!(self.current_token, Token::Eof(_)) && 
              !matches!(self.current_token, Token::Let(_)) &&
              !matches!(self.current_token, Token::Contract(_)) &&
              !matches!(self.current_token, Token::App(_)) {
            self.next_token();
        }
        
        Some(AppDefinition {
            name,
            body: AppBody::default(),
            span: start_span,
        })
    }
    
    /// Parse an enum definition (stub implementation)
    fn parse_enum_definition(&mut self) -> Option<EnumDefinition> {
        let start_span = self.current_token.span();
        
        // Consume 'enum' token
        if !matches!(self.current_token, Token::Enum(_)) {
            return None;
        }
        self.next_token();
        
        // Get enum name
        let name = if let Token::Identifier(name, _) = &self.current_token {
            let enum_name = name.clone();
            self.next_token();
            enum_name
        } else {
            self.errors.push(ParseError {
                message: "Expected enum name".to_string(),
                span: self.current_token.span(),
            });
            return None;
        };
        
        // Skip the rest for now
        while !matches!(self.current_token, Token::Eof(_)) && 
              !matches!(self.current_token, Token::Let(_)) &&
              !matches!(self.current_token, Token::Contract(_)) &&
              !matches!(self.current_token, Token::App(_)) &&
              !matches!(self.current_token, Token::Enum(_)) {
            self.next_token();
        }
        
        Some(EnumDefinition {
            name,
            variants: Vec::new(),
            span: start_span,
        })
}
