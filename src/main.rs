use clap::Parser;
use std::fmt::Error;
use std::path::{Path, PathBuf};
use glob::glob;
use itertools::Itertools;
use plist::Value;
use byteorder::{ByteOrder, LittleEndian};

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


/// iterate over the extracted notes and find
fn iterate_extracted_notes() -> Result<(), Box<dyn std::error::Error>>{
    for note in glob("data/note/*")? {
        let mut note = note?;
        note.push("Session.plist");

        // load and process curve points
        let points = load_points(&note)
            .ok_or(Error)?;

        let points_processed = process_points(points)?;
    }
    Ok(())
}

/// Loads the session.plist file from the given path
/// :returns: Vec<8> containing the curve points

fn load_points(path: &Path) -> Option<Vec<u8>> {
    let file = Value::from_file(&path).ok()?;
    let file = file.as_dictionary()?;

    let points = file
        .get("$objects")?
        .as_array()?
        .get(51)?
        .as_dictionary()?
        .get("curvespoints")?
        .as_data()?;

    Some(points.to_owned())
}


/// Process points by generating float values
fn process_points(points: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    for chunk in &points.into_iter().chunks(4) {
        let mut buffer: [u8; 4] = [0; 4];
        // Convert to float with little endianness
        for (i, byte) in chunk.into_iter().enumerate() {
            buffer[i] = byte;
        }
        let number = LittleEndian::read_f32(&buffer);
        dbg!(buffer);
    }
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_points() {
        let path = PathBuf::from(r"data/note/Note 5. Aug 2022/Session.plist");
        let points = load_points(&path).unwrap();
    }

    #[test]
    fn test_iterate_extracted_notes() {
        iterate_extracted_notes()
            .expect("Could not iterate over notes.");
    }

    #[test]
    fn test_process_points() {
        let mut points: Vec<u8> = Vec::new();
        points.push(217);
        points.push(99);
        points.push(44);
        points.push(1);

        process_points(points).unwrap();
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

