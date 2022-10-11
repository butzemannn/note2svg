use std::path::Path;
use std::process::Command;
use clap::Parser;


#[derive(Parser)]
struct Cli {
    file_path: String,
}

fn main() {
    let args = Cli::parse();
    let file = Path::new(&args.file_path);
    extract_data(file);

}
fn extract_data(file: &Path) {
    // Extracts note data into data/note/<Name>
    let folder = &file.parent().unwrap();
    let file = &file.to_str().unwrap();
    let folder = folder.to_str().unwrap();

    let mut result: String = folder.to_owned();
    result.push_str("/note");

    let output = Command::new("/usr/bin/unzip")
        .args(["-o", &file, "-d", &result])
        .output()
        .expect("failed to unzip note");

    dbg!(output);
}

