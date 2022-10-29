use clap::Parser;
use std::fmt::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str;
use glob::glob;
use xmltree::Element;

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
        let points = load_xml_file(&note)?;
        let points_processed = process_points(points)?;
    }

    Ok(())
}

/// Loads the session.plist file from the given file_path
/// :returns: String containing the curvepoints
fn load_xml_file(file_path: &PathBuf) -> Result<(String), Box<dyn std::error::Error>> {
    let xml = read_to_string(file_path)?;
    let xml: &str = &xml[..];
    let mut elm = Element::parse(xml.as_bytes()).unwrap();
    //elm = elm.children[0];
    // TODO: search for elements instead of hardcoding
    let elm = elm.children[0].as_element().unwrap();
    let elm = elm.children[7].as_element().unwrap();
    let elm = elm.children[51].as_element().unwrap();
    let elm = elm.children[9].as_element().unwrap();
    let text = elm.children[0].as_text().unwrap();

    // clean text
    let text = text.replace("\n", "");
    let text = text.replace("\t", "");

    Ok((text))
}


fn process_points(points: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Decode with base64
    let points_dec = base64::decode(points)?;

    Ok((points_dec))
}


#[cfg(test)]
mod test {
    use clap::error::ContextValue::String;
    use super::*;

    #[test]
    fn test_process_points() {
        //let points = "q9b8QpLeqEKQ7vtCh76nQjeeAEN1o6JCVREDQ0dhnEItFwRDUcSZQn0MBUOC7ZZCIsIFQ2lilEKgbQZDtPuRQoOwBkNoso9CVckGQ9MUjkKe6wZDlNmLQklQBkO29opCIsIFQ45li0J49gRDXASMQrs+BEO1yo9CicQDQ6QklUJobQNDa/WYQiAtA0PAtZ1CsDwDQ/kcokI5WQND";
        let points = "q9b8QpLe";
        let points_processed = process_points(points.to_string()).unwrap();
        dbg!(points_processed);
    }

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

