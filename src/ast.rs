// expression → literal
//            | unary
//            | binary
//            | grouping
//
// literal    → NUMBER | STRING | "true" | "false" | "nil"
// grouping   → "(" expression ")"
// unary      → ( "-" | "!" ) expression
// binary     → expression operator expression
// operator   → "==" | "!=" | "<" | "<=" | ">" | ">="
//            | "+"  | "-"  | "*" | "/"
//

use crate::binary_tree::BinaryTree;
use crate::lexer::Token;

type Ast<'a> = BinaryTree<Token<'a>>;
