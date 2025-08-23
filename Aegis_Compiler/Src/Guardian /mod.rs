//! This module contains the Guardian, the semantic analyzer for the Aegis language.
//! It traverses the Abstract Syntax Tree (AST) provided by the Architect to
//! ensure the code is logically sound and adheres to all of the language's rules.
//! It checks for type mismatches, undeclared variables, incorrect function calls,
//! and other semantic errors.

use crate::ast::*;
use crate::error::SemanticError;
use crate::guardian_symbol_table::{SymbolKind, SymbolTable};
use crate::guardian_types::Type;
use std::collections::HashMap;

/// The Guardian walks the AST to find semantic errors and build metadata.
pub struct Guardian {
    /// A list of semantic errors found during analysis.
    pub errors: Vec<SemanticError>,
    /// The symbol table for managing scopes and declared identifiers.
    symbol_table: SymbolTable,
    /// The dependency graph for the reactive UI system.
    /// Maps a tracked variable name to a list of UI node IDs that use it.
    dependency_graph: HashMap<String, Vec<String>>,
    /// Context flag for validating that `await` is only used inside `async` functions.
    is_in_async_context: bool,
    /// Context for validating that `return` statements match the function's signature.
    current_return_type: Option<Type>,
    // Other context fields can be added here as needed.
}

impl Guardian {
    /// Creates a new Guardian, initializing its state.
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            symbol_table: SymbolTable::default(),
            dependency_graph: HashMap::new(),
            is_in_async_context: false,
            current_return_type: None,
        }
    }

    /// The main entry point for semantic analysis.
    /// It iterates through all top-level definitions in the program.
    pub fn check_program(&mut self, program: &Program) {
        for def in &program.definitions {
            self.check_definition(def);
        }
    }

    /// Dispatches to the correct checking function based on the definition type.
    fn check_definition(&mut self, def: &Definition) {
        match def {
            Definition::Enum(enum_def) => self.check_enum_definition(enum_def),
            // Other definition types would be handled here in a complete implementation
            _ => {
                // This is a placeholder for the full dispatch logic. In a real implementation,
                // it would look at the current definition and decide which specific check
                // function to call (e.g., `check_function_definition`).
            }
        }
    }

    /// Dispatches to the correct checking function for a statement.
    fn check_statement(&mut self, stmt: &Statement) {
        // This is a placeholder for the full dispatch logic for statements like
        // `let's`, `for`, `return`, etc.
    }

    /// The main entry point for type inference and checking of expressions.
    /// This function recursively determines the type of every expression,
    /// reporting errors for invalid operations.
    pub fn infer_expression_type(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Literal(Literal::Number(_), _) => Type::Number,
            Expression::Literal(Literal::String(_), _) => Type::String,
            Expression::Literal(Literal::Boolean(_), _) => Type::Boolean,
            Expression::Literal(Literal::Nothing, _) => Type::Nothing,
            
            Expression::Identifier(name, _) => {
                if let Some(symbol) = self.symbol_table.resolve(name) {
                    symbol.ty
                } else {
                    // Error: Undefined identifier
                    Type::Error
                }
            }

            // UPDATED: MemberAccess now also handles enum instantiation.
            Expression::MemberAccess(member_access) => {
                if let Expression::Identifier(ident_name, _) = &member_access.object {
                    // Check if the identifier is a known type.
                    if let Some(symbol) = self.symbol_table.resolve(ident_name) {
                        // If the symbol is an enum...
                        if let SymbolKind::Enum { variants } = &symbol.kind {
                            // ...and the property is a valid variant of that enum...
                            if variants.contains(&member_access.property) {
                                // ...then this is a valid enum instantiation.
                                return Type::Enum(ident_name.clone());
                            } else {
                                // Error: `property` is not a variant of enum `ident_name`.
                                return Type::Error;
                            }
                        }
                    }
                }
                // (Fallback to existing logic for object.property access)
                Type::Error
            }

            // UPDATED: `when` expression checking is now more powerful.
            Expression::When(when_expr) => {
                let subject_type = self.infer_expression_type(&when_expr.value);
                let mut case_types = Vec::new();

                for case in &when_expr.cases {
                    match &case.pattern {
                        WhenPattern::Literal(literal) => {
                            let literal_type = self.infer_literal_type(literal);
                            if literal_type != subject_type {
                                // Error: Pattern type mismatch.
                            }
                        }
                        // NEW: Check enum variant patterns.
                        WhenPattern::EnumVariant { enum_name, variant_name, .. } => {
                            // Check that the pattern's enum type matches the subject's enum type.
                            if subject_type != Type::Enum(enum_name.clone()) {
                                // Error: Pattern is for a different enum type.
                                return Type::Error;
                            }
                            // Check that the variant name is valid for this enum.
                            if let Some(symbol) = self.symbol_table.resolve(enum_name) {
                                if let SymbolKind::Enum { variants } = &symbol.kind {
                                    if !variants.contains(variant_name) {
                                        // Error: Invalid variant name for enum.
                                        return Type::Error;
                                    }
                                }
                            }
                        }
                        WhenPattern::Identifier(_) => {
                            // For now, assume identifier patterns are valid
                        }
                        WhenPattern::Else => {
                            // Else patterns are always valid
                        }
                    }
                    case_types.push(self.infer_expression_type(&case.body));
                }

                // Ensure all cases return the same type.
                if case_types.windows(2).all(|w| w[0] == w[1]) {
                    case_types.get(0).cloned().unwrap_or(Type::Nothing)
                } else {
                    // Error: `when` expression cases must return the same type.
                    Type::Error
                }
            }

            Expression::Infix(infix_expr) => {
                let left_type = self.infer_expression_type(&infix_expr.left);
                let right_type = self.infer_expression_type(&infix_expr.right);
                // Check if the operation is valid for the inferred types.
                if left_type == Type::Number && right_type == Type::Number {
                    Type::Number
                } else {
                    // If not, push a SemanticError to the `errors` vector.
                    Type::Error
                }
            }
            
            // ... cases for all other expression types ...
            _ => Type::Error,
        }
    }

    // In a complete implementation, this file would continue with many specific
    // checking functions, such as:
    //
    // /// Checks a function definition, manages its scope, and validates return types.
    // fn check_function_definition(&mut self, func: &FunctionDefinition) { ... }
    //
    // /// Checks a contract definition and registers it as a new type.
    // fn check_contract_definition(&mut self, contract: &ContractDefinition) { ... }
    //
    // /// Recursively checks a UI tree, validates components, and builds the dependency graph.
    // fn check_ui_node(&mut self, node: &UiNode) { ... }

    /// Validates an enum definition and adds it to the symbol table.
    pub fn check_enum_definition(&mut self, enum_def: &EnumDefinition) {
        // 1. Define the enum type itself in the current scope.
        let enum_type = Type::Enum(enum_def.name.clone());
        let enum_kind = SymbolKind::Enum {
            variants: enum_def.variants.iter().map(|v| v.name.clone()).collect(),
        };
        self.symbol_table.define(enum_def.name.clone(), enum_type, enum_kind);

        // 2. Future: Validate that variant names are unique, etc.
    }

    /// Helper method to infer the type of a literal value.
    fn infer_literal_type(&self, literal: &Literal) -> Type {
        match literal {
            Literal::Number(_) => Type::Number,
            Literal::String(_) => Type::String,
            Literal::Boolean(_) => Type::Boolean,
            Literal::Nothing => Type::Nothing,
            Literal::List(_) => Type::Error, // Simplified for now
            Literal::Map(_) => Type::Error,  // Simplified for now
        }
    }
}
