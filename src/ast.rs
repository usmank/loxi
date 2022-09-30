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
    Grouping(Box<Expression<T>>),
}

impl<T: fmt::Display> Expression<T> {
    fn stringify(&self, indent: usize) -> String {
        let spacer = format!("{:.<indent$} ", "");
        let nested_indent = indent + 4;

        match self {
            Expression::Literal(value) => format!("{spacer}{value}"),
            Expression::Unary { operator, right } => {
                format!("{spacer}{operator}\n{}", right.stringify(nested_indent))
            }
            Expression::Binary {
                operator,
                left,
                right,
            } => {
                format!(
                    "{spacer}{operator}\n{}\n{}",
                    left.stringify(nested_indent),
                    right.stringify(nested_indent)
                )
            }
            Expression::Grouping(expression) => {
                format!("{spacer}grouping\n{}", expression.stringify(nested_indent))
            }
        }
    }
}

impl<T: fmt::Display> fmt::Display for Expression<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify(0))
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

        // Just check the number of lines in output.
        assert_eq!(output.lines().count(), 4);
    }
}
