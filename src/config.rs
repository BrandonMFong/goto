/**
 * author: Brando
 * date: 7/27/23
 */

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::keypath::KeyPath;
use std::io::Lines;

pub struct Entries {
    _lines: Option<Lines<BufReader<File>>>
}

// https://doc.rust-lang.org/rust-by-example/trait/iter.html
impl Iterator for Entries {
    type Item = KeyPath;
    fn next(&mut self) -> Option<Self::Item> {
        let lines = &self._lines;
        match lines.unwrap().next() {
            Some(entry) => return Some(KeyPath::from_entry(&entry.unwrap())),
            None => return None,
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
        match File::options().read(true).open(&self._path) {
            Err(e) => {
                return Entries{ _lines: None };
            } Ok(f) => {
                return Entries{ _lines: Some(BufReader::new(f).lines()) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
}

