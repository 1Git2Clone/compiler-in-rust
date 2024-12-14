use crate::prelude::*;

pub mod ast;
pub mod parse;
pub mod tokenize;

/// Evaluate expressions based on an [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
///
///
/// ```rust
/// use compiler::prelude::*;
/// use compiler::run::compile;
///
/// fn compute(str: &str) -> i32 {
///     compile(&parse(&tokenize(str).unwrap()).unwrap().unwrap()).unwrap()
/// }
/// assert_eq!(compute("3 + 2 * 6"), 15);
/// assert_eq!(compute("(3 + 2) * 6"), 30);
/// // No, this compiler isn't smart enough to handle nested parentheses.
/// //
/// // assert_eq!(compute("(4 + (2 * 3)) * 6"), 60);
/// ```
pub fn compile(ast: &Ast) -> Result<i32, Error> {
    match ast {
        Ast::Int(val) => Ok(*val),
        Ast::BinOp { lhs, op, rhs } => {
            let lhs_val = compile(lhs)?;
            let rhs_val = compile(rhs)?;
            match op {
                Token::Plus => Ok(lhs_val + rhs_val),
                Token::Minus => Ok(lhs_val - rhs_val),
                Token::Mul => Ok(lhs_val * rhs_val),
                Token::Div => Ok(lhs_val / rhs_val),
                Token::Lte => Ok((lhs_val >= rhs_val) as i32),
                Token::Lt => Ok((lhs_val < rhs_val) as i32),
                Token::Gte => Ok((lhs_val >= rhs_val) as i32),
                Token::Gt => Ok((lhs_val > rhs_val) as i32),
                Token::Eq => Ok((lhs_val == rhs_val) as i32),
                _ => Err(format!("Invalid token: {:?}.", op).into()),
            }
        }
    }
}
