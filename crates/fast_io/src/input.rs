use std::{
    fmt::Debug,
    io::{self, BufRead, Error, ErrorKind},
    marker::PhantomData,
};

use super::FromBytes;

/// A wrapper of [BufReader](std::io::BufReader).
pub struct FastInput<R: BufRead> {
    reader: R,
    consumed: usize,
}

impl<R: BufRead> FastInput<R> {
    /// Cheats a new buffered handler of the given reader.
    #[inline]
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            consumed: 0,
        }
    }

    // TODO: use thiserror
    pub fn next_token<T: FromBytes>(&mut self) -> io::Result<T>
    where
        <T as FromBytes>::Err: Debug,
    {
        // self.consumed will be 0.
        self.reader.consume(std::mem::take(&mut self.consumed));

        let mut buf = Vec::new();
        // the process usually completes in two iteration
        {
            let src = self.reader.fill_buf()?;
            if src.is_empty() {
                return Err(Error::new(ErrorKind::Other, "no more data"));
            }
            if let Some(skip) = src.iter().position(|b| b.is_ascii_graphic()) {
                if let Some(n) = src[skip..].iter().position(|b| !b.is_ascii_graphic()) {
                    self.consumed = skip + n;
                    // TODO: parsing error
                    return Ok(T::from_bytes(&src[skip..skip + n]).unwrap());
                } else {
                    buf.extend_from_slice(&src[skip..]);
                }
            }
            let len = src.len();
            self.reader.consume(len);
        }
        {
            let src = self.reader.fill_buf()?;
            if src.is_empty() {
                return Err(Error::new(ErrorKind::Other, "no more data"));
            }
            if let Some(skip) = src.iter().position(|b| b.is_ascii_graphic()) {
                if !buf.is_empty() && skip != 0 {
                    // TODO: parsing error
                    self.reader.consume(skip);
                    return Ok(T::from_bytes(&buf).unwrap());
                }
                if let Some(n) = src[skip..].iter().position(|b| !b.is_ascii_graphic()) {
                    self.consumed = skip + n;
                    // TODO: parsing error
                    if !buf.is_empty() {
                        debug_assert_eq!(skip, 0);

                        buf.extend_from_slice(&src[..n]);
                        return Ok(T::from_bytes(&buf).unwrap());
                    } else {
                        return Ok(T::from_bytes(&src[skip..skip + n]).unwrap());
                    }
                } else {
                    buf.extend_from_slice(&src[skip..]);
                }
            }
            let len = src.len();
            self.reader.consume(len);
        }

        const ITERATION_LIMIT: usize = 1_000_000;
        for _ in 0..ITERATION_LIMIT {
            let src = self.reader.fill_buf()?;
            if src.is_empty() {
                return Err(Error::new(ErrorKind::Other, "no more data"));
            }
            if let Some(skip) = src.iter().position(|b| b.is_ascii_graphic()) {
                if !buf.is_empty() && skip != 0 {
                    // TODO: parsing error
                    self.reader.consume(skip);
                    return Ok(T::from_bytes(&buf).unwrap());
                }
                if let Some(n) = src[skip..].iter().position(|b| !b.is_ascii_graphic()) {
                    self.consumed = skip + n;
                    // TODO: parsing error
                    if !buf.is_empty() {
                        debug_assert_eq!(skip, 0);

                        buf.extend_from_slice(&src[..n]);
                        return Ok(T::from_bytes(&buf).unwrap());
                    } else {
                        return Ok(T::from_bytes(&src[skip..skip + n]).unwrap());
                    }
                } else {
                    buf.extend_from_slice(&src[skip..]);
                }
            }
            let len = src.len();
            self.reader.consume(len);
        }

        panic!("reached iteration limit: {}", ITERATION_LIMIT);
    }
}

pub enum Token<'a, R: BufRead> {
    Slice(&'a [u8], PhantomData<&'a R>),
    Bytes(Vec<u8>),
}

// impl<'a> Token<'a> {
//     #[inline]
//     pub fn as_slice(&self) -> &[u8] {
//         match self {
//             Token::Slice(buf) => buf,
//             Token::Bytes(buf) => buf,
//         }
//     }
// }
