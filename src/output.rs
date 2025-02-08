use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};

/// Writes the contents of each file in `scanned_files` to `output_file`
/// in the format:
///
/// -- file_path --file_name
/// <contents>
///
pub fn write_output(
    scanned_files: &[PathBuf],
    output_file: &str,
) -> io::Result<()> {
    // Open (or create) the output file, truncating any previous contents
    let mut out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)?;

    // For each path, read its contents, then write to the output
    for path in scanned_files {
        // Convert path to a string for printing
        let path_str = path.display().to_string();
        
        // Extract just the file name (or fallback)
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<no_name>");

        // Read the file
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;

        // Write the section header
        writeln!(out, "-- {} --{}", path_str, file_name)?;
        // Write the file contents
        writeln!(out, "{}", contents)?;
    }

    Ok(())
}