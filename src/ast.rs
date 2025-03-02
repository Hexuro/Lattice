#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

pub trait Stmt {
    fn kind(&self) -> NodeType;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Box<dyn Stmt>>,
}

impl Stmt for Program {
    fn kind(&self) -> NodeType {
        self.kind.clone()
    }
}

pub trait Expr: Stmt {}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: String,
}

impl Stmt for BinaryExpr {
    fn kind(&self) -> NodeType {
        self.kind.clone()
    }
}

impl Expr for BinaryExpr {}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String,
}

impl Stmt for Identifier {
    fn kind(&self) -> NodeType {
        self.kind.clone()
    }
}

impl Expr for Identifier {}

#[derive(Debug, Clone, PartialEq)]
pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: f64,
}

impl Stmt for NumericLiteral {
    fn kind(&self) -> NodeType {
        self.kind.clone()
    }
}

impl Expr for NumericLiteral {}
