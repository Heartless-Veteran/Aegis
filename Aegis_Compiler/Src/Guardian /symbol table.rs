use crate::guardian_types::Type;
use std::collections::HashMap;

/// Represents a declared identifier in the code.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Variable {
        is_tracked: bool,
    },
    Type,
    Function {
        param_types: Vec<Type>,
        return_type: Box<Type>,
    },
    Enum {
        variants: Vec<String>,
    },
    Contract {
        fields: HashMap<String, Type>,
    },
    GenericContract {
        params: Vec<String>,
        fields: HashMap<String, Type>,
    },
}

/// A new table is created for each new scope (e.g., function or loop bodies).
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    /// Optional link to the parent (outer) scope.
    outer: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new_enclosed(outer: SymbolTable) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    /// Defines a new symbol in the current scope. Fails if it's a redeclaration.
    pub fn define(&mut self, name: String, ty: Type, kind: SymbolKind) -> bool {
        if self.store.contains_key(&name) {
            return false;
        }
        let symbol = Symbol {
            name: name.clone(),
            kind,
            ty,
        };
        self.store.insert(name, symbol);
        true
    }

    /// Resolves a symbol by looking in the current scope, then recursively checking outer scopes.
    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        match self.store.get(name) {
            Some(symbol) => Some(symbol.clone()),
            None => self.outer.as_ref().and_then(|o| o.resolve(name)),
        }
    }
}
