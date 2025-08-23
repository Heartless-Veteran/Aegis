//! Aegis Programming Language Compiler

pub mod token;
pub mod ast;
pub mod error;

// Include the Scribe from mod.rs
include!("mod.rs");

// Guardian module components
#[path = "Guardian/mod.rs"]
pub mod guardian_impl;

#[path = "Guardian/types.rs"]
pub mod guardian_types;

#[path = "Guardian/symbol table.rs"]
pub mod guardian_symbol_table;

// Architect module components
#[path = "Architect/mod.rs"]
pub mod architect_impl;

// Re-export the main types
pub use architect_impl::Architect;
pub use guardian_impl::Guardian;
pub use token::{Token, Span};
pub use ast::Expression;

// Create a module alias for Guardian
pub mod guardian {
    pub use crate::guardian_impl::Guardian;
}