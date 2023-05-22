use crate::utilities::parse;
use serde_yaml;
use std::collections::HashMap as Map;
use std::path::PathBuf;

/// Implements the describe command, where the user can view a summary of the playbook
/// and the full contents of the playbook.
///
/// Flag Verbose:
/// Return the full contents of the playbook instead of jsut the summary
///
/// # Errors
/// Returns an error if playbook does not exist
/// Returns an error if the playbook directory does not exit
/// Returns an error if yaml can't be parsed
/// Returns an error if there is no name field in the playbook
pub fn call_describe(
    books: &Map<u64, String>,
    verbose: &bool,
    playbook: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut results = "".to_owned();
    for (i, book) in books {
        let mut book_path = playbook.clone();
        book_path.push(book);
        if *verbose {
            let book_content = parse::contents(&book_path)?;
            results.push_str(&i.to_string());
            results.push_str(": ");
            results.push_str(&book);
            results.push_str("\n===========================\n");
            results.push_str(&book_content);
        } else {
            let book_name = parse::unwrap_name(&book_path)?;
            let book_envs = parse::unwrap_envs(&book_path)?;
            results.push_str(&i.to_string());
            results.push_str(": ");
            results.push_str(&book);
            results.push_str(" - ");
            results.push_str(&serde_yaml::to_string(&book_name).unwrap());
            if !book_envs.is_empty() {
                results.push_str("Envs: ");
                results.push_str(&book_envs.join(", "));
            }
            results.push_str("\n");
        }
    }
    Ok(results)
}
