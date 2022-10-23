use std::path::Path;
use std::process::Command;
use clap::Parser;
use plist::{Dictionary, Value};
use std::fmt::Error;


#[derive(Parser)]
struct Cli {
    file_path: String,
}

fn main() {
    let args = Cli::parse();
    extract_data(args.file_path)
        .expect("Could not extract data.");

}

fn extract_data(file_path: String) -> Option<()> {
    // Extracts note data into data/note/<Name>
    let file = Path::new(&file_path);
    let folder = file.parent()?;

    let file = file.to_str()?;

    // Append note subdirectory as extraction folder
    let mut folder = folder.to_str()?.to_owned();
    folder.push_str("/note");

    // Extract file into folder
    Command::new("/usr/bin/unzip")
        .args(["-o", &file, "-d", &folder]);

    Some(())
}

// Loads the session.plist file from the given file_path
fn load_session(file_path: String) -> Result<Dictionary, Box<dyn std::error::Error>> {
    let session = Value::from_file(file_path)?;

    // Convert session to dict and return with ownership
    let session = session
        .as_dictionary()
        .ok_or(Error)?
        .to_owned();

    Ok(session)
}


// steps:
// 1. Iterate over all files in the notes directory
// 2. Load Session.plist
// 3. Read as dictionary and get curve points section
// 4. Decode curve points with base64
// 5. Convert curve points with float32 and little endian
// 6. Write floats into list, 2 following floats can be interpreted as a 2D point
// 7. Convert points into svg file format

