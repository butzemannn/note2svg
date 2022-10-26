use clap::Parser;
use std::fmt::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str;
use glob::glob;
use roxmltree::Document;

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
        let session = load_xml_file(&note)?;
        //process_xml(session)
        //    .ok_or(Error)?;
    }

    Ok(())
}

/// Loads the session.plist file from the given file_path
fn load_xml_file(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let xml = read_to_string(file_path)?;
    let xml = Document::parse(&xml[..])?;
    let root = xml.root_element();
    dbg!(xml);
    Ok(())
}


fn process_xml() {

}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_load_xml_file() {
        let file_path = PathBuf::from("data/note/Note 5. Aug 2022/Session.xml");
        let session = load_xml_file(&file_path)
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

