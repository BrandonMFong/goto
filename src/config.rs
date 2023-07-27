/**
 * author: Brando
 * date: 7/27/23
 */

struct Config {
    _path: String
}

impl Config {
    pub fn open(path: &str) -> Self {
        Self {
            _path: path.to_string()
        }
    }
}

