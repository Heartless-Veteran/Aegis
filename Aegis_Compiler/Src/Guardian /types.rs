//! This module defines the `Type` enum.
//! It is used by the Guardian (semantic analyzer) to represent and track the
//! type of every expression and variable within an Aegis program.

use std::collections::HashMap;

/// The internal representation of types within the Guardian.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// The primitive number type, representing both integers and floating-point values.
    Number,
    /// The primitive boolean type, representing `true` or `false`.
    Boolean,
    /// The primitive string type, representing a sequence of characters.
    String,
    /// The `nothing` type, representing the absence of a value (similar to null or unit).
    Nothing,

    /// Represents a type that couldn't be determined due to a prior error.
    /// This is used internally to prevent reporting cascading, useless errors.
    Error,

    /// A user-defined type from a `contract`, identified by its name.
    Custom(String),

    /// A generic List with an inner type, e.g., `List<Number>`.
    List(Box<Type>),

    /// A generic Map with key and value types, e.g., `Map<String, User>`.
    Map(Box<Type>, Box<Type>),

    /// A generic Set with an inner type, e.g., `Set<String>`.
    Set(Box<Type>),

    /// An optional type, representing a value that could be `nothing`.
    /// Used for operations that might fail, like `Map.get(key)`.
    Optional(Box<Type>),

    /// A future type for the result of an `async` operation, e.g., `Future<String>`.
    Future(Box<Type>),

    /// A dynamic type from an external source, like the JavaScript Bridge,
    /// where the type is not known at compile time.
    Dynamic,
    
    /// A user-defined enum type with its variants and their associated types.
    Enum {
        name: String,
        /// The key is the variant name, the value is the list of associated types.
        variants: HashMap<String, Vec<Type>>,
    },
}
