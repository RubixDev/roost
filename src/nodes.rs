use std::{ops::Deref, slice};

use crate::{error::Location, tokens::TokenType};
use rust_decimal::Decimal;

macro_rules! node {
    ($name:ident; $($field:ident : $type:ty),* $(,)?) => {
        #[derive(Debug)]
        pub struct $name<'f> {
            pub start: Location<'f>,
            pub end: Location<'f>,
            $(
                pub $field: $type,
            )*
        }
    };
}

/////////////////////////////////////////////

type Identifier = String;
type Opt<T> = Option<T>;

#[derive(Debug)]
pub struct Rep<T>(Option<Vec<T>>);

impl<T> From<Option<Vec<T>>> for Rep<T> {
    fn from(val: Option<Vec<T>>) -> Self {
        Self(val)
    }
}

impl<T> Deref for Rep<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            Some(vec) => vec,
            None => &[],
        }
    }
}

impl<'a, T> IntoIterator for &'a Rep<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/////////////////////////////////////////////

pub type Statements<'f> = Rep<Statement<'f>>;

#[derive(Debug)]
pub enum Block<'f> {
    Single(Box<Statement<'f>>),
    Multiple(BlockExpr<'f>),
}

#[derive(Debug)]
pub enum Statement<'f> {
    Var(VarStmt<'f>),
    Function(FunctionDecl<'f>),
    Class(ClassDecl<'f>),
    Break(BreakStmt<'f>),
    Continue(ContinueStmt<'f>),
    Return(ReturnStmt<'f>),
    Expr(Expression<'f>),
}

node! {
    VarStmt;
    ident: Identifier,
    expr: Expression<'f>,
}

node! {
    FunctionDecl;
    ident: Identifier,
    args: ArgNames,
    block: Block<'f>,
}

node! {
    ClassDecl;
    ident: Identifier,
    block: MemberBlock<'f>,
}

node! {
    BreakStmt;
    expr: Expression<'f>,
}

node! { ContinueStmt; }

node! {
    ReturnStmt;
    expr: Expression<'f>,
}

node! {
    Member;
    is_static: bool,
    member_type: MemberType<'f>,
}

#[derive(Debug)]
pub enum MemberType<'f> {
    Attribute(VarStmt<'f>),
    Method(FunctionDecl<'f>),
}

node! {
    MemberBlock;
    members: Rep<Member<'f>>,
}

pub type Expression<'f> = RangeExpr<'f>;

node! {
    RangeExpr;
    base: Box<OrExpr<'f>>,
    range: Opt<(TokenType, Box<OrExpr<'f>>)>,
}

node! {
    OrExpr;
    base: AndExpr<'f>,
    following: Rep<AndExpr<'f>>,
}

node! {
    AndExpr;
    base: BitOrExpr<'f>,
    following: Rep<BitOrExpr<'f>>,
}

node! {
    BitOrExpr;
    base: BitXorExpr<'f>,
    following: Rep<BitXorExpr<'f>>,
}

node! {
    BitXorExpr;
    base: BitAndExpr<'f>,
    following: Rep<BitAndExpr<'f>>,
}

node! {
    BitAndExpr;
    base: EqExpr<'f>,
    following: Rep<EqExpr<'f>>,
}

node! {
    EqExpr;
    left: RelExpr<'f>,
    right: Opt<(TokenType, RelExpr<'f>)>,
}

node! {
    RelExpr;
    left: ShiftExpr<'f>,
    right: Opt<(TokenType, ShiftExpr<'f>)>,
}

node! {
    ShiftExpr;
    base: AddExpr<'f>,
    following: Rep<(TokenType, AddExpr<'f>)>,
}

node! {
    AddExpr;
    base: MulExpr<'f>,
    following: Rep<(TokenType, MulExpr<'f>)>,
}

node! {
    MulExpr;
    base: UnaryExpr<'f>,
    following: Rep<(TokenType, UnaryExpr<'f>)>,
}

#[derive(Debug)]
pub enum UnaryExpr<'f> {
    Unary {
        start: Location<'f>,
        end: Location<'f>,
        operator: TokenType,
        expr: Box<UnaryExpr<'f>>,
    },
    Done(Box<ExpExpr<'f>>),
}

node! {
    ExpExpr;
    base: AssignExpr<'f>,
    exponent: Opt<UnaryExpr<'f>>,
}

node! {
    AssignExpr;
    left: CallExpr<'f>,
    right: Opt<(TokenType, Expression<'f>)>,
}

node! {
    CallExpr;
    base: MemberExpr<'f>,
    following: Opt<(Args<'f>, Rep<CallPart<'f>>)>,
}

node! {
    MemberExpr;
    base: Atom<'f>,
    following: Rep<MemberPart>,
}

#[derive(Debug)]
pub enum Atom<'f> {
    Number(Decimal),
    Bool(bool),
    String(String),
    Null,
    Identifier {
        start: Location<'f>,
        end: Location<'f>,
        name: Identifier,
    },
    Expr(Expression<'f>),
    IfExpr(IfExpr<'f>),
    ForExpr(ForExpr<'f>),
    WhileExpr(WhileExpr<'f>),
    LoopExpr(LoopExpr<'f>),
    FunExpr(FunExpr<'f>),
    ClassExpr(ClassExpr<'f>),
    TryExpr(TryExpr<'f>),
    BlockExpr(BlockExpr<'f>),
}

node! {
    IfExpr;
    condition: Expression<'f>,
    block: Block<'f>,
    else_block: Opt<Block<'f>>,
}

node! {
    ForExpr;
    ident: Identifier,
    iter: Expression<'f>,
    block: Block<'f>,
}

node! {
    WhileExpr;
    condition: Expression<'f>,
    block: Block<'f>,
}

node! {
    LoopExpr;
    block: Block<'f>,
}

node! {
    FunExpr;
    args: ArgNames,
    block: Block<'f>,
}

node! {
    ClassExpr;
    block: MemberBlock<'f>,
}

node! {
    TryExpr;
    try_block: Block<'f>,
    ident: Identifier,
    catch_block: Block<'f>,
}

node! {
    BlockExpr;
    stmts: Statements<'f>,
    expr: Opt<Expression<'f>>,
}

#[derive(Debug)]
pub enum MemberPart {
    Field(Identifier),
}
#[derive(Debug)]
pub enum CallPart<'f> {
    Member(MemberPart),
    Args(Args<'f>),
}
type Args<'f> = Rep<Expression<'f>>;
type ArgNames = Rep<Identifier>;
