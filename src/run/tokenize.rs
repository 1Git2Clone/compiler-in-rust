use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    // --------------------------------------------------------------------------------------------
    // Data types
    // --------------------------------------------------------------------------------------------
    Int(i32),
    // --------------------------------------------------------------------------------------------
    // Operators
    // --------------------------------------------------------------------------------------------
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Mul,
    /// `/`
    Div,
    // --------------------------------------------------------------------------------------------
    // Comparison operators
    // --------------------------------------------------------------------------------------------
    /// `==`
    Eq,
    /// `<=`
    Lte,
    /// `<`
    Lt,
    /// `>=`
    Gte,
    /// `>`
    Gt,
    /// `!`
    Not,
    // --------------------------------------------------------------------------------------------
    // Parenthesis for order of operations
    // --------------------------------------------------------------------------------------------
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    // --------------------------------------------------------------------------------------------
    // End of statement
    // --------------------------------------------------------------------------------------------
    /// `;`
    EndOfStmt,
}

/// Get all the tokens from a string slice.
///
/// ```rust
/// use compiler::prelude::*;
/// use compiler::run::compile;
///
/// fn compute(str: &str) -> Vec<Token> {
///     tokenize(str).unwrap()
/// }
///
/// assert_eq!(
///     compute("3121254 + 1451252"),
///     vec![
///         Token::Int(3121254),
///         Token::Plus,
///         Token::Int(1451252),
///     ]
/// );
/// assert_eq!(
///     compute("3 + 2 * 6"),
///     vec![
///         Token::Int(3),
///         Token::Plus,
///         Token::Int(2),
///         Token::Mul,
///         Token::Int(6),
///     ]
/// );
/// assert_eq!(
///     compute("(3 + 2) * 6"),
///     vec![
///         Token::LeftParen,
///         Token::Int(3),
///         Token::Plus,
///         Token::Int(2),
///         Token::RightParen,
///         Token::Mul,
///         Token::Int(6),
///     ]
/// );
/// assert_eq!(
///     compute("\n\t  (  3 + 2\t) \n* \t   6   "),
///     vec![
///         Token::LeftParen,
///         Token::Int(3),
///         Token::Plus,
///         Token::Int(2),
///         Token::RightParen,
///         Token::Mul,
///         Token::Int(6)
///     ]
/// );
/// ```
pub fn tokenize(src: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::<Token>::new();

    let mut chars = src.chars().peekable();
    while let Some(c) = chars.next() {
        #[cfg(feature = "debug")]
        println!("At: '{}'", c);
        let token = match c {
            '0'..='9' => {
                let res = String::from(c)
                    + &chars
                        .clone()
                        .take_while(|c| c.is_numeric())
                        .collect::<String>();
                // `.advance_by()` is unstable | #77404
                // chars.advance_by(res.len() - 1);
                for _ in 0..res.len() - 1 {
                    chars.next();
                }
                Token::Int(res.parse::<i32>().unwrap())
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mul,
            '/' => Token::Div,
            '<' if chars.peek().is_some_and(|c| *c == '=') => Token::Lte,
            '<' => Token::Lt,
            '>' if chars.peek().is_some_and(|c| *c == '=') => Token::Gte,
            '>' => Token::Gt,
            '=' if chars.peek().is_some_and(|c| *c == '=') => Token::Eq,
            '!' => Token::Not,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ';' => Token::EndOfStmt,
            _ => continue,
        };
        tokens.push(token);
    }

    Ok(tokens)
}
