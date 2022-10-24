use std::path::Path;
use std::process::Command;

/// Extracts note data into data/note/<Name>
pub fn extract_data(file_path: &str) -> Option<()> {
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
