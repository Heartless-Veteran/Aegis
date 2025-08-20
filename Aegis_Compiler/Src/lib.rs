//! Aegis Programming Language Compiler
//! 
//! This crate provides the complete compiler infrastructure for the Aegis
//! programming language, including lexical analysis, parsing, semantic analysis,
//! and code generation.
//! 
//! ## Architecture
//! 
//! The compiler is organized into several key components:
//! 
//! - **Scribe (Lexer)**: Converts source code into tokens
//! - **Architect (Parser)**: Builds Abstract Syntax Trees from tokens  
//! - **Guardian (Semantic Analyzer)**: Performs type checking and validation
//! - **Engine (Code Generator)**: Generates target code (Kotlin/Android)
//! 
//! ## Usage
//! 
//! ```rust
//! use aegis_compiler::{Scribe, Architect, Guardian};
//! 
//! let source = r#"
//!     app MyApp:
//!         let's track counter = 0
//!         show:
//!             text "Count: {counter}"
//! "#;
//! 
//! // Lexical analysis
//! let scribe = Scribe::new(source);
//! 
//! // Parsing
//! let mut architect = Architect::new(scribe);
//! let program = architect.parse_program();
//! 
//! // Semantic analysis
//! let mut guardian = Guardian::new();
//! guardian.check_program(&program);
//! ```

pub mod token;
pub mod architect;
pub mod guardian;
pub mod engine;
pub mod error;

// Re-export main types for convenience
pub use crate::Scribe;
pub use architect::Architect;
pub use guardian::Guardian;
pub use token::{Token, Span};
