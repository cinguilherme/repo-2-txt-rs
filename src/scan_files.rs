use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    // If the file name starts with '.', treat it as hidden
    if let Some(name) = entry.file_name().to_str() {
        name.starts_with('.')
    } else {
        false
    }
}

pub fn scan_path(
    root_path: &str,
    extensions: &[String],
    ignore_hidden: bool,
) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    // Use `filter_entry` to exclude hidden directories/files upfront
    let walker = WalkDir::new(root_path).into_iter().filter_entry(|e| {
        if ignore_hidden {
            !is_hidden(e)
        } else {
            true
        }
    });

    for entry in walker.filter_map(Result::ok) {
        // Only collect actual files
        if entry.file_type().is_file() {
            if extensions.is_empty() {
                files.push(entry.path().to_path_buf());
            } else if let Some(ext) = entry.path().extension().and_then(|os| os.to_str()) {
                // The user might pass `.rs`, so compare accordingly
                let file_ext = format!(".{}", ext);
                if extensions.contains(&file_ext) {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    files
}