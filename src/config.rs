/**
 * author: Brando
 * date: 7/27/23
 */

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::keypath::KeyPath;
use std::io::Lines;
use std::io;

pub struct Entries {
    _lines: Option<Lines<BufReader<File>>>
}

// https://doc.rust-lang.org/rust-by-example/trait/iter.html
impl Iterator for Entries {
    type Item = KeyPath;
    fn next(&mut self) -> Option<Self::Item> {
        if self._lines.is_none() {
            None
        } else {
            let lines = &mut self._lines;
            match lines.as_mut().unwrap().next() {
                Some(entry) => return Some(KeyPath::from_entry(&entry.unwrap())),
                None => return None,
            }
        }
    }
}

pub struct Config {
    _path: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, &str> {
        if path.is_empty() {
            return Err("empty path to config");
        } else {
            return Ok(Self{ _path: path.to_string()});
        }
    }

    pub fn entries(&self) -> Entries {
        match get_file_reader_for_file(&self._path) {
            Err(_) => {
                return Entries{ _lines: None };
            } Ok(f) => {
                return Entries{ _lines: Some(f.lines()) };
            }
        }
    }
}

fn get_file_reader_for_file(path: &str) -> Result<BufReader<File>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup, teardown};
    use crate::goto_key_paths_file_path;

    #[test]
    fn valid_file_reader() {
        setup();
        let reader = get_file_reader_for_file(&goto_key_paths_file_path());
        assert!(reader.is_ok());
        assert!(reader.unwrap().lines().count() == 1, "we are expecting only one line in this test case");
        teardown();
    }
}

