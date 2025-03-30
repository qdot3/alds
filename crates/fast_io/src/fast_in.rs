use std::io::{stdin, BufRead, StdinLock};

use crate::FromBytes;

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
        // token may be very long
        while let Ok(bytes) = self.source.fill_buf() {
            // EOF
            if bytes.is_empty() {
                let result = T::from_bytes(&self.concat_buf);
                self.concat_buf.clear();
                return result;
            }

            let start = bytes.iter().take_while(|b| b.is_ascii_whitespace()).count();
            if start == bytes.len() {
                self.source.consume(start);
                continue;
            } else if start != 0 && !self.concat_buf.is_empty() {
                let result = T::from_bytes(&self.concat_buf);
                self.concat_buf.clear();
                return result;
            }

            let end = start
                + bytes[start..]
                    .iter()
                    .take_while(|b| !b.is_ascii_whitespace())
                    .count();
            // token may be partitioned
            if end == bytes.len() {
                self.concat_buf.extend_from_slice(&bytes[start..]);
                self.source.consume(end);
                continue;
            }

            let result = if self.concat_buf.is_empty() {
                let result = T::from_bytes(&bytes[start..end]);
                self.source.consume(end);
                result
            } else {
                // token is partitioned
                debug_assert_eq!(start, 0);
                self.concat_buf.extend_from_slice(&bytes[..end]);
                self.source.consume(end);

                let result = T::from_bytes(&self.concat_buf);
                self.concat_buf.clear();
                result
            };
            return result;
        }

        todo!("error handling")
    }
}
