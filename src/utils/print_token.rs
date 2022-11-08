use crate::token::Token;

pub fn pr(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{token:?}");
    }
}
