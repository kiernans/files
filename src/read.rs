use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let file = File::open(file_path).expect("File was not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
