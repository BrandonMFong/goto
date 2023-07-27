/**
 * author: brando
 * date: 7/27/23
 */

pub static GOTO_KEY_PATH_DELIMITER: &'static str = "|";

pub struct KeyPath {
    _key: String,
    _path: String,
    _valid: bool
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
    pub fn from_entry(entry: &str) -> Self {
        let mut result = KeyPath {
            _key: String::new(),
            _path: String::new(),
            _valid: false
        };

        if !entry.is_empty() {
            let vec: Vec<&str> = split_key_path_line_entry(entry);
            if vec.len() == 2 {
                result._key = vec[0].to_string();
                result._path = vec[1].to_string();
                result._valid = true;
            }
        }

        return result;
    }

    pub fn new(key: &str, path: &str) -> Self {
        let mut result = KeyPath {
            _key: String::new(),
            _path: String::new(),
            _valid: false
        };

        if !key.is_empty() && !path.is_empty() {
            result._key = key.to_string();
            result._path = path.to_string();
            result._valid = true;
        }

        return result;
    }

    /**
     * true if key and path are both set
     */
    pub fn is_valid(&self) -> bool {
        self._valid
    }

    pub fn key(&self) -> &str {
        &self._key
    }

    pub fn path(&self) -> &str {
        &self._path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn key_path_constructor_from_entry() {
        let mut kp = KeyPath::from_entry("hello|world");
        assert!(kp.is_valid());
        assert!(kp.key() == "hello", "{} != 'hello'", kp.key());
        assert!(kp.path() == "world", "{} != 'world'", kp.path());

        kp = KeyPath::from_entry("hello world");
        assert!(!kp.is_valid());
        assert!(kp.key().is_empty());
        assert!(kp.path().is_empty());

        kp = KeyPath::from_entry("hello|world|amazing");
        assert!(!kp.is_valid());
        assert!(kp.key().is_empty());
        assert!(kp.path().is_empty());
    }

    #[test]
    fn key_path_constructor_manual() {
        let mut kp = KeyPath::new("key", "path");
        assert!(kp.is_valid());
        assert!(kp.key() == "key", "{} != 'key'", kp.key());
        assert!(kp.path() == "path", "{} != 'path'", kp.path());

        kp = KeyPath::new("key", "");
        assert!(!kp.is_valid());
        assert!(kp.key().is_empty());
        assert!(kp.path().is_empty());

        kp = KeyPath::new("", "path");
        assert!(!kp.is_valid());
        assert!(kp.key().is_empty());
        assert!(kp.path().is_empty());
    }
}

