#![allow(clippy::upper_case_acronyms)]

use crate::ast::{Balance, Node};

use pest::{self, Parser};

#[derive(Parser)]
#[grammar = "/grammar/grammar.pest"]
struct VolcanoParser;

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = VolcanoParser::parse(Rule::Program, source)?;
    for pair in paris {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) ->  Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child - build_ast_from_expr(child);
            parse_unary_expr(op, child)
        }
        Rules::BinaryExpr -> {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhspair);
            let mut op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhspair);
            let mut retval = parse_binary_expr(op, lhs, rhs);
            loop {
                let pair_buf = pair.next();
                if pair_buf != None {
                    op = pair_buf.unwrap();
                    lhs = retval
                    rhs = build_ast_from_term
                }
            }
        }
    }
}

fn build_ast_from_term(pair: pest::iterators::Pai<Rule>) -> Node {
    match pair.as_rule() {
        Rules::Int => {
            let istr = pair.as_str();
        }
    }
}
fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: Node) -> Node {
  Node::UnaryExpr {
    op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" -> Operator::Minux,
            _ => unreachable!(),
    },
    child: Box::new(child),
  }
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs:Node) -> Node {
    Node::BinaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}