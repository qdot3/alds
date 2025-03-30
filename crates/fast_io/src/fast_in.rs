use std::{
    io::{stdin, BufRead, StdinLock},
    result,
};

use crate::FromBytes;

// TODO: make faster. this is too late.
pub struct FastInput<'a> {
    source: StdinLock<'a>,
    concat_buf: Vec<u8>,
}

impl<'a> FastInput<'a> {
    pub fn new() -> Self {
        Self {
            source: stdin().lock(),
            concat_buf: Vec::new(),
        }
    }

    pub fn next_token<T>(&mut self) -> T::Output
    where
        T: FromBytes,
    {
        let (mut l, mut r) = (0, 0);
        while let Ok(bytes) = self.source.fill_buf() {
            if bytes.is_empty() {
                let result = T::from_bytes(&self.concat_buf);
                self.concat_buf.clear();
                return result;
            }
            if !self.concat_buf.is_empty() && bytes[l].is_ascii_whitespace() {
                let result = T::from_bytes(&self.concat_buf);
                self.concat_buf.clear();
                return result;
            }
            while l < bytes.len() {
                if bytes[l].is_ascii_whitespace() {
                    l += 1
                }
            }
            r += l;
            while r < bytes.len() {
                if !bytes[r].is_ascii_whitespace() {
                    r += 1
                }
            }

            if r == bytes.len() {
                self.concat_buf.extend_from_slice(&bytes[l..]);
                self.source.consume(r);
                (l, r) = (0, 0);
                continue;
            } else if self.concat_buf.is_empty() {
                let result = T::from_bytes(&bytes[l..r]);
                self.source.consume(r);
                return result;
            } else {
                self.concat_buf.extend_from_slice(&bytes[..r]);
                self.source.consume(r);

                let result = T::from_bytes(&self.concat_buf);
                self.concat_buf.clear();
                return result;
            }
        }
        todo!("I/O error handling")
    }
}
