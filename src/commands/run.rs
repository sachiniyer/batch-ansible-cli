use std::collections::HashMap as Map;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Implements the run command, where a user can run a playbook(s)
///
/// Flag Verbose:
/// Instead of just viewing whether a command succeeded or failed, view all of stdio
///
/// # Errors
/// Returns an error if the playbook(s) is not found
/// Returns an error if the playbook directory does not exist
/// Returns an error if the inventory file is not found
///
/// Sample command that will be run
/// ansible-playbook -i ../inventory.yaml install_ior.yaml
pub fn call_run(
    books: &Map<u64, (String, Map<String, String>)>,
    verbose: &bool,
    playbook_dir: &PathBuf,
    inventory: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut results = "".to_owned();

    let mut args = Vec::new();
    args.push("-i".to_owned());
    args.push(inventory.to_str().unwrap().to_owned());

    for (i, book_map) in books {
        let book = book_map.0.clone();
        let envs = book_map.1.clone();
        let mut book_path = playbook_dir.clone();
        book_path.push(book.clone());
        args.push(book_path.to_str().unwrap().to_owned());

        if *verbose {
            if let Ok(true) =
                run_command_verbose("ansible-playbook".to_owned(), args.clone(), envs.clone())
            {
                results.push_str(&format!("{}: {} - Success\n", i, book));
            } else {
                results.push_str(&format!("{}: {} - Failed\n", i, book));
            }
        } else {
            if let Ok(true) = run_command("ansible-playbook".to_owned(), args.clone(), envs.clone())
            {
                results.push_str(&format!("{}: {} - Success\n", i, book));
            } else {
                results.push_str(&format!("{}: {} - Failed\n", i, book));
            }
        }
        args.pop();
    }
    Ok(results)
}

/// Implements a simple run command. Returns true if the command succeeded, false otherwise
///
/// # Errors
/// Returns an error if the command fails to execute
fn run_command(
    cmd: String,
    mut args: Vec<String>,
    envs: Map<String, String>,
) -> Result<bool, Box<dyn std::error::Error>> {
    // take envs map, and create a vector of strings, where you push -e and then key=value
    for (key, value) in envs {
        args.push(format!("-e {}={}", key, value));
    }
    let status = Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Failed to execute command");
    if status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Implements a verbose run command. Returns true if the command succeeded, false otherwise
/// Prints all of the stdio of the command
///
/// # Errors
/// Returns an error if the command fails to execute
/// Returns an error if the command fails to open stdout
/// Returns an error if the command fails to open stderr
/// Returns an error if the command fails to wait
fn run_command_verbose(
    cmd: String,
    mut args: Vec<String>,
    envs: Map<String, String>,
) -> Result<bool, Box<dyn std::error::Error>> {
    for (key, value) in envs {
        args.push(format!("-e {}={}", key, value));
    }
    let mut command = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let stdout = command.stdout.take().expect("Failed to open stdout");
    let stderr = command.stderr.take().expect("Failed to open stdout");

    let stdoutreader = BufReader::new(stdout);
    let stderrreader = BufReader::new(stderr);

    let stdoutlines = stdoutreader.lines();
    let stderrlines = stderrreader.lines();

    for line in stdoutlines.chain(stderrlines) {
        if let Ok(line) = line {
            println!("{}", line);
        }
    }

    let status = command.wait().expect("Failed to wait for command");
    if status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}
