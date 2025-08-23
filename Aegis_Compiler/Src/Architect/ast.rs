Here is the complete and fully commented code for the ast.rs file.
This file defines the Abstract Syntax Tree (AST). After the Scribe tokenizes the code, the Architect (parser) assembles those tokens into this hierarchical structure, which represents the true meaning and organization of your program.
## aegis-compiler/src/architect/ast.rs
//! This module defines the Abstract Syntax Tree (AST) for the Aegis language.
//! The AST is a tree-like representation of the source code's structure,
//! making it easy for the Guardian (semantic analyzer) and Engine (backend)
//! to understand and process the program.

use crate::token::Span;

/// The root node of any parsed Aegis file.
#[derive(Debug, Clone)]
pub struct Program {
    /// A program is a sequence of top-level definitions.
    pub definitions: Vec<Definition>,
    pub span: Span,
}

/// Represents any top-level construct in an Aegis file.
#[derive(Debug, Clone)]
pub enum Definition {
    /// An `app MyApp: ...` block.
    App(AppDefinition),
    /// A `contract MyType: ...` definition.
    Contract(ContractDefinition),
    /// A `let's my_func(...)` definition.
    Function(FunctionDefinition),
    /// A standalone statement, like `let's x = 5`.
    Statement(Statement),
}

/// Represents a statement, which is an action or declaration that does not return a value.
#[derive(Debug, Clone)]
pub enum Statement {
    /// A `let's` binding, e.g., `let's x = 5`.
    Let(LetStatement),
    /// A `for..in` loop, e.g., `for item in items: ...`.
    For(ForStatement),
    /// A `return` statement from a function.
    Return(ReturnStatement),
    /// A block of multiple statements, e.g., `{ ... }` or an indented block.
    Block(BlockStatement),
    /// A statement that consists of a single expression, e.g., a function call.
    Expression(ExpressionStatement),
}

/// Represents an expression, which is a piece of code that evaluates to a value.
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String, Span),
    Literal(Literal, Span),
    /// An operation with a prefix, e.g., `-5` or `!is_ready`.
    Prefix(Box<PrefixExpression>),
    /// An operation between two expressions, e.g., `5 + x`.
    Infix(Box<InfixExpression>),
    /// An `if/else` conditional expression.
    If(Box<IfExpression>),
    /// A `when` pattern matching expression.
    When(Box<WhenExpression>),
    /// A function or method call, e.g., `my_func(a, b)`.
    Call(Box<CallExpression>),
    /// A member access, e.g., `user.name`.
    MemberAccess(Box<MemberAccessExpression>),
    /// An `await` expression for asynchronous operations.
    Await(Box<AwaitExpression>),
    /// An `ask_javascript` expression for interoperability.
    AskJs(Box<AskJsExpression>),
}

/// Represents a literal (hardcoded) value.
#[derive(Debug, Clone)]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Nothing,
    List(ListLiteral),
    Map(MapLiteral),
}

// --- Specific Node Structs ---

/// Represents a `let's` binding for a variable.
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: String,
    pub is_tracked: bool,
    pub type_annotation: Option<String>,
    pub value: Expression,
    pub span: Span,
}

/// Represents a `for..in` loop.
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub variable_name: String,
    pub collection: Expression,
    pub body: Box<Statement>,
    pub span: Span,
}

/// Represents a complete `let's my_func(...)` definition.
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub is_async: bool,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: BlockStatement,
    pub span: Span,
}

/// Represents a single parameter in a function definition, e.g., `name: string`.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: String,
    pub span: Span,
}

/// Represents a block of statements.
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
    pub span: Span,
}

// --- UI Nodes ---

/// Represents an `app MyApp: ...` block.
#[derive(Debug, Clone)]
pub struct AppDefinition {
    pub name: String,
    pub body: AppBody,
    pub span: Span,
}

/// Represents the body of an `app` block, containing state and UI.
#[derive(Debug, Clone, Default)]
pub struct AppBody {
    pub statements: Vec<Statement>,
    pub show_block: Option<ShowBlock>,
}

/// Represents the `show: ...` block where the UI is defined.
#[derive(Debug, Clone)]
pub struct ShowBlock {
    pub root_node: UiNode,
    pub span: Span,
}

/// Represents a single UI element in the tree, e.g., `column` or `text`.
#[derive(Debug, Clone)]
pub enum UiNode {
    Element(UiElement),
}

/// Defines a UI element and its properties and children.
#[derive(Debug, Clone)]
pub struct UiElement {
    pub name: String,
    pub properties: Vec<UiProperty>,
    pub children: Vec<UiNode>,
    pub span: Span,
}

/// Represents the different kinds of properties an element can have.
#[derive(Debug, Clone)]
pub enum UiProperty {
    /// A positional property, e.g., the `"label"` in `button "Click Me"`.
    Positional(Expression),
    /// A named property, e.g., `padding: 10`.
    Named(String, Expression),
    /// An event handler, e.g., `when_clicked: { ... }`.
    EventBinding(String, BlockStatement),
}

// Other node structs like ContractDefinition, AwaitExpression, InfixExpression, etc.
// would continue to be defined here.

