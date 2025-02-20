/*
 * author: Brando
 * date: 6/14/23
 */

mod keypath;
mod config;
mod history;

use std::env;
use std::process;
use home;
use std::path::PathBuf;
use std::path::Path;
use std::fs::canonicalize;
use crate::keypath::KeyPath;
use crate::config::Config;
use std::fs;

static ARG_GETPATH_PREV: &'static str = "--prev";
static ARG_GETKEYS: &'static str = "--show-keys";
static ARG_ADD: &'static str = "--add";
static ARG_HELP: &'static str = "--help";
static ARG_SHOWALLKEYPAIRS: &'static str = "--show-all";
static ARG_GETVERSION: &'static str = "--version";

static ARG_REMOVE: &'static str = "--remove";

/**
 * outputs all args in a space-delimited list
 */
static ARG_ACCEPTEDARGS: &'static str = "--accepted-args";

/**
 * this argument expects a parameter
 */
static ARG_GETSUGKEYS: &'static str = "--show-suggested-keys";

/**
 * outputs the content for zsh-completions
 *
 * https://github.com/zsh-users/zsh-completions/blob/master/zsh-completions-howto.org#table-of-contents
 */
static ARG_COMPLETION_ZSH: &'static str = "--completion-zsh";

static GOTO_UTILS_DIRNAME_TEST: &'static str = ".goto_test";
static GOTO_UTILS_DIRNAME_RELEASE: &'static str = ".goto";
static GOTO_UTILS_DIRNAME_KEYPATHS: &'static str = "keypaths";
static GOTO_UTILS_DIRNAME_HISTORY: &'static str = "history";
static GOTO_UTILS_DIRNAME: &'static str = if cfg!(test) { GOTO_UTILS_DIRNAME_TEST } else { GOTO_UTILS_DIRNAME_RELEASE };

fn version() -> String {
    return env!("CARGO_PKG_VERSION").to_owned();
}

/**
 * this help is in the context of the script wrapper function: goto
 */
fn help() {
    let tool_name = "goto";
    println!("usage: {tool_name} <arg>");
    println!("arguments:");
    println!("");
    println!("{tool_name} {ARG_GETPATH_PREV} = cd back into previous directory");
    println!("{tool_name} {ARG_GETKEYS} <path> = returns all keys for the path");
    println!("{tool_name} {ARG_ADD} <key> <path> = adds key/path pair");
    println!("{tool_name} {ARG_REMOVE} <key> = removes key/path pair via key");
    println!("{tool_name} {ARG_SHOWALLKEYPAIRS} = shows all key pairs");
    println!("{tool_name} {ARG_HELP} = gets help");

    println!();
    println!("version: {}, 2024", version());
}

/**
 * writes content for the completion file: _goto
 *
 * https://github.com/zsh-users/zsh-completions/blob/master/zsh-completions-howto.org
 */
fn completion_zsh() -> Result<(), i32> {
    println!("#compdef goto");
    println!("local -a subcmds");
    println!("subcmds=( \\");

    match Config::new(&goto_key_paths_file_path()) {
        Err(e) => {
            eprintln!("{}", e);
        } Ok(conf) => {
            for key_path_pair in conf.entries() {
                if !key_path_pair.is_valid() {
                    break;
                } else {
                    println!("'{}:{}' \\", key_path_pair.key(), key_path_pair.path());
                }
            }
        }
    }
 
    println!("'{ARG_HELP}:gets help' \\");
    println!("'{ARG_GETPATH_PREV}:cd back into previous directory' \\");
    println!("'{ARG_GETKEYS}:returns all keys for the path' \\");
    println!("'{ARG_ADD}:adds key/path pair' \\");
    println!("'{ARG_REMOVE}:removes key/path pair via key' \\");
    println!("'{ARG_SHOWALLKEYPAIRS}:shows all key pairs' \\");
    println!(")");
    println!("_describe 'goto' subcmds");

    Ok(())
}

fn main() {
    let mut error = 0;
    match run() {
        Err(errno) => error = errno,
        Ok(()) => {}
    } 

    process::exit(error);
}

fn run() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].eq(ARG_HELP) {
        help();
    } else if args[1].eq(ARG_ADD) {
        add_key_path(&args)?;
    } else if args[1].eq(ARG_GETKEYS) {
        print_keys_for_path(&args)?;
    } else if args[1].eq(ARG_GETSUGKEYS) {
        print_suggested_keys(&args)?;
    } else if args[1].eq(ARG_REMOVE) {
        remove_key_path(&args)?;
    } else if args[1].eq(ARG_SHOWALLKEYPAIRS) {
        print_all_key_pairs()?;
    } else if args[1].eq(ARG_GETPATH_PREV) {
        print_previous_path()?;
    } else if args[1].eq(ARG_GETVERSION) {
        println!("{}", version());
    } else if args[1].eq(ARG_ACCEPTEDARGS) {
        print_all_accepted_args()?;
    } else if args[1].eq(ARG_COMPLETION_ZSH) {
        completion_zsh()?;
    } else {
        print_path_for_key(&args)?;
    }

    Ok(())
}

fn print_all_accepted_args() -> Result<(), i32> {
    println!(
    "{ARG_GETPATH_PREV} {ARG_GETKEYS} \
    {ARG_GETSUGKEYS} {ARG_ADD} \
    {ARG_HELP} {ARG_SHOWALLKEYPAIRS} \
    {ARG_GETVERSION} {ARG_ACCEPTEDARGS} \
    {ARG_COMPLETION_ZSH} \
    {ARG_REMOVE}");

    Ok(())
}

fn print_all_key_pairs() -> Result<(), i32> {
    match Config::new(&goto_key_paths_file_path()) {
        Err(e) => {
            eprintln!("{}", e);
            return Err(1)
        } Ok(conf) => {
            for key_path_pair in conf.entries() {
                if !key_path_pair.is_valid() {
                    eprintln!("error in key path pair");
                    return Err(1)
                } else {
                    println!("{} => {}", key_path_pair.key(), key_path_pair.path());
                }
            }
        }
    }

    Ok(())
}

fn print_previous_path() -> Result<(), i32> {
    match history::pop() {
        Err(e) => {
            eprintln!("{}", e);
            return Err(1)
        }
        Ok(path) => {
            if path.len() == 0 {
                return Err(1)
            } else {
                println!("{}", path);
            }
        }
    }

    Ok(())
}

fn print_path_for_key(args: &Vec<String>) -> Result<(), i32> {
    if args.len() < 2 {
        return Err(1);
    }

    let key: &String = &args[1];
    match Config::new(&goto_key_paths_file_path()) {
        Err(e) => {
            eprintln!("Could not read file {}: {}", goto_key_paths_file_path(), e);
            return Err(1)
        } Ok(conf) => {
            for key_path_pair in conf.entries() {
                if !key_path_pair.is_valid() {
                    eprintln!("error in key path pair");
                    return Err(1)
                } else if key_path_pair.key() == key {
                    println!("{}", key_path_pair.path());
                    if let Err(e) = history::push(key_path_pair.path()) {
                        eprintln!("{}", e);
                    }
                    return Ok(())
                }
            }
        }
    }

    eprintln!("Could not find path for key: {key}");

    Err(1)
}

fn print_keys_for_path(args: &Vec<String>) -> Result<(), i32> {
    if args.len() < 3 {
        return Err(1);
    }

    let path: &String = &args[2];
    if !Path::new(path).exists() {
        eprintln!("path \"{}\" is not an existing path", path);
        return Err(1)
    }

    // Expand the input path
    let expanded_path = expand_path(&path);
    match Config::new(&goto_key_paths_file_path()) {
        Err(e) => {
            eprintln!("Could not read file {}: {}", goto_key_paths_file_path(), e);
            return Err(1)
        } Ok(conf) => {
            for key_path_pair in conf.entries() {
                if !key_path_pair.is_valid() {
                    eprintln!("error in key path pair");
                    return Err(1)
                } else if key_path_pair.path() == expanded_path {
                    println!("{} => {}", key_path_pair.key(), key_path_pair.path());
                }
            }
        }
    }

    Ok(())
}

/**
 * prints similar keys to input
 */
fn print_suggested_keys(args: &Vec<String>) -> Result<(), i32> {
    if args.len() < 3 {
        return Err(1);
    }

    let input: &String = &args[2];
    // Expand the input path
    match Config::new(&goto_key_paths_file_path()) {
        Err(e) => {
            eprintln!("Could not read file {}: {}", goto_key_paths_file_path(), e);
            return Err(1)
        } Ok(conf) => {
            for key_path_pair in conf.entries() {
                if !key_path_pair.is_valid() {
                    eprintln!("error in key path pair");
                    return Err(1)
                } else if key_path_pair.key().starts_with(input) {
                    println!("{}", key_path_pair.key());
                }
            }
        }
    }

    Ok(())
}

fn remove_key_path(args: &Vec<String>) -> Result<(), i32> {
    if args.len() < 3 {
        return Err(1);
    }

    let key: &String = &args[2];
    let key_paths_file_path = goto_key_paths_file_path();
    let config = Config::new(&key_paths_file_path);
    if let Err(e) = config {
        eprintln!("Could not read file {}: {}", goto_key_paths_file_path(), e);
        return Err(1)
    }

    match config.unwrap().remove_keypath(key) {
        Err(e) => {
            eprintln!("could not remove key: {}", key);
            eprintln!("{}", e);
            return Err(1)
        } Ok (_) => {

        }
    }

    Ok(())
}

fn add_key_path(args: &Vec<String>) -> Result<(), i32> {
    if args.len() < 4 {
        return Err(1);
    }

    let key: &String = &args[2];
    let path: &String = &args[3];
    let key_paths_file_path = goto_key_paths_file_path();
    let config = Config::new(&key_paths_file_path);
    if let Err(e) = config {
        eprintln!("Could not read file {}: {}", goto_key_paths_file_path(), e);
        return Err(1);
    } else {
        let conf = config.as_ref().unwrap();
        for key_path_pair in conf.entries() {
            if key_path_pair.is_valid() && key_path_pair.key() == key {
                eprintln!("key ({key}) already exists");
                return Err(1);
            }
        }
    }
    
    // create key path
    let expanded_path = expand_path(&path);
    let kp = KeyPath::new(&key, &expanded_path);
    if !kp.is_valid() {
        eprintln!("invalid key path pair");
        return Err(1);
    }
  
    // write new keypath to config
    match config.unwrap().enter_keypath(kp) {
        Err(e) => {
            eprintln!("experienced an error entry new key path: {}", e);
            return Err(1)
        } Ok(_) => {}
    }

    Ok(())
}

fn goto_utils_path() -> String {
    let mut res = PathBuf::from(home::home_dir().unwrap());
    res.push(GOTO_UTILS_DIRNAME);
    return res.to_string_lossy().to_string();
}

fn goto_key_paths_file_path() -> String {
    let mut res = goto_utils_path();
    res += &("/".to_owned() + GOTO_UTILS_DIRNAME_KEYPATHS);
    return res;
}

pub fn goto_history_file_path() -> String {
    let mut res = goto_utils_path();
    res += &("/".to_owned() + GOTO_UTILS_DIRNAME_HISTORY);
    return res;
}

/**
 * expands relative path
 */
fn expand_path(path: &str) -> String {
    return canonicalize(path).unwrap().into_os_string().into_string().unwrap();
}

fn path_exists(path: &str) -> bool {
    if let Ok(_) = fs::metadata(path) {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    /**
     * Creates .gotoutils directory at user home directory
     *
     * Creates a keypaths with one entry
     */
    pub fn setup() {
        // create the test env
        let mut path = goto_utils_path();

        if path_exists(&path) {
            let result = fs::remove_dir_all(&path);
            assert!(result.is_ok());
        }

        let result = fs::create_dir(&path);
        assert!(result.is_ok(), "couldn't create {}", path);
       
        // make sure it is created
        let meta = fs::metadata(&path);
        assert!(meta.is_ok());
        assert!(meta.unwrap().is_dir());

        // we will be writing test data
        path = goto_key_paths_file_path();
        let file = File::create(path);
        assert!(file.is_ok());

        // write test data
        let home_dir = PathBuf::from(home::home_dir().unwrap());
        let line = "home|".to_owned() + &home_dir.to_string_lossy().to_string();
        let result = file.expect("could not write to file").write_all(line.as_bytes());
        assert!(result.is_ok());
    }

    /**
     * removes gotoutils directory
     */
    pub fn teardown() {
        let path = goto_utils_path();
        let result = fs::remove_dir_all(path);
        assert!(result.is_ok());
    }

    #[test]
    fn version_string_is_not_empty() {
        let result = version();
        assert!(!result.is_empty());
    }

    #[test]
    fn gotoutil_dirname_is_for_tests() {
        assert_eq!(GOTO_UTILS_DIRNAME, GOTO_UTILS_DIRNAME_TEST);
    }

    #[test]
    fn goto_util_dirpath_not_empty() {
        assert!(!goto_utils_path().is_empty());
    }

    #[test]
    fn goto_util_keypath_file_path_not_empty() {
        assert!(!goto_key_paths_file_path().is_empty());
    }

    #[test]
    fn expanding_paths() {
        let path = expand_path(".");
        assert!(!path.is_empty());
        assert!(path_exists(&path));
    }
}

