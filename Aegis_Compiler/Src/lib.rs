//! Aegis Programming Language Compiler

pub mod token;
pub mod ast;
pub mod error;

// Include the Scribe from mod.rs
include!("mod.rs");

// Stub modules for components that need to be implemented
pub mod architect {
    use crate::{Scribe, token::Token, error::ParseError, ast::Program};
    
    pub struct Architect {
        pub errors: Vec<ParseError>,
    }
    
    impl Architect {
        pub fn new(_scribe: Scribe) -> Self {
            Self { errors: Vec::new() }
        }
        
        pub fn parse_program(&mut self) -> Program {
            Program {
                definitions: Vec::new(),
                span: crate::token::Span { start: 0, end: 0 },
            }
        }
    }
}

pub mod guardian {
    use crate::{error::SemanticError, ast::Program};
    
    pub struct Guardian {
        pub errors: Vec<SemanticError>,
    }
    
    impl Guardian {
        pub fn new() -> Self {
            Self { errors: Vec::new() }
        }
        
        pub fn check_program(&mut self, _program: &Program) {
            // Stub implementation
        }
    }
}

// Re-export main types for convenience
pub use architect::Architect;
pub use guardian::Guardian;
pub use token::{Token, Span};