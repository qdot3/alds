mod parser;

use std::io::StdinLock;

pub use parser::FromBytes;

pub struct FastIn<'a> {
    stdin: StdinLock<'a>,
}