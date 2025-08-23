//! Error types for the Aegis compiler

use crate::token::Span;
use std::fmt;

/// Parse errors from the Architect (parser)
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl ParseError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at {}..{}: {}", 
            self.span.start, self.span.end, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Semantic errors from the Guardian (semantic analyzer)
#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
    pub error_type: SemanticErrorType,
}

impl SemanticError {
    pub fn new(message: String, span: Span, error_type: SemanticErrorType) -> Self {
        Self { message, span, error_type }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Semantic error at {}..{}: {}", 
            self.span.start, self.span.end, self.message)
    }
}

impl std::error::Error for SemanticError {}

/// Types of semantic errors
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticErrorType {
    /// Variable or function not found
    UndefinedSymbol,
    /// Type mismatch in assignment or operation
    TypeMismatch,
    /// Function called with wrong number of arguments
    ArityMismatch,
    /// Return type doesn't match function signature
    ReturnTypeMismatch,
    /// await used outside async function
    AwaitOutsideAsync,
    /// Duplicate declaration
    DuplicateDeclaration,
    /// Invalid member access
    InvalidMemberAccess,
    /// Invalid operation for given types
    InvalidOperation,
    /// Missing required field in contract
    MissingField,
    /// Invalid UI component or property
    InvalidUIComponent,
    /// Unknown field in contract initialization
    UnknownField,
    /// Invalid field key in contract initialization
    InvalidFieldKey,
    /// Undefined type
    UndefinedType,
    /// Other semantic error
    Other,
}

/// Code generation errors from the Engine
#[derive(Debug, Clone)]
pub struct CodeGenError {
    pub message: String,
    pub span: Option<Span>,
}

impl CodeGenError {
    pub fn new(message: String, span: Option<Span>) -> Self {
        Self { message, span }
    }
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = self.span {
            write!(f, "Code generation error at {}..{}: {}", 
                span.start, span.end, self.message)
        } else {
            write!(f, "Code generation error: {}", self.message)
        }
    }
}

impl std::error::Error for CodeGenError {}