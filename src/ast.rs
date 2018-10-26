// expression → literal
// | unary
// | binary
// | grouping
//
// literal    → NUMBER | STRING | "true" | "false" | "nil"
// grouping   → "(" expression ")"
// unary      → ( "-" | "!" ) expression
// binary     → expression operator expression
// operator   → "==" | "!=" | "<" | "<=" | ">" | ">="
// | "+"  | "-"  | "*" | "/"
//

use binary_tree::{NodeId, NodeArena};
use lexer::Token;

#[derive(Debug)]
pub struct Ast<'a> {
    arena: NodeArena<Token<'a>>,
    root: AstNode,
}

#[derive(Debug)]
pub enum AstNode {
    Unary(NodeId),
    Binary(NodeId),
    Literal(NodeId),
}

impl<'a> Ast<'a> {
    pub fn new(root: AstNode) -> Ast<'a> {
        Ast {
            arena: NodeArena::new(),
            root: root,
        }
    }

    pub fn new_unary_node(&mut self, operator: Token<'a>, right: NodeId) -> AstNode {
        let new_node_id = self.arena.new_node(operator, None, Some(right));
        AstNode::Unary(new_node_id)
    }

    pub fn new_binary_node(&mut self, operator: Token<'a>, left: NodeId, right: NodeId) -> AstNode {
        let new_node_id = self.arena.new_node(operator, Some(left), Some(right));
        AstNode::Binary(new_node_id)
    }

    pub fn new_literal_node(&mut self, operator: Token<'a>) -> AstNode {
        let new_node_id = self.arena.new_node(operator, None, None);
        AstNode::Literal(new_node_id)
    }
}
