//! This module defines the lexical building blocks of the Aegis language.
//! The Scribe (lexer) produces a stream of these `Token`s, which the
//! Architect (parser) then consumes to build the Abstract Syntax Tree.

/// Represents a byte-range in the source code string.
/// It's crucial for providing accurate, user-friendly error messages.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Span {
    /// The starting byte index of the token in the source string.
    pub start: usize,
    /// The ending byte index (exclusive) of the token.
    pub end: usize,
}

/// Represents every possible lexical unit in the Aegis language.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // --- Special Tokens ---
    /// A character that is not valid in the Aegis language.
    Illegal(char, Span),
    /// Marks the end of the input file.
    Eof(Span),

    // --- Identifiers & Literals ---
    /// A user-defined name, e.g., `my_variable`, `MyApp`.
    Identifier(String, Span),
    /// A numeric literal, e.g., `10`, `3.14`.
    Number(String, Span),
    /// A string literal, e.g., `"Hello, World!"`.
    String(String, Span),

    // --- Operators ---
    /// The assignment operator, `=`.
    Assign(Span),
    /// The equality operator, `==`.
    Equals(Span),
    /// The inequality operator, `!=`.
    NotEquals(Span),
    /// The addition operator, `+`.
    Plus(Span),
    /// The subtraction operator, `-`.
    Minus(Span),
    /// The logical NOT operator, `!`.
    Bang(Span),
    /// The multiplication operator, `*`.
    Asterisk(Span),
    /// The division operator, `/`.
    Slash(Span),
    /// The less than operator, `<`.
    LessThan(Span),
    /// The greater than operator, `>`.
    GreaterThan(Span),
    /// The member access operator, `.`.
    Dot(Span),
    /// The fat arrow for `when` cases, `=>`.
    FatArrow(Span),

    // --- Delimiters ---
    Comma(Span),        // ,
    Colon(Span),        // :
    LParen(Span),       // (
    RParen(Span),       // )
    LBrace(Span),       // {
    RBrace(Span),       // }
    LBracket(Span),     // [
    RBracket(Span),     // ]

    // --- Keywords ---
    /// The `app` keyword for defining an application.
    App(Span),
    /// The `let's` keyword for variable and function declarations.
    Let(Span),
    /// The `track` keyword for reactive state variables.
    Track(Span),
    /// The `when` keyword for pattern matching.
    When(Span),
    /// The `show` keyword for defining a UI block.
    Show(Span),
    /// The `change` keyword for defining OS event handlers.
    Change(Span),
    /// The `contract` keyword for defining custom data types.
    Contract(Span),
    /// The `for` keyword for iteration.
    For(Span),
    /// The `in` keyword used in `for` loops.
    In(Span),
    /// The `is` keyword used in `when` expressions.
    Is(Span),
    /// The `return` keyword for exiting a function.
    Return(Span),
    /// The boolean literal `true`.
    True(Span),
    /// The boolean literal `false`.
    False(Span),
    /// The `if` keyword for conditional logic.
    If(Span),
    /// The `else` keyword for conditional logic.
    Else(Span),
    /// The `async` keyword for defining asynchronous functions.
    Async(Span),
    /// The `await` keyword for pausing an asynchronous function.
    Await(Span),
    /// The literal for a null or empty value, `nothing`.
    Nothing(Span),
}

/// Helper methods for the `Token` enum.
impl Token {
    /// A convenience function to extract the `Span` from any token variant.
    pub fn span(&self) -> Span {
        match self {
            Token::Illegal(_, s) | Token::Eof(s) | Token::Identifier(_, s) |
            Token::Number(_, s) | Token::String(_, s) | Token::Assign(s) |
            Token::Equals(s) | Token::NotEquals(s) | Token::Plus(s) |
            Token::Minus(s) | Token::Bang(s) | Token::Asterisk(s) |
            Token::Slash(s) | Token::LessThan(s) | Token::GreaterThan(s) |
            Token::Dot(s) | Token::FatArrow(s) | Token::Comma(s) | Token::Colon(s) |
            Token::LParen(s) | Token::RParen(s) | Token::LBrace(s) |
            Token::RBrace(s) | Token::LBracket(s) | Token::RBracket(s) |

            Token::App(s) | Token::Let(s) | Token::Track(s) |
            Token::When(s) | Token::Show(s) | Token::Change(s) |
            Token::Contract(s) | Token::For(s) | Token::In(s) |
            Token::Is(s) | Token::Return(s) | Token::True(s) |
            Token::False(s) | Token::If(s) | Token::Else(s) |
            Token::Async(s) | Token::Await(s) | Token::Nothing(s) => *s,
        }
    }

    /// A convenience function to get the string value of an `Identifier` token.
    /// Returns an empty string for all other token types.
    pub fn literal_string(&self) -> String {
        match self {
            Token::Identifier(s, _) => s.clone(),
            _ => "".to-string(),
        }
    }
}
