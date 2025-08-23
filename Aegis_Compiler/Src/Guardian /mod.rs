//! This module contains the Guardian, the semantic analyzer for the Aegis language.
//! It traverses the Abstract Syntax Tree (AST) provided by the Architect to
//! ensure the code is logically sound and adheres to all of the language's rules.
//! It checks for type mismatches, undeclared variables, incorrect function calls,
//! and other semantic errors.

use crate::ast::*;
use crate::error::{SemanticError, SemanticErrorType};
use crate::guardian_symbol_table::{SymbolKind, SymbolTable};
use crate::guardian_types::Type;
use crate::token::Span;
use std::collections::HashMap;

/// The Guardian walks the AST to find semantic errors and build metadata.
#[derive(Default)]
pub struct Guardian {
    /// A list of semantic errors found during analysis.
    pub errors: Vec<SemanticError>,
    /// The symbol table for managing scopes and declared identifiers.
    symbol_table: SymbolTable,
    // Note: Additional context fields like dependency_graph, is_in_async_context, 
    // and current_return_type will be added when implementing those features.
}

impl Guardian {
    /// Creates a new Guardian, initializing its state.
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            symbol_table: SymbolTable::default(),
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
            Definition::Contract(contract_def) => self.check_contract_definition(contract_def),
            Definition::Function(func_def) => self.check_function_definition(func_def),
            Definition::Statement(stmt) => self.check_statement(stmt),
            Definition::App(app_def) => self.check_app_definition(app_def),
        }
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

            // UPDATED: A `Call` expression can now be an enum instantiation with data.
            Expression::Call(call_expr) => {
                // Check if the "function" being called is an enum variant, e.g., `LoadState::Success`.
                if let Expression::MemberAccess(member_access) = &call_expr.function {
                    if let Expression::Identifier(ident_name, _) = &member_access.object {
                        if let Some(symbol) = self.symbol_table.resolve(ident_name) {
                            if let Type::Enum { name, variants } = &symbol.ty {
                                // This is an enum. Now check if the variant exists.
                                if let Some(expected_types) = variants.get(&member_access.property) {
                                    // It's a valid variant. Now check the arguments.
                                    if call_expr.arguments.len() != expected_types.len() {
                                        // Error: Incorrect number of arguments for variant.
                                        return Type::Error;
                                    }
                                    // Check that each argument's type matches the expected type.
                                    for (arg, expected_ty) in call_expr.arguments.iter().zip(expected_types) {
                                        let arg_ty = self.infer_expression_type(arg);
                                        if &arg_ty != expected_ty {
                                            // Error: Mismatched argument type.
                                            return Type::Error;
                                        }
                                    }
                                    // If all checks pass, the type is the enum itself.
                                    return Type::Enum { name: name.clone(), variants: variants.clone() };
                                }
                            }
                        }
                    }
                }
                // (Fallback to existing logic for regular function calls)
                Type::Error
            }

            // UPDATED: MemberAccess now also handles enum instantiation.
            Expression::MemberAccess(member_access) => {
                if let Expression::Identifier(ident_name, _) = &member_access.object {
                    // Check if the identifier is a known type.
                    if let Some(symbol) = self.symbol_table.resolve(ident_name) {
                        // If the symbol is an enum...
                        if let Type::Enum { name, variants } = &symbol.ty {
                            // ...and the property is a valid variant of that enum...
                            if variants.contains_key(&member_access.property) {
                                // Check if the variant has no associated data (0 types)
                                if let Some(variant_types) = variants.get(&member_access.property) {
                                    if variant_types.is_empty() {
                                        // ...then this is a valid enum instantiation without data.
                                        return Type::Enum { name: name.clone(), variants: variants.clone() };
                                    }
                                }
                            }
                            // Error: `property` is not a variant of enum `ident_name` or has associated data.
                            return Type::Error;
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
                            if let Type::Enum { name: subject_name, .. } = &subject_type {
                                if subject_name != enum_name {
                                    // Error: Pattern is for a different enum type.
                                    return Type::Error;
                                }
                            } else {
                                // Error: Subject is not an enum type.
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
                    case_types.first().cloned().unwrap_or(Type::Nothing)
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
        let mut resolved_variants = HashMap::new();

        // 1. Resolve the type names for each variant.
        for variant in &enum_def.variants {
            let mut resolved_types = Vec::new();
            for type_name in &variant.types {
                // For this example, we'll assume it's a Custom type.
                // In a complete implementation, this would look up the type in the symbol table.
                resolved_types.push(Type::Custom(type_name.clone()));
            }
            resolved_variants.insert(variant.name.clone(), resolved_types);
        }

        // 2. Define the enum type itself in the current scope.
        let enum_type = Type::Enum {
            name: enum_def.name.clone(),
            variants: resolved_variants.clone(),
        };
        let enum_kind = SymbolKind::Enum {
            variants: resolved_variants.keys().cloned().collect(),
        };
        self.symbol_table.define(enum_def.name.clone(), enum_type, enum_kind);
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

    /// Checks a contract definition and registers it as a new type.
    pub fn check_contract_definition(&mut self, contract: &ContractDefinition) {
        let mut fields = HashMap::new();
        
        // Process each field in the contract
        for field in &contract.fields {
            let field_type = self.resolve_type_from_string(&field.type_annotation);
            fields.insert(field.name.clone(), field_type);
        }
        
        // Create the contract type
        let contract_type = Type::Custom(contract.name.clone());
        let contract_kind = SymbolKind::Contract { fields };
        
        // Define the contract type in the symbol table
        if !self.symbol_table.define(contract.name.clone(), contract_type, contract_kind) {
            self.errors.push(SemanticError::new(
                format!("Contract '{}' is already declared", contract.name),
                contract.span.clone(),
                SemanticErrorType::DuplicateDeclaration,
            ));
        }
    }

    /// Checks a function definition, manages its scope, and validates return types.
    pub fn check_function_definition(&mut self, func: &FunctionDefinition) {
        // For now, just add the function to the symbol table
        // TODO: Implement full function body checking
        let param_types: Vec<Type> = func.parameters.iter()
            .map(|p| self.resolve_type_from_string(&p.type_annotation))
            .collect();
            
        let return_type = if let Some(ret_type) = &func.return_type {
            Box::new(self.resolve_type_from_string(ret_type))
        } else {
            Box::new(Type::Nothing)
        };
        
        let func_type = Type::Function {
            params: param_types.clone(),
            return_type: return_type.clone(),
        };
        
        let func_kind = SymbolKind::Function {
            param_types,
            return_type,
        };
        
        if !self.symbol_table.define(func.name.clone(), func_type, func_kind) {
            self.errors.push(SemanticError::new(
                format!("Function '{}' is already declared", func.name),
                func.span.clone(),
                SemanticErrorType::DuplicateDeclaration,
            ));
        }
    }

    /// Checks a statement
    pub fn check_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let(let_stmt) => self.check_let_statement(let_stmt),
            _ => {
                // Other statement types not implemented yet
            }
        }
    }

    /// Checks an app definition
    pub fn check_app_definition(&mut self, app: &AppDefinition) {
        // For now, just register the app
        // TODO: Check app body and show blocks
        let app_type = Type::Custom(format!("App<{}>", app.name));
        let app_kind = SymbolKind::Type;
        
        if !self.symbol_table.define(app.name.clone(), app_type, app_kind) {
            self.errors.push(SemanticError::new(
                format!("App '{}' is already declared", app.name),
                app.span.clone(),
                SemanticErrorType::DuplicateDeclaration,
            ));
        }
    }

    /// Checks a let statement including contract initializers
    pub fn check_let_statement(&mut self, let_stmt: &LetStatement) {
        // Infer the type of the value expression
        let value_type = self.infer_expression_type(&let_stmt.value);
        
        // If there's a type annotation, validate it matches
        if let Some(expected_type_str) = &let_stmt.type_annotation {
            let expected_type = self.resolve_type_from_string(expected_type_str);
            
            // Special handling for contract initializers (map literals)
            if let Expression::Literal(Literal::Map(map_literal), _) = &let_stmt.value {
                if let Type::Custom(contract_name) = &expected_type {
                    self.check_contract_initialization(contract_name, map_literal, &let_stmt.span);
                }
            } else {
                // Regular type checking
                if !self.types_are_compatible(&expected_type, &value_type) {
                    self.errors.push(SemanticError::new(
                        format!("Type mismatch: expected {:?}, found {:?}", expected_type, value_type),
                        let_stmt.span.clone(),
                        SemanticErrorType::TypeMismatch,
                    ));
                }
            }
        }
        
        // Register the variable
        let var_type = if let Some(type_annotation) = &let_stmt.type_annotation {
            self.resolve_type_from_string(type_annotation)
        } else {
            value_type
        };
        
        let var_kind = SymbolKind::Variable {
            is_tracked: let_stmt.is_tracked,
        };
        
        if !self.symbol_table.define(let_stmt.name.clone(), var_type, var_kind) {
            self.errors.push(SemanticError::new(
                format!("Variable '{}' is already declared", let_stmt.name),
                let_stmt.span.clone(),
                SemanticErrorType::DuplicateDeclaration,
            ));
        }
    }

    /// Check contract initialization from map literal
    pub fn check_contract_initialization(&mut self, contract_name: &str, map_literal: &MapLiteral, span: &Span) {
        // Look up the contract definition
        if let Some(contract_symbol) = self.symbol_table.resolve(contract_name) {
            if let SymbolKind::Contract { fields } = &contract_symbol.kind {
                let mut found_fields = HashMap::new();
                
                // Check each field in the map literal
                for (key_expr, value_expr) in &map_literal.pairs {
                    let field_name = match key_expr {
                        Expression::Literal(Literal::String(s), _) => s.trim_matches('"'), // Remove quotes
                        Expression::Identifier(name, _) => name.as_str(), // Allow identifiers for field names
                        _ => {
                            self.errors.push(SemanticError::new(
                                "Contract field keys must be string literals or identifiers".to_string(),
                                span.clone(),
                                SemanticErrorType::InvalidFieldKey,
                            ));
                            continue;
                        }
                    };
                        
                    if let Some(expected_type) = fields.get(field_name) {
                        let actual_type = self.infer_expression_type(value_expr);
                        
                        if !self.types_are_compatible(expected_type, &actual_type) {
                            self.errors.push(SemanticError::new(
                                format!("Type mismatch in field '{}': expected {:?}, found {:?}", 
                                    field_name, expected_type, actual_type),
                                span.clone(),
                                SemanticErrorType::TypeMismatch,
                            ));
                        }
                        
                        found_fields.insert(field_name.to_string(), true);
                    } else {
                        self.errors.push(SemanticError::new(
                            format!("Unknown field '{}' in contract '{}'", field_name, contract_name),
                            span.clone(),
                            SemanticErrorType::UnknownField,
                        ));
                    }
                }
                
                // Check for missing fields
                for (field_name, _) in fields {
                    if !found_fields.contains_key(field_name) {
                        self.errors.push(SemanticError::new(
                            format!("Missing required field '{}' in contract '{}'", field_name, contract_name),
                            span.clone(),
                            SemanticErrorType::MissingField,
                        ));
                    }
                }
            }
        } else {
            self.errors.push(SemanticError::new(
                format!("Undefined contract type '{}'", contract_name),
                span.clone(),
                SemanticErrorType::UndefinedType,
            ));
        }
    }

    /// Helper method to resolve type from string annotation
    fn resolve_type_from_string(&self, type_str: &str) -> Type {
        match type_str {
            "number" => Type::Number,
            "string" => Type::String,
            "boolean" => Type::Boolean,
            "nothing" => Type::Nothing,
            _ => {
                // Check if it's a user-defined type
                if let Some(symbol) = self.symbol_table.resolve(type_str) {
                    symbol.ty
                } else {
                    Type::Custom(type_str.to_string())
                }
            }
        }
    }

    /// Helper method to check type compatibility
    fn types_are_compatible(&self, expected: &Type, actual: &Type) -> bool {
        match (expected, actual) {
            (Type::Error, _) | (_, Type::Error) => true, // Error types are compatible with anything
            (a, b) if a == b => true,
            _ => false,
        }
    }
}
