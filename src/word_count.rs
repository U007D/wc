#[cfg(test)]
mod unit_tests;
use crate::{Result, WordCountable};

pub fn word_count<WC: WordCountable>(input_data: WC) -> Result<usize> {
    Ok(input_data.read()?.split_whitespace().count())
}
