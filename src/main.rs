use clap::Parser;
use plist::{Dictionary, Value};
use std::fmt::Error;
use std::path::PathBuf;
use glob::glob;

mod utils;


/// Command line parameters. Currently only file_path is supported
#[derive(Parser)]
struct Cli {
    file_path: String,
}

fn main() {
    let args = Cli::parse();
    utils::extract_data(&args.file_path)
        .expect("Could not extract data.");

}

/// Loads the session.plist file from the given file_path
fn load_session(file_path: &PathBuf) -> Result<Dictionary, Box<dyn std::error::Error>> {
    let session = Value::from_file(file_path)?;

    // Convert session to dict and return with ownership
    let session = session
        .as_dictionary()
        .ok_or(Error)?
        .to_owned();

    Ok(session)
}

/// iterate over the extracted notes and find
fn iterate_extracted_notes() -> Result<(), Box<dyn std::error::Error>>{
    for note in glob("data/note/*")? {
        let mut note = note?;
        note.push("Session.plist");
        let session = load_session(&note)?;
        process_session(session)
            .ok_or(Error)?;
    }

    Ok(())
}

fn process_session(session: Dictionary) -> Option<()>{
    let curve_points = session
        .get("$objects")?
        .as_array()?;

    let curve_points = curve_points
        .get(51)?
        .as_dictionary()?
        .get("curvespoints")?;

    let curve_points = curve_points.as_data()?;


    Some(())
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_load_session() {
        let file_path = PathBuf::from("data/note/Note 5. Aug 2022/Session.plist");
        let session = load_session(&file_path)
            .expect("Could not load session");
    }

    #[test]
    fn test_iterate_extracted_notes() {
        iterate_extracted_notes()
            .expect("Could not iterate over notes.");
    }
}

/* steps:
 1. Iterate over all files in the notes directory
 2. Load Session.plist
 3. Read as dictionary and get curve points section
 4. Decode curve points with base64
 5. Convert curve points with float32 and little endian
 6. Write floats into list, 2 following floats can be interpreted as a 2D point
 7. Convert points into svg file format
*/

