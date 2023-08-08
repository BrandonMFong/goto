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
use std::io::SeekFrom;
use std::io::Seek;

pub struct Config {
    _path: String,
}

impl Config {
    /**
     * Creates a new config object
     */
    pub fn new(path: &str) -> Result<Self, &str> {
        if path.is_empty() {
            return Err("empty path to config");
        } else {
            return Ok(Self{ _path: path.to_string().clone()});
        }
    }

    /**
     * returns an iterator that provides each key path entry in config
     */
    pub fn entries(&self) -> Entries {
        match Self::create_reader(&self._path) {
            Err(_) => {
                return Entries{ _lines: None };
            } Ok(f) => {
                return Entries{ _lines: Some(f.lines()) };
            }
        }
    }

    /**
     * Writes keypath entry into config
     *
     * does not check if kp already exists
     */
    pub fn enter_keypath(&self, kp: KeyPath) -> Result<(), &str> {
        // Write new key and path pair
        let file_writer = Config::create_writer(&self._path);
        if file_writer.is_err() {
            return Err(file_writer.err().unwrap());
        }

        // write
        if let Err(error) = writeln!(file_writer.as_ref().unwrap(), "{}", kp.entry()) {
            eprintln!("Error occurred writing line: {} ({})", kp.entry(), error);
            return Err("could not write entry");
        }

        // Make sure the writer flushed all data
        if let Err(error) = file_writer.as_ref().unwrap().flush() {
            eprintln!("{}", error);
            return Err("error flushing");
        }

        return Ok(());
    }

    pub fn remove_keypath(&self, key: &str) -> Result<(), &str> {
        // Open the file in read-write mode
        let file = Config::create_writer(&self._path);
        if file.is_err() {
            return Err(file.err().unwrap());
        }

        // Create a buffer to store the modified contents
        let mut buffer = Vec::new();

        // Seek to the beginning of the file
        if let Err(error) = file.as_ref().unwrap().seek(SeekFrom::Start(0)) {
            eprintln!("Error occured: {}", error);
            return Err("couldn't seek to start of file");
        }

        // Iterate over the lines and exclude the line to be removed
        for line in io::BufReader::new(file.as_ref().unwrap()).lines() {
            // Write non-matching lines to the buffer
            if let Ok(ref ip) = line {
                let key_path_pair = KeyPath::from_entry(&ip);
                if key_path_pair.is_valid() && key_path_pair.key() != key {
                    buffer.extend(line.unwrap().bytes());
                    buffer.push(b'\n');
                }
            }
        }

        // Truncate the file to the current position (i.e., remove the remaining contents)
        let seek_current = file.as_ref().unwrap().seek(SeekFrom::Start(0)).unwrap();
        if let Err(error) = file.as_ref().unwrap().set_len(seek_current) {
            eprintln!("Error occured: {}", error);
            return Err("couldn't erase file");
        }

        // Write the modified contents back to the file
        if let Err(error) = file.as_ref().unwrap().write_all(&buffer) {
            eprintln!("Error occured: {}", error);
            return Err("couldn't dump to file");
        }

        return Ok(());
    }

    /**
     * creates a writer for path
     */
    fn create_writer(path: &str) -> Result<File, &str> {
        match OpenOptions::new().create(true).write(true).append(true).open(&path) {
            Err(_) => {
                Err("couldn't open for writing")
            } Ok(res) => {
                Ok(res)
            }
        }
    }
    fn create_reader(path: &str) -> Result<BufReader<File>, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader)
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup, teardown};
    use crate::{goto_key_paths_file_path, goto_utils_path};

    #[test]
    fn valid_file_reader() {
        setup();
        let reader = Config::create_reader(&goto_key_paths_file_path());
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

    #[test]
    fn create_writer() {
        setup();
        let path = goto_key_paths_file_path();
        let writer = Config::create_writer(&path);
        assert!(writer.is_ok());

        let mut path = goto_utils_path();
        println!("{}", path);
        path += "/test";
        let writer = Config::create_writer(&path);
        assert!(writer.is_ok());
        teardown();
    }
}

