/**
 * author: brando
 * date: 7/27/23
 */

pub static GOTO_KEY_PATH_DELIMITER: &'static str = "|";

struct KeyPath {
    key: String,
    path: String
}

/**
 * Takes in a line from keypaths file and splits it out by the GOTO_KEY_PATH_DELIMITER
 */
fn split_key_path_line_entry(entry: &str) -> Vec<&str> {
    if entry.is_empty() {
        return Vec::new() 
    } else {
        return entry.split(GOTO_KEY_PATH_DELIMITER).collect();
    }
}

impl KeyPath {
    pub fn new(entry: &str) -> Self {
        let mut result = KeyPath {key: String::new(), path: String::new()};
        if !entry.is_empty() {
            let vec: Vec<&str> = split_key_path_line_entry(entry);
            
            if vec.len() == 2 {
                result.key = vec[0].to_string();
                result.path = vec[1].to_string();
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn key_path_line_entry_split() {
        let mut vec = split_key_path_line_entry("hello|world");
        assert!(vec.len() == 2);
        assert!(vec[0] == "hello", "{} != 'hello'", vec[0]);
        assert!(vec[1] == "world", "{} != 'world'", vec[1]);

        vec = split_key_path_line_entry("hello");
        assert!(vec.len() == 1);
        assert!(vec[0] == "hello", "{} != 'hello'", vec[0]);

        // We should not have any splits
        vec = split_key_path_line_entry("");
        assert!(vec.len() == 0);
    }
}

