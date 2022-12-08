mod days;

pub use days::*;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

pub fn get_input<P: AsRef<Path>>(p: P) -> Result<Vec<String>, Error> {
    let mut file = File::open(p)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string
        .split('\n')
        .into_iter()
        .map(|s| s.trim_end().to_string())
        .collect())
}

#[cfg(test)]
pub fn parse_input_static(raw: &str) -> Vec<String> {
    raw.split('\n').map(|s| s.trim_end().to_string()).collect()
}
