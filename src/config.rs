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
    _reader: Option<BufReader<File>>
}

impl Iterator for Entries {
    type Item = KeyPath;
    fn next(&mut self) -> Option<Self::Item> {
        if !self._reader.is_none() {
            
        }
        Some(KeyPath::new("", ""))
    }
}

pub struct Config {
    _path: String,
    _entries: Entries
}

impl Config {
    pub fn open_for_read(path: &str) -> Result<Self, &str> {
        let mut result = Self {
            _path: path.to_string(), 
            _entries: Entries{ _reader: None }
        };
        
        match File::options().read(true).open(&result._path) {
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err("Could not open for read");
            } Ok(f) => {
                result._entries._reader = Some(BufReader::new(f));
            }
        }

        if result._entries._reader.is_none() {
            return Err("Reader could not be used");
        }

        Ok(result)
    }

    pub fn entries(&self) -> Result<&Entries, &str> {
        Ok(&self._entries)
    }
}


#[cfg(test)]
mod tests {
}

