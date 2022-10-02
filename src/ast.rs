use std::fmt;

pub enum Expression<T> {
    Literal(LiteralValue),
    Unary {
        operator: T,
        right: Box<Expression<T>>,
    },
    Binary {
        operator: T,
        left: Box<Expression<T>>,
        right: Box<Expression<T>>,
    },
    Ternary {
        operator: T,
        left: Box<Expression<T>>,
        middle: Box<Expression<T>>,
        right: Box<Expression<T>>,
    },
    Grouping(Box<Expression<T>>),
}

impl<T: fmt::Display> fmt::Display for Expression<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Literal(value) => write!(f, "{value}"),
            Expression::Unary { operator, right } => write!(f, "({operator} {right})"),
            Expression::Binary {
                operator,
                left,
                right,
            } => write!(f, "({operator} {left} {right})"),
            Expression::Ternary {
                operator,
                left,
                middle,
                right,
            } => write!(f, "({operator} {left} {middle} {right})"),
            Expression::Grouping(expression) => write!(f, "(group {expression})"),
        }
    }
}

pub enum LiteralValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl<'a> fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Number(value) => write!(f, "{value}"),
            LiteralValue::String(value) => write!(f, "{value}"),
            LiteralValue::True => write!(f, "true"),
            LiteralValue::False => write!(f, "false"),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_expression() {
        let num_expr = Box::new(Expression::Literal(LiteralValue::Number(3.14)));
        let num_expr_2 = Box::new(Expression::Literal(LiteralValue::Number(6.28)));
        let minus_op = "-";
        let mul_op = "*";
        let negate_expr = Box::new(Expression::Unary {
            operator: minus_op,
            right: num_expr_2,
        });
        let expr = Expression::Binary {
            operator: mul_op,
            left: num_expr,
            right: negate_expr,
        };

        let output: String = format!("{expr}");

        assert_eq!(output, "(* 3.14 (- 6.28))");
    }
}
