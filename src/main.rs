/*
 * author: Brando
 * date: 6/14/23
 */

use std::env;
use std::process;
use home;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{prelude::*};
use std::fs::canonicalize;
use std::io::SeekFrom;

static ARG_GETPATH: &'static str = "getpath";
static ARG_GETKEYS: &'static str = "getkeys";
static ARG_GETSUGKEYS: &'static str = "getsugkeys";
static ARG_ADD: &'static str = "add";
static ARG_REMOVE: &'static str = "rm";
static ARG_HELP: &'static str = "help";
static ARG_SHOWALLKEYPAIRS: &'static str = "getallpairs";

static GOTO_KEY_PATH_DELIMITER: &'static str = "|";

fn help() {
    let args: Vec<String> = env::args().collect();
    let tool_name = &args[0];
    println!("usage: {tool_name} <arg>");
    println!("\targuments:");
    println!("\t\t{ARG_GETPATH}: {tool_name} {ARG_GETPATH} <key>");
    println!("\t\t{ARG_GETKEYS}: {tool_name} {ARG_GETKEYS} <path>");
    println!("\t\t{ARG_GETSUGKEYS}: {tool_name} {ARG_GETSUGKEYS}");
    println!("\t\t{ARG_ADD}: {tool_name} {ARG_ADD} <key> <path>");
    println!("\t\t{ARG_REMOVE}: {tool_name} {ARG_REMOVE} <key>");
    println!("\t\t{ARG_HELP}: {tool_name} {ARG_HELP}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }

    let mut error = 0;
    if args[1].eq(ARG_HELP) {
        help();
    } else {
        if args.len() > 3 {
            if args[1].eq(ARG_ADD) {
                error = add_key_path(&args[2], &args[3]);
            }
        } else if args.len() > 2 {
            if args[1].eq(ARG_GETPATH) {
                error = print_path_for_key(&args[2]);
            } else if args[1].eq(ARG_GETKEYS) {
                error = print_keys_for_path(&args[2]);
            } else if args[1].eq(ARG_GETSUGKEYS) {
                error = print_suggested_keys(&args[2]);
            } else if args[1].eq(ARG_REMOVE) {
                error = remove_key_path(&args[2]);
            } else {
                eprintln!("unknown argument: {}", &args[1]);
            }
        } else if args.len() > 1 {
            if args[1].eq(ARG_SHOWALLKEYPAIRS) {
                error = print_all_key_pairs();
            }
        }
    }

    process::exit(error);
}

fn goto_utils_path() -> PathBuf {
    let mut res = PathBuf::from(home::home_dir().unwrap());
    res.push(".gotoutils");
    return res;
}

fn goto_key_paths_file_path() -> PathBuf {
    let mut res = goto_utils_path();
    res.push("keypaths");
    return res
}

fn print_all_key_pairs() -> i32 {
    match get_file_reader_for_file(&goto_key_paths_file_path().to_string_lossy().to_string()) {
        Err(e) => {
            eprintln!("Could not read file {}: {}", goto_key_paths_file_path().display(), e);
            return -1;
        } Ok(reader) => {
            for line in reader.lines() {
                if let Ok(ip) = line {
                    // key|path
                    let key_path_pair: Vec<&str> = ip.split(GOTO_KEY_PATH_DELIMITER).collect();
                    if key_path_pair.len() != 2 {
                        eprintln!("error in key path pair");
                        return -1;
                    } else {
                        println!("{} => {}", key_path_pair[0], key_path_pair[1]);
                    }
                }
            }
        }
    }

    return 0;

}

fn print_path_for_key(key: &String) -> i32 {
    // See if key already exists
    match get_file_reader_for_file(&goto_key_paths_file_path().to_string_lossy().to_string()) {
        Err(e) => {
            eprintln!("Could not read file {}: {}", goto_key_paths_file_path().display(), e);
            return -1;
        } Ok(reader) => {
            for line in reader.lines() {
                if let Ok(ip) = line {
                    // key|path
                    let key_path_pair: Vec<&str> = ip.split(GOTO_KEY_PATH_DELIMITER).collect();
                    if key_path_pair.len() != 2 {
                        eprintln!("error in key path pair");
                        return -1;
                    } else if key_path_pair[0] == key {
                        println!("{}", key_path_pair[1]);
                        return 0;
                    }
                }
            }
        }
    }

    eprintln!("Could not find path for key: {key}");

    return -1;
}

fn print_keys_for_path(path: &String) -> i32 {
    if !Path::new(path).exists() {
        eprintln!("path \"{}\" is not an existing path", path);
        return -1;
    }

    // Expand the input path
    let expanded_path = canonicalize(path).unwrap().into_os_string().into_string().unwrap();

    match get_file_reader_for_file(&goto_key_paths_file_path().to_string_lossy().to_string()) {
        Err(e) => {
            eprintln!("Could not read file {}: {}", goto_key_paths_file_path().display(), e);
            return -1;
        } Ok(reader) => {
            for line in reader.lines() {
                if let Ok(ip) = line {
                    // key|path
                    let key_path_pair: Vec<&str> = ip.split(GOTO_KEY_PATH_DELIMITER).collect();
                    if key_path_pair.len() != 2 {
                        eprintln!("error in key path pair");
                        return -1;
                    } else if key_path_pair[1] == expanded_path {
                        println!("{} => {}", key_path_pair[0], key_path_pair[1]);
                    }
                }
            }
        }
    }

    return 0;
}

/**
 * prints similar keys to input
 */
fn print_suggested_keys(input: &String) -> i32 {
    // Expand the input path
    match get_file_reader_for_file(&goto_key_paths_file_path().to_string_lossy().to_string()) {
        Err(e) => {
            eprintln!("Could not read file {}: {}", goto_key_paths_file_path().display(), e);
            return -1;
        } Ok(reader) => {
            for line in reader.lines() {
                if let Ok(ip) = line {
                    // key|path
                    let key_path_pair: Vec<&str> = ip.split(GOTO_KEY_PATH_DELIMITER).collect();
                    if key_path_pair.len() != 2 {
                        eprintln!("error in key path pair");
                        return -1;
                    } else if key_path_pair[0].starts_with(input) {
                        println!("{}", key_path_pair[0]);
                    }
                }
            }
        }
    }

    return 0;
}

fn remove_key_path(key: &String) -> i32 {
    // Open the file in read-write mode
    let mut file = OpenOptions::new().read(true).write(true).open(&goto_key_paths_file_path().to_string_lossy().to_string()).unwrap();

    // Create a buffer to store the modified contents
    let mut buffer = Vec::new();

    // Seek to the beginning of the file
    if let Err(error) = file.seek(SeekFrom::Start(0)) {
        eprintln!("Error occured: {}", error);
        return -1;
    }

    // Iterate over the lines and exclude the line to be removed
    for line in io::BufReader::new(&file).lines() {
        // Write non-matching lines to the buffer
        if let Ok(ref ip) = line {
            let key_path_pair: Vec<&str> = ip.split(GOTO_KEY_PATH_DELIMITER).collect();
            if key_path_pair[0] != key {
                buffer.extend(line.unwrap().bytes());
                buffer.push(b'\n');
            }
        }
    }

    // Truncate the file to the current position (i.e., remove the remaining contents)
    let seek_current = file.seek(SeekFrom::Start(0)).unwrap();
    if let Err(error) = file.set_len(seek_current) {
        eprintln!("Error occured: {}", error);
        return -1;
    }

    // Write the modified contents back to the file
    if let Err(error) = file.write_all(&buffer) {
        eprintln!("Error occured: {}", error);
        return -1;
    }

    return 0;
}

fn add_key_path(key: &String, path: &String) -> i32 {
    // See if key already exists
    
    match get_file_reader_for_file(&goto_key_paths_file_path().to_string_lossy().to_string()) {
        Err(e) => {
            eprintln!("Could not read file {}: {}", goto_key_paths_file_path().display(), e);
            return -1;
        } Ok(reader) => {
            for line in reader.lines() {
                if let Ok(ip) = line {
                    // key|path
                    let key_path_pair: Vec<&str> = ip.split(GOTO_KEY_PATH_DELIMITER).collect();
                    if key_path_pair[0] == key {
                        eprintln!("key ({key}) already exists");
                        return -1;
                    }
                }
            }
        }
    }

    // Write new key and path pair
    let mut file_writer = OpenOptions::new().create(true).write(true).append(true).open(&goto_key_paths_file_path().to_string_lossy().to_string()).unwrap();

    // Expand the input path
    let expanded_path = canonicalize(path).unwrap().into_os_string().into_string().unwrap();

    // write
    if let Err(error) = writeln!(file_writer, "{key}{GOTO_KEY_PATH_DELIMITER}{expanded_path}") {
        eprintln!("Error occurred writing line: {}", error);
        return -1;
    }

    // Make sure the writer flushed all data
    if let Err(error) = file_writer.flush() {
        eprintln!("Could not flush file: {}", error);
        return -1;
    }

    return 0;
}

fn get_file_reader_for_file(path: &str) -> Result<BufReader<File>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

