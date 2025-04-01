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
    #[inline]
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
    #[inline]
    pub fn parse_unwrap<T>(&mut self) -> T
    where
        T: FromBytes,
        T::Err: Debug,
    {
        let token = self.next_token().unwrap();
        T::from_bytes(token.as_slice()).unwrap()
    }

    pub fn next_token(&mut self) -> io::Result<Token> {
        let mut bytes = Vec::new();
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
                bytes.extend_from_slice(&self.buf[self.pos..self.filled])
            }
        }
        {
            self.filled = self.reader.read(&mut self.buf)?;
            self.pos = 0;
            if self.filled == 0 {
                return Err(io::Error::new(ErrorKind::Other, "empty"));
            }
            if let Some(skip) = self.buf[..self.filled]
                .iter()
                .position(|b| b.is_ascii_graphic())
            {
                self.pos += skip;
                if !bytes.is_empty() && skip > 0 {
                    return Ok(Token::Bytes(bytes));
                }
                if let Some(n) = self.buf[self.pos..self.filled]
                    .iter()
                    .position(|b| !b.is_ascii_graphic())
                {
                    self.pos += n;
                    return if bytes.is_empty() {
                        Ok(Token::Slice(&self.buf[self.pos - n..self.pos]))
                    } else {
                        bytes.extend_from_slice(&self.buf[0..self.pos]);
                        Ok(Token::Bytes(bytes))
                    };
                } else {
                    bytes.extend_from_slice(&self.buf[self.pos..])
                }
            }
        }
        loop {
            self.filled = self.reader.read(&mut self.buf)?;
            self.pos = 0;
            if self.filled == 0 {
                return Err(io::Error::new(ErrorKind::Other, "empty"));
            }
            if let Some(skip) = self.buf[..self.filled]
                .iter()
                .position(|b| b.is_ascii_graphic())
            {
                self.pos += skip;
                if !bytes.is_empty() && skip > 0 {
                    return Ok(Token::Bytes(bytes));
                }
                if let Some(n) = self.buf[self.pos..self.filled]
                    .iter()
                    .position(|b| !b.is_ascii_graphic())
                {
                    self.pos += n;
                    return if bytes.is_empty() {
                        Ok(Token::Slice(&self.buf[self.pos - n..self.pos]))
                    } else {
                        bytes.extend_from_slice(&self.buf[0..self.pos]);
                        Ok(Token::Bytes(bytes))
                    };
                } else {
                    bytes.extend_from_slice(&self.buf[self.pos..])
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
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Token::Slice(buf) => buf,
            Token::Bytes(buf) => buf,
        }
    }
}
