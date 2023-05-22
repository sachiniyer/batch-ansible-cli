use crate::utilities::parse;
use std::collections::HashMap as Map;
use std::path::PathBuf;

/// Implements the list command, where the user can view
/// all the available playbooks in a dir
///
/// Flag Verbose:
/// Will give the name field of the playbook as well as the playbook file name
///
/// # Erorrs
/// Returns an error if the directory does not exist
/// Returns an error if there is no name field in one of the playbooks (with -v)
pub fn call_list(
    verbose: &bool,
    files: &Map<u64, String>,
    playbook: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut results = "".to_owned();

    // sort the files by their primary key
    // and then iterate over them
    let mut files_sorted: Vec<(&u64, &String)> = files.iter().collect();
    files_sorted.sort_by(|a, b| a.0.cmp(b.0));

    for (i, file_name) in files_sorted.iter() {
        let mut book_path = playbook.clone();
        book_path.push(file_name.clone());
        if *verbose {
            let book_name = parse::unwrap_name(&book_path)?;
            let res = format!("{}: {} - {} \n", i, file_name, &book_name);
            results.push_str(&res);
        } else {
            let res = format!("{}: {}\n", i, file_name);
            results.push_str(&res);
        }
    }
    Ok(results)
}
