use crate::prelude::*;

pub fn valid_paren(tok: &[Token]) -> bool {
    let mut stack = vec![];

    for t in tok {
        match t {
            Token::LeftParen => stack.push(Token::RightParen),
            Token::RightParen if stack.pop().is_some_and(|paren| &paren != t) => {
                return false;
            }
            _ => (),
        }
    }

    stack.is_empty()
}
