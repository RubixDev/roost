use rust_decimal::Decimal;
use crate::error::Location;

#[derive(Debug, PartialEq, Clone)]
pub struct Statements {
    pub start: Location,
    pub end: Location,
    pub statements: Vec<Statement>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Declare   (DeclareStatement),
    Assign    (AssignStatement),
    If        (IfStatement),
    Loop      (LoopStatement),
    While     (WhileStatement),
    For       (ForStatement),
    Function  (FunctionDeclaration),
    Expression(Expression),
    Break,
    Continue,
    Return    (ReturnStatement),
}
#[derive(Debug, PartialEq, Clone)]
pub struct DeclareStatement {
    pub start: Location,
    pub end: Location,
    pub identifier: String,
    pub expression: Expression,
}
#[derive(Debug, PartialEq, Clone)]
pub struct AssignStatement {
    pub start: Location,
    pub end: Location,
    pub identifier: String,
    pub operator: AssignOperator,
    pub expression: Expression,
}
#[derive(Debug, PartialEq, Clone)]
pub enum AssignOperator {
    Normal,    // =
    Plus,      // +=
    Minus,     // -=
    Multiply,  // *=
    Divide,    // /=
    Modulo,    // %=
    IntDivide, // \=
    Power,     // **=
}
#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub start: Location,
    pub end: Location,
    pub condition: Expression,
    pub block: Statements,
    pub else_block: Option<Statements>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct LoopStatement {
    pub start: Location,
    pub end: Location,
    pub block: Statements,
}
#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub start: Location,
    pub end: Location,
    pub condition: Expression,
    pub block: Statements,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub start: Location,
    pub end: Location,
    pub identifier: String,
    pub expression: Expression,
    pub block: Statements,
}
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub start: Location,
    pub end: Location,
    pub identifier: String,
    pub params: Vec<String>,
    pub block: Statements,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub start: Location,
    pub end: Location,
    pub expression: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub start: Location,
    pub end: Location,
    pub base: Box<TernaryExpression>,
    pub range: Box<Option<(bool, TernaryExpression)>>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct TernaryExpression {
    pub start: Location,
    pub end: Location,
    pub base: OrExpression,
    pub ternary: Option<(Expression, Expression)>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct OrExpression {
    pub start: Location,
    pub end: Location,
    pub base: AndExpression,
    pub following: Vec<AndExpression>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct AndExpression {
    pub start: Location,
    pub end: Location,
    pub base: EqualityExpression,
    pub following: Vec<EqualityExpression>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum EqualityOperator {
    Equal,    // ==
    NotEqual, // !=
}
#[derive(Debug, PartialEq, Clone)]
pub struct EqualityExpression {
    pub start: Location,
    pub end: Location,
    pub base: RelationalExpression,
    pub other: Option<(EqualityOperator, RelationalExpression)>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum RelationalOperator {
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
}
#[derive(Debug, PartialEq, Clone)]
pub struct RelationalExpression {
    pub start: Location,
    pub end: Location,
    pub base: AdditiveExpression,
    pub other: Option<(RelationalOperator, AdditiveExpression)>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum AdditiveOperator {
    Plus,  // +
    Minus, // -
}
#[derive(Debug, PartialEq, Clone)]
pub struct AdditiveExpression {
    pub start: Location,
    pub end: Location,
    pub base: MultiplicativeExpression,
    pub following: Vec<(AdditiveOperator, MultiplicativeExpression)>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum MultiplicativeOperator {
    Multiply,  // *
    Divide,    // /
    Modulo,    // %
    IntDivide, // \
}
#[derive(Debug, PartialEq, Clone)]
pub struct MultiplicativeExpression {
    pub start: Location,
    pub end: Location,
    pub base: UnaryExpression,
    pub following: Vec<(MultiplicativeOperator, UnaryExpression)>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Plus,  // +
    Minus, // -
    Not,   // !
}
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryExpression {
    Operator {
        start: Location,
        end: Location,
        operator: UnaryOperator,
        expression: Box<UnaryExpression>,
    },
    Power(Box<ExponentialExpression>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ExponentialExpression {
    pub start: Location,
    pub end: Location,
    pub base: Atom,
    pub exponent: Option<UnaryExpression>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(Decimal),
    Bool(bool),
    String(String),
    Identifier { start: Location, end: Location, name: String },
    Call(CallExpression),
    Null,
    Expression(Expression),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub start: Location,
    pub end: Location,
    pub identifier: String,
    pub args: Vec<Expression>,
}
