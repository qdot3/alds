use std::{
    fmt::Debug,
    io::{self, BufRead, ErrorKind},
};

use super::FromBytes;

/// A wrapper of [BufReader](std::io::BufReader).
pub struct FastInput<const N: usize, R: BufRead> {
    reader: R,
    buf: [u8; N],
    pos: usize,
    filled: usize,
}

impl<const N: usize, R: BufRead> FastInput<N, R> {
    /// Cheats a new buffered handler of the given reader.
    pub fn new(mut reader: R) -> Self {
        assert!(N != 0);

        let mut buf = [0; N];
        if let Ok(n) = reader.read(&mut buf) {
            FastInput {
                reader,
                buf,
                pos: 0,
                filled: n,
            }
        } else {
            FastInput {
                reader,
                buf,
                pos: 0,
                filled: 0,
            }
        }
    }

    /// # Panics
    ///
    /// Panics if
    ///
    /// 1. no more data is found, or
    /// 2. parsing fails.
    pub fn parse_unwrap<T>(&mut self) -> T
    where
        T: FromBytes,
        T::Err: Debug,
    {
        let token = self.next_token().unwrap();
        T::from_bytes(token.as_slice()).unwrap()
    }

    pub fn next_token(&mut self) -> io::Result<Token> {
        let mut buf = Vec::new();
        // The loop usually completes in a single iteration
        if let Some(skip) = self.buf[self.pos..self.filled]
            .iter()
            .position(|b| b.is_ascii_graphic())
        {
            self.pos += skip;
            if let Some(n) = self.buf[self.pos..self.filled]
                .iter()
                .position(|b| !b.is_ascii_graphic())
            {
                self.pos += n;
                return Ok(Token::Slice(&self.buf[self.pos - n..self.pos]));
            } else {
                buf.extend_from_slice(&self.buf[self.pos..self.filled])
            }
        }
        self.pos = self.filled;

        // Otherwise, the loop usually completes within two iterations.
        if self.cold_next_token_2(&mut buf)? {
            return Ok(Token::Bytes(buf));
        } else {
            return self.cold_next_token_3_or_more(buf);
        }
    }

    #[cold]
    #[inline(never)]
    fn cold_next_token_2(&mut self, buf: &mut Vec<u8>) -> io::Result<bool> {
        debug_assert_eq!(self.pos, self.filled);

        self.filled = self.reader.read(&mut self.buf)?;
        self.pos = 0;

        // EOF or empty
        if self.filled == 0 {
            return Err(io::Error::new(ErrorKind::Other, "no more data"));
        }

        if let Some(skip) = self.buf[..self.filled]
            .iter()
            .position(|b| b.is_ascii_graphic())
        {
            self.pos += skip;
            if !buf.is_empty() && skip > 0 {
                return Ok(true);
            }

            if let Some(n) = self.buf[self.pos..self.filled]
                .iter()
                .position(|b| !b.is_ascii_graphic())
            {
                self.pos += n;
                buf.extend_from_slice(&self.buf[self.pos - n..self.pos]);
                return Ok(true);
            } else {
                buf.extend_from_slice(&self.buf[self.pos..])
            }
        }
        self.pos = self.filled;

        Ok(false)
    }

    #[cold]
    #[inline(never)]
    fn cold_next_token_3_or_more(&mut self, mut buf: Vec<u8>) -> io::Result<Token> {
        debug_assert_eq!(self.pos, self.filled);

        loop {
            self.filled = self.reader.read(&mut self.buf)?;
            self.pos = 0;

            // EOF or empty
            if self.filled == 0 {
                return Err(io::Error::new(ErrorKind::Other, "no more data"));
            }

            if let Some(skip) = self.buf[..self.filled]
                .iter()
                .position(|b| b.is_ascii_graphic())
            {
                self.pos += skip;
                if !buf.is_empty() && skip > 0 {
                    return Ok(Token::Bytes(buf));
                }

                if let Some(n) = self.buf[self.pos..self.filled]
                    .iter()
                    .position(|b| !b.is_ascii_graphic())
                {
                    self.pos += n;
                    return if buf.is_empty() {
                        Ok(Token::Slice(&self.buf[self.pos - n..self.pos]))
                    } else {
                        buf.extend_from_slice(&self.buf[0..self.pos]);
                        Ok(Token::Bytes(buf))
                    };
                } else {
                    buf.extend_from_slice(&self.buf[self.pos..])
                }
            }
        }
    }
}

pub enum Token<'a> {
    Slice(&'a [u8]),
    Bytes(Vec<u8>),
}

impl<'a> Token<'a> {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Token::Slice(buf) => buf,
            Token::Bytes(buf) => buf,
        }
    }
}
