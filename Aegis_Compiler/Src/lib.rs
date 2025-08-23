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

// Include the real Architect module
#[path = "Architect /mod.rs"]
pub mod architect_impl;

pub use architect_impl::Architect;

pub mod guardian {
    pub use crate::guardian_impl::Guardian;
}

// Re-export main types for convenience
pub use guardian::Guardian;
pub use token::{Token, Span};
pub use ast::Expression;