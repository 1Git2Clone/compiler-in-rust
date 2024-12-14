use crate::prelude::*;
use core::slice;
use std::iter::Peekable;

/// Parse the tokens into an abstract syntax tree.
///
/// ```rust
/// use compiler::prelude::*;
/// use compiler::run::compile;
///
/// fn compute(str: &str) -> Ast {
///     parse(&tokenize(str).unwrap()).unwrap().unwrap()
/// }
/// assert_eq!(
///     compute("3 + 2 * 6"),
///     Ast::BinOp {
///         lhs: Box::new(Ast::Int(3)),
///         op: Token::Plus,
///         rhs: Box::new(Ast::BinOp {
///             lhs: Box::new(Ast::Int(2)),
///             op: Token::Mul,
///             rhs: Box::new(Ast::Int(6))
///         })
///     }
/// );
/// assert_eq!(
///     compute("(3 + 2) * 6"),
///     Ast::BinOp {
///         lhs: Box::new(Ast::BinOp {
///             lhs: Box::new(Ast::Int(3)),
///             op: Token::Plus,
///             rhs: Box::new(Ast::Int(2))
///         }),
///         op: Token::Mul,
///         rhs: Box::new(Ast::Int(6)),
///     }
/// );
/// ```
pub fn parse(tokens: &[Token]) -> Result<Option<Ast>, Error> {
    let mut peekable_tokens = tokens.iter().peekable();
    let mut res: Option<Ast> = None;

    let get_tokens_in_parentheses =
        |peekable_tokens: &mut Peekable<slice::Iter<'_, Token>>| -> Result<Vec<Token>, Error> {
            let mut res = vec![Token::LeftParen];
            peekable_tokens
                .clone()
                .take_while(|t| !matches!(t, Token::RightParen))
                .for_each(|t| res.push(t.clone()));
            for _ in 0..res.len() - 1 {
                peekable_tokens.next();
            }
            let Some(closing) = peekable_tokens.next() else {
                return Err("No matching closing parenthesis".into());
            };
            #[cfg(feature = "debug")]
            println!("Closing token = ({:?})", closing);

            res.push(closing.clone());

            #[cfg(feature = "debug")]
            println!("Paren tokens: {:#?}", res);
            if !valid_paren(&res) {
                return Err("Invalid parenthesis".into());
            }
            if res.len() == 2 {
                return Err("Empty parenthesis".into());
            }

            Ok(res)
        };

    let get_rhs = |res: &mut Option<Ast>,
                   tok: &Token,
                   peekable_tokens: &mut Peekable<slice::Iter<'_, Token>>|
     -> Result<Ast, Error> {
        let Some(rhs_tok) = peekable_tokens.next() else {
            return Err(format!(
                "Couldn't parse Rhs for token {:?} with {:?} as the lhs",
                tok, res
            )
            .into());
        };
        let rhs = match rhs_tok {
            Token::Int(x) => {
                if let Some(next_tok_op) = peekable_tokens.peek() {
                    match next_tok_op {
                        Token::Mul | Token::Div => {
                            let op = peekable_tokens.next().unwrap();
                            let rhs = match peekable_tokens.next() {
                                Some(Token::Int(x)) => Ast::Int(*x),
                                Some(Token::LeftParen) => {
                                    let tokens = get_tokens_in_parentheses(peekable_tokens)?;
                                    parse(&tokens[1..tokens.len() - 1])?.unwrap()
                                }
                                None => return Err("Expected rhs for BinOp".into()),
                                _ => return Err("Invalid rhs token for BinOp".into()),
                            };
                            Ast::BinOp {
                                lhs: Box::new(Ast::Int(*x)),
                                op: op.clone(),
                                rhs: Box::new(rhs),
                            }
                        }
                        _ => Ast::Int(*x),
                    }
                } else {
                    Ast::Int(*x)
                }
            }
            Token::LeftParen => {
                let tokens = get_tokens_in_parentheses(peekable_tokens)?;
                parse(&tokens[1..tokens.len() - 1])?.unwrap()
            }
            _ => {
                return Err(format!("{:?} is an invalid rhs token for Ast::BinOp", rhs_tok).into());
            }
        };
        Ok(rhs)
    };

    while let Some(tok) = peekable_tokens.next() {
        #[cfg(feature = "debug")]
        println!("Token({:?})", tok);
        match tok {
            Token::RightParen => {
                return Err("Unexpected closing parenthesis".into());
            }
            Token::LeftParen => {
                let tokens = get_tokens_in_parentheses(&mut peekable_tokens)?;
                if res.is_none() {
                    res = parse(&tokens[1..tokens.len() - 1])?
                }
            }
            Token::Int(x) if res.is_none() => res = Some(Ast::Int(*x)),
            Token::Int(_) => return Err("Unmatched Int token".into()),
            Token::Mul | Token::Div | Token::Plus | Token::Minus => {
                let rhs = match res {
                    Some(_) => Some(get_rhs(&mut res, tok, &mut peekable_tokens)?),
                    None => None,
                };
                res = res.map(|some| Ast::BinOp {
                    lhs: Box::new(some),
                    op: tok.clone(),
                    //           |This is safe|
                    rhs: Box::new(rhs.unwrap()),
                });
            }
            // TODO: Implement comparison operators.
            _ => todo!(),
        };
    }

    Ok(res)
}
