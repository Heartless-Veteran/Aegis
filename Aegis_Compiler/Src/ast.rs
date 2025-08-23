//! Abstract Syntax Tree (AST) definitions for the Aegis language

use crate::token::Span;

/// The root node of any parsed Aegis file
#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
    pub span: Span,
}

/// Top-level definitions
#[derive(Debug, Clone)]
pub enum Definition {
    App(AppDefinition),
    Contract(ContractDefinition),
    Function(FunctionDefinition),
    Statement(Statement),
    Enum(EnumDefinition),
}

/// Statements
#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    For(ForStatement),
    Return(ReturnStatement),
    Block(BlockStatement),
    Expression(ExpressionStatement),
}

/// Expressions
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String, Span),
    Literal(Literal, Span),
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
    If(Box<IfExpression>),
    When(Box<WhenExpression>),
    Call(Box<CallExpression>),
    MemberAccess(Box<MemberAccessExpression>),
    Await(Box<AwaitExpression>),
    AskJs(Box<AskJsExpression>),
}

/// Literal values
#[derive(Debug, Clone)]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Nothing,
    List(ListLiteral),
    Map(MapLiteral),
}

/// Let statement
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: String,
    pub is_tracked: bool,
    pub type_annotation: Option<String>,
    pub value: Expression,
    pub span: Span,
}

/// For statement
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub variable_name: String,
    pub collection: Expression,
    pub body: Box<Statement>,
    pub span: Span,
}

/// Return statement
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Expression,
    pub span: Span,
}

/// Block statement
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// Expression statement
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub span: Span,
}

/// Function definition
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub is_async: bool,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: BlockStatement,
    pub span: Span,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: String,
    pub span: Span,
}

/// Contract definition
#[derive(Debug, Clone)]
pub struct ContractDefinition {
    pub name: String,
    pub fields: Vec<ContractField>,
    pub span: Span,
}

/// Contract field
#[derive(Debug, Clone)]
pub struct ContractField {
    pub name: String,
    pub type_annotation: String,
    pub span: Span,
}

/// App definition
#[derive(Debug, Clone)]
pub struct AppDefinition {
    pub name: String,
    pub body: AppBody,
    pub span: Span,
}

/// App body
#[derive(Debug, Clone, Default)]
pub struct AppBody {
    pub statements: Vec<Statement>,
    pub show_block: Option<ShowBlock>,
}

/// Show block
#[derive(Debug, Clone)]
pub struct ShowBlock {
    pub root_node: UiNode,
    pub span: Span,
}

/// UI node
#[derive(Debug, Clone)]
pub enum UiNode {
    Element(UiElement),
}

/// UI element
#[derive(Debug, Clone)]
pub struct UiElement {
    pub name: String,
    pub properties: Vec<UiProperty>,
    pub children: Vec<UiNode>,
    pub span: Span,
}

/// UI property
#[derive(Debug, Clone)]
pub enum UiProperty {
    Positional(Expression),
    Named(String, Expression),
    EventBinding(String, BlockStatement),
}

/// Prefix expression
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub operator: PrefixOperator,
    pub right: Expression,
    pub span: Span,
}

/// Prefix operators
#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOperator {
    Not,
    Minus,
}

/// Infix expression
#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Expression,
    pub operator: InfixOperator,
    pub right: Expression,
    pub span: Span,
}

/// Infix operators
#[derive(Debug, Clone, PartialEq)]
pub enum InfixOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

/// If expression
#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Expression,
    pub then_branch: Expression,
    pub else_branch: Option<Expression>,
    pub span: Span,
}

/// When expression
#[derive(Debug, Clone)]
pub struct WhenExpression {
    pub value: Expression,
    pub cases: Vec<WhenCase>,
    pub span: Span,
}

/// When case
#[derive(Debug, Clone)]
pub struct WhenCase {
    pub pattern: WhenPattern,
    pub body: Expression,
    pub span: Span,
}

/// When pattern
#[derive(Debug, Clone)]
pub enum WhenPattern {
    Literal(Literal),
    Identifier(String),
    Else,
}

/// Call expression
#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: Expression,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

/// Member access expression
#[derive(Debug, Clone)]
pub struct MemberAccessExpression {
    pub object: Expression,
    pub property: String,
    pub span: Span,
}

/// Await expression
#[derive(Debug, Clone)]
pub struct AwaitExpression {
    pub expression: Expression,
    pub span: Span,
}

/// Ask JavaScript expression
#[derive(Debug, Clone)]
pub struct AskJsExpression {
    pub code: String,
    pub span: Span,
}

/// List literal
#[derive(Debug, Clone)]
pub struct ListLiteral {
    pub elements: Vec<Expression>,
    pub span: Span,
}

/// Map literal
#[derive(Debug, Clone)]
pub struct MapLiteral {
    pub pairs: Vec<(Expression, Expression)>,
    pub span: Span,
}

/// Enum definition
#[derive(Debug, Clone)]
pub struct EnumDefinition {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

/// Enum variant
#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub span: Span,
}