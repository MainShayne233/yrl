#[derive(Debug, Clone, PartialEq)]
pub struct Grammar {
    pub declarations: Vec<Declaration>,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub lhs: String,
    pub rhs: Vec<String>,
    pub expressions: Vec<NodeExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeExpression {
    Integer {
        value: i64,
    },
    Atom {
        value: String,
    },
    Charlist {
        value: String,
    },
    Tuple {
        values: Box<Vec<NodeExpression>>,
    },
    FunctionCall {
        name: String,
        args: Box<Vec<NodeExpression>>,
    },
    List {
        values: Box<Vec<NodeExpression>>,
    },
    HeadTailList {
        head: Box<Vec<NodeExpression>>,
        tail: Box<NodeExpression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration(pub DeclarationType, pub Vec<String>);

#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationType {
    Nonterminals,
    Terminals,
    Rootsymbol,
    Expect,
    Left,
    Right,
    Nonassoc,
}
