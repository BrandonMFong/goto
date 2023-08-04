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
    _lines: Lines<BufReader<File>>
}

// https://doc.rust-lang.org/rust-by-example/trait/iter.html
impl Iterator for Entries {
    type Item = KeyPath;
    fn next(&mut self) -> Option<Self::Item> {
        let line = self._lines.next();
        match line {
            Some(entry) => return Some(KeyPath::from_entry(&entry.unwrap())),
            None => return None,
        }
    }
}

pub struct Config {
    _path: String,
    _entries: Entries
}

impl Config {
    pub fn open_for_read(path: &str) -> Result<Self, &str> {
        match File::options().read(true).open(path) {
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err("Could not open for read");
            } Ok(f) => {
                let result = Self {
                    _path: path.to_string(), 
                    _entries: Entries{ _lines: BufReader::new(f).lines() }
                };
 
                Ok(result)
            }
        }
    }

    pub fn entries(&self) -> Result<Entries, &str> {
        Ok(self._entries)
    }
}


#[cfg(test)]
mod tests {
}

