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
use std::fs::OpenOptions;
use std::io::Write;

pub struct Config {
    _path: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, &str> {
        if path.is_empty() {
            return Err("empty path to config");
        } else {
            return Ok(Self{ _path: path.to_string().clone()});
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

    pub fn enter_keypath(&self, kp: KeyPath) -> Result<(), &str> {
        // Write new key and path pair
        let mut file_writer = OpenOptions::new().create(true).write(true).append(true).open(&self._path).unwrap();

        // write
        if let Err(error) = writeln!(file_writer, "{}", kp.entry()) {
            eprintln!("Error occurred writing line: {} ({})", kp.entry(), error);
            return Err("could not write entry");
        }

        // Make sure the writer flushed all data
        if let Err(error) = file_writer.flush() {
            eprintln!("{}", error);
            return Err("error flushing");
        }

        return Ok(());
    }
}

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

    #[test]
    fn config_constructor() {
        setup();
        let path = goto_key_paths_file_path();
        let conf = Config::new(&path);
        assert!(conf.is_ok());
        teardown();
    }

    #[test]
    fn entries_with_no_lines() {
        let entries = Entries { _lines: None };
        assert!(entries._lines.is_none());
    }

    #[test]
    fn config_constructor_with_empty_path() {
        let conf = Config::new("");
        assert!(conf.is_err());
    }

    #[test]
    fn loop_through_test_entries() {
        setup();
        let path = goto_key_paths_file_path();
        let conf = Config::new(&path);
        assert!(conf.is_ok());
        let mut i = 0;
        for _ in conf.unwrap().entries() {
            i += 1;
        }

        assert!(i > 0);
        teardown();
    }
}

