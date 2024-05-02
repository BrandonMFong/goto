/**
 * author: brando
 * date: 4/26/24
 *
 * history file is used as a stack
 */

use std::fs::File;
use std::io::{self, Write};
use std::io::BufRead;
use crate::goto_history_file_path;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::BufReader;

/**
 * adds `path` to the end of the history file
 *
 * creates a history file if it doesn't exist
 */
pub fn push(path: &str) -> io::Result<()> {
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(goto_history_file_path())?;

    // Write the new line to the file
    writeln!(file, "{}", path)?;

    Ok(())
}

/**
 * returns the second to last line
 *
 * both the last line and second to last line are removed from history
 */
pub fn pop() -> io::Result<String> {
    // Open the file for reading and writing
    let mut file = File::open(goto_history_file_path())?;
    let reader = BufReader::new(&file);

    // Read the lines into a vector
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    lines.pop();

    // Seek to the position before the last line
    file.seek(SeekFrom::Start(0))?;

    let mut file = File::create(goto_history_file_path())?;
    for line in &lines {
        writeln!(file, "{}", line)?;
    }
    
    let last_line = lines.pop();

    if let Some(res) = last_line {
        return Ok(res);
    } else {
        return Ok(String::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup, teardown};
    use crate::{goto_utils_path};

    #[test]
    fn push_path_then_check_existence() {
        setup();
        let path = goto_utils_path();
        let _ = push(&path);
  
        let file = File::open(goto_history_file_path()).unwrap();
        let reader = BufReader::new(&file);

        // Read the lines into a vector
        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        assert!(lines.len() == 1);
        assert!(lines[0] == path);

        teardown();
    }

    #[test]
    fn pop_path() {
        setup();
        
        let path = goto_utils_path();
        let _ = push(&path);
  
        let newpath = pop().unwrap();
        assert!(newpath.len() == 0, "new path should be empty");

        let _ = push(&path);
        let _ = push(&path);
        let newpath = pop().unwrap();
        assert!(newpath.len() > 0, "new path is empty");
        assert!(newpath == path, "{} != {}", newpath, path);

        teardown();
    }
}

