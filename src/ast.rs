#[derive(Debug, Clone, PartialEq)]
pub struct Grammar {
    pub declarations: Vec<Declaration>,
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
