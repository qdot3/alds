use std::io::Read;

pub struct FastIn {
    buf: Vec<u8>,
    cursor: usize,
}

impl FastIn {
    pub fn new(mut reader: impl Read) -> Self {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();

        Self { buf, cursor: 0 }
    }

    pub fn next_token(&mut self) -> &[u8] {
        todo!()
    }
}
