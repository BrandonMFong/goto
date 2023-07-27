/**
 * author: Brando
 * date: 7/27/23
 */

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::keypath::KeyPath;
use std::io::Lines;

pub struct Config {
    _path: String,
    _reader: Option<BufReader<File>>
}

impl Config {
    pub fn open_for_read(path: &str) -> Result<Self, &str> {
        let mut result = Self {_path: path.to_string(), _reader: None };
        
        match File::options().read(true).open(&result._path) {
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err("Could not open for read");
            } Ok(f) => {
                result._reader = Some(BufReader::new(f));
            }
        }

        if result._reader.is_none() {
            return Err("Reader could not be used");
        }

        Ok(result)
    }

    pub fn entries(&self) -> Result<KeyPath, &str> {
        let result = KeyPath::new("", "");

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
}

