/**
 * author: Brando
 * date: 7/27/23
 */

use std::fs::File;

pub struct Config {
    _path: String,
    _file: File
}

impl Config {
    pub fn open_for_read(path: &str) -> Result<Self, &str> {
        let result = Self { _path: path.to_string(), _file: None};
        

        Ok(result);
    }
}

#[cfg(test)]
mod tests {
}

