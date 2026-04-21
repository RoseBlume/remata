#[allow(unused)]
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct Darwin;

impl Darwin {
    pub fn parse<R: Read + Seek>(_reader: &mut R) -> io::Result<Self> {
        Ok(Self {})
    }
}