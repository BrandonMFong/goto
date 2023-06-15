/*
 * author: Brando
 * date: 6/14/23
 */

use std::env;
use std::process;
use home;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{self, prelude::*};
use std::fs::canonicalize;

static ARG_GETPATH: &'static str = "getpath";
static ARG_GETKEY: &'static str = "getkey";
static ARG_ADD: &'static str = "add";
static ARG_REMOVE: &'static str = "rm";

static GOTO_KEY_PATH_DELIMITER: &'static str = "|";

fn help() {
    let args: Vec<String> = env::args().collect();
    let tool_name = &args[0];
    println!("usage: {tool_name} <arg>");
    println!("\targuments:");
    println!("\t\t{ARG_GETPATH}: {tool_name} {ARG_GETPATH} <key>");
    println!("\t\t{ARG_GETKEY}: {tool_name} {ARG_GETKEY} <path>");
    println!("\t\t{ARG_ADD}: {tool_name} {ARG_ADD} <key> <path>");
    println!("\t\t{ARG_REMOVE}: {tool_name} {ARG_REMOVE} <key>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments");
        help();
        process::exit(1);
    }

    let mut error = 0;
    if args[1].eq(ARG_GETPATH) {
        error = print_path_for_key(&args[2]);
    } else if args[1].eq(ARG_GETKEY) {

    } else if args[1].eq(ARG_ADD) {
       error = add_key_path(&args[2], &args[3]);
    } else if args[1].eq(ARG_REMOVE) {

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

fn print_path_for_key(key: &String) -> i32 {
    // See if key already exists
    if let Ok(lines) = read_lines(goto_key_paths_file_path()) {
        for line in lines {
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

    return -1;
}

fn add_key_path(key: &String, path: &String) -> i32 {
    // See if key already exists
    if let Ok(lines) = read_lines(goto_key_paths_file_path()) {
        for line in lines {
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

    // Write new key and path pair
    let mut file_writer = OpenOptions::new().create(true).write(true).append(true).open(goto_key_paths_file_path()).unwrap();

    // Expand the input path
    let expanded_path = canonicalize(path).unwrap().into_os_string().into_string().unwrap();

    // write
    match writeln!(file_writer, "{key}{GOTO_KEY_PATH_DELIMITER}{expanded_path}") {
        Ok(_) => { } Err(error) => {
            eprintln!("Error occurred writing line: {}", error);
            return -1;
        }
    }

    // Make sure the writer flushed all data
    match file_writer.flush() {
        Ok(_) => { } Err(error) => {
            eprintln!("Could not flush file: {}", error);
            return -1;
        }
    }

    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

