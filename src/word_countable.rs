use crate::Result;
use std::{fs::File, io::Read};

pub trait WordCountable: Sized {
    fn read(self) -> Result<String>;
}

impl WordCountable for String {
    fn read(self) -> Result<String> {
        Ok(self)
    }
}

impl WordCountable for File {
    fn read(mut self) -> Result<String> {
        let mut input = String::new();
        self.read_to_string(&mut input)?;
        Ok(input)
    }
}
