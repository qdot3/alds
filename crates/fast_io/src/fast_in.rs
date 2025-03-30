use std::io::{stdin, BufRead, StdinLock};

use super::FromBytes;

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
        // token may be very long
        while let Ok(bytes) = self.source.fill_buf() {
            // EOF
            if bytes.is_empty() {
                let result = T::from_bytes(&self.concat_buf);
                self.concat_buf.clear();
                return result;
            }

            let start = bytes.iter().take_while(|b| !b.is_ascii_graphic()).count();
            if start == bytes.len() {
                self.source.consume(start);
                continue;
            } else if start != 0 && !self.concat_buf.is_empty() {
                self.source.consume(start);
                let result = T::from_bytes(&self.concat_buf);
                self.concat_buf.clear();
                return result;
            }

            let end = start
                + bytes[start..]
                    .iter()
                    .take_while(|b| b.is_ascii_graphic())
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

pub enum Token<'a> {
    Slice(&'a [u8]),
    Bytes(Vec<u8>),
}

pub struct Tokenizer<'a> {
    source: StdinLock<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new() -> Self {
        Self {
            source: stdin().lock(),
        }
    }

    pub fn next_token<T>(&mut self) -> T::Output
    where
        T: FromBytes,
    {
        // skip ascii non-graphic characters (= separators)
        while let Ok(bytes) = self.source.fill_buf() {
            // EOF
            if bytes.is_empty() {
                return T::from_bytes(b"");
            }

            let mut i = 0;
            let len = bytes.len();
            while i < len && !bytes[i].is_ascii_graphic() {
                i += 1;
            }
            self.source.consume(i);

            if i < len {
                break;
            }
        }

        let mut buf = Vec::new();
        while let Ok(bytes) = self.source.fill_buf() {
            // EOF
            if bytes.is_empty() {
                return T::from_bytes(&buf);
            }

            let mut i = 0;
            let len = bytes.len();
            while i < len && bytes[i].is_ascii_graphic() {
                i += 1;
            }
            if i < len {
                if buf.is_empty() || i == 0 {
                    let result = T::from_bytes(&bytes[..i]);
                    self.source.consume(i);
                    return result;
                } else {
                    buf.extend_from_slice(&bytes[..i]);
                    self.source.consume(i);
                    return T::from_bytes(&buf);
                }
            } else {
                buf.extend_from_slice(&bytes);
                self.source.consume(len);
            }
        }
        unreachable!()
    }
}
