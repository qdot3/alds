use std::io::{stdin, StdinLock};


pub struct FastInput<'a> {
    source: StdinLock<'a>,
}

impl<'a> FastInput<'a> {
    pub fn new() -> Self {
        Self {
            source: stdin().lock(),
        }
    }

    pub fn next_token(&mut self) -> &[u8] {
        todo!()
    }
}
