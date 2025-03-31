use std::io::{self, BufRead, ErrorKind};

pub struct FastInput<const N: usize, R: BufRead> {
    reader: R,
    buf: [u8; N],
    pos: usize,
    filled: usize,
}

impl<const N: usize, R: BufRead> FastInput<N, R> {
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

    pub fn next_token(&mut self) -> io::Result<Token> {
        let mut bytes = Vec::new();
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
                bytes.extend_from_slice(&self.buf[self.pos..self.filled])
            }
        }

        // Otherwise, the loop usually completes within two iterations."
        {
            self.filled = self.reader.read(&mut self.buf)?;
            self.pos = 0;

            // EOF or empty
            if self.filled == 0 {
                return Err(io::Error::new(ErrorKind::UnexpectedEof, "reached EOF"));
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

        // cold if `buf` hsa sufficient large capacity.
        loop {
            self.filled = self.reader.read(&mut self.buf)?;
            self.pos = 0;

            // EOF or empty
            if self.filled == 0 {
                return Err(io::Error::new(ErrorKind::UnexpectedEof, "reached EOF"));
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
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Token::Slice(bytes) => &bytes,
            Token::Bytes(bytes) => &bytes,
        }
    }
}
