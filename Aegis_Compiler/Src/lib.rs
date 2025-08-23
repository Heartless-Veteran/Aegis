//! Aegis Programming Language Compiler

pub mod token;
pub mod ast;
pub mod error;

// Include the Scribe from mod.rs
include!("mod.rs");

// Include the Guardian module properly
#[path = "Guardian /mod.rs"]
pub mod guardian_impl;

#[path = "Guardian /types.rs"]
pub mod guardian_types;

#[path = "Guardian /symbol table.rs"]
pub mod guardian_symbol_table;

// Stub modules for components that need to be implemented
pub mod architect {
    use crate::{Scribe, error::ParseError, ast::Program};
    
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
    pub use crate::guardian_impl::Guardian;
}

// Re-export main types for convenience
pub use architect::Architect;
pub use guardian::Guardian;
pub use token::{Token, Span};
pub use ast::Expression;