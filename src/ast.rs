use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Balance {
    FetchAddress {
        address: String,
        network: String
    },
    Conc {
        nodes: Vec<Balance>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// ANCHOR: operator
pub enum Operator {
    Plus,
    Minus,
}


impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Balance::FetchAddress { address, network } => write!(f, "FetchAddress {{ address: {}, network: {} }}", address, network),
            Balance::Conc { nodes } => write!(f, "Concatenate {{ nodes: [{}] }}", nodes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","")),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Int(i32),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Int(n) => write!(f, "{}", n),
            Node::UnaryExpr { op, child } => write!(f, "{}{}", op, child),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}
