use clap::{Arg, Command, ArgAction};

mod scan_files;
mod output;
use scan_files::scan_path;
use output::write_output;

fn main() {
    let matches = Command::new("repo-2-text-rs")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("Converts a repo’s files into a single text output")
        .arg(
            Arg::new("path")
                .help("Path to the root folder")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("types")
                .long("types")
                .help("File extensions to include (e.g. .rs, .md, etc.)")
                // Accept one or more occurrences
                .num_args(1..) 
                // If you’d like to accept multiple values separated by commas/spaces
                .use_value_delimiter(true),
        )
        .arg(
            Arg::new("ignore-hidden-folders")
                .long("ignore-hidden-folders")
                .help("Skips hidden folders")
                // For booleans in Clap 4, use action instead of .takes_value()
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Extract values
    let path = matches.get_one::<String>("path").unwrap();
    let extensions = matches
        .get_many::<String>("types")
        .map(|vals| vals.cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    let ignore_hidden = matches.get_flag("ignore-hidden-folders");

    println!("Path: {}", path);
    println!("Types: {:?}", extensions);
    println!("Ignore hidden folders: {}", ignore_hidden);

    let scans = scan_path(&path, &extensions, ignore_hidden);

    write_output(&scans, "output.txt").expect("Failed to write output");

    // continue with your directory walking / output logic...
}

