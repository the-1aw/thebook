use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

pub fn with_match(filename: &str) -> File {
    let greeting_file_result = File::open(filename);
    match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(file) => file,
                Err(err) => panic!("Problem creating the file: {err:?}"),
            },
            _ => panic!("Problem opening the file: {err:?}"),
        },
    }
}

pub fn with_unwrap_or_else(path: &str) -> File {
    File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    })
}

pub fn with_match_except_mix(path: &str) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                File::create(path).expect(&format!("{path} was not found and cannot be created"))
            }
            _ => panic!("Problem opening file: {err:?}"),
        },
    }
}

pub fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut username_file = match File::open(path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_propagate_err(path: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(path)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn read_username_from_file_shortcut(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
