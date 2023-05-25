use serde_yaml;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// Implements the contents function which just gives the entire data of the file
///
/// # Errors
/// Returns an error if the path is not valid
pub fn contents(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path).unwrap_or_else(|_| file_not_found(path));
    Ok(contents)
}

/// Implements the unwrap function which takes a given file and returns a yaml object
///
/// # Errors
/// Returns an error if the file path is not valid
/// Returns an error if the file is not parsable
pub fn unwrap(path: &PathBuf) -> Result<serde_yaml::Value, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path).unwrap_or_else(|_| file_not_found(path));
    let deserialized = serde_yaml::from_str::<serde_yaml::Value>(&contents)
        .unwrap_or_else(|_| file_not_parsable(path));
    Ok(deserialized)
}

/// Implements the unwrap_name function which takes a given file and returns a string
/// with the value of the name field
///
/// # Errors
/// Returns an error if the file path is not valid
/// Returns an error if the file is not parsable
/// Returns an error if the name field is not there
pub fn unwrap_name(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(path).unwrap_or_else(|_| file_not_found(path));
    let mut yaml_str = String::new();
    file.read_to_string(&mut yaml_str)
        .unwrap_or_else(|_| file_not_parsable(path));
    let value_seq: serde_yaml::Sequence =
        serde_yaml::from_str(&yaml_str).unwrap_or_else(|_| file_not_parsable(path));
    let mut value = value_seq.get(0).unwrap();
    value = &value["name"];
    let output = value.as_str().unwrap();
    Ok(output.to_string())
}
/// Implements the unwrap_envs function which takes a given file and returns all the
/// ansible variables in it
///
/// # Errors
/// Returns an error if the file path is not valid
/// Returns an error if the file is not parsable
pub fn unwrap_envs(path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open(path).unwrap_or_else(|_| file_not_found(path));
    let mut yaml_str = String::new();
    file.read_to_string(&mut yaml_str)
        .unwrap_or_else(|_| file_not_parsable(path));
    let value_seq: serde_yaml::Sequence =
        serde_yaml::from_str(&yaml_str).unwrap_or_else(|_| file_not_parsable(path));
    let mut value = value_seq.get(0).unwrap();
    if !value.get("vars").is_some() {
        return Ok(Vec::new());
    }
    value = &value.get("vars").unwrap();
    let output = value.as_mapping().unwrap().values();
    let mut result = Vec::new();
    for i in output {
        let istr_res = i.as_str();
        match istr_res {
            Some(istr) => {
                if istr.contains("{{") && istr.contains("}}") {
                    let start = istr.find("{{").unwrap() + 2;
                    let end = istr.find("}}").unwrap();
                    let mut var = i.as_str().unwrap().get(start..end).unwrap();
                    var = var.trim();
                    result.push(var.to_string());
                }
            }
            None => {}
        }
    }
    Ok(result)
}

/// Implement error when file is not found
/// Will throw a panic
fn file_not_found(path: &PathBuf) -> ! {
    panic!(
        "File {} does not exist or is not readable",
        path.to_str().unwrap()
    )
}

/// Implement error when file is not parsable
/// Will throw a panic
fn file_not_parsable(path: &PathBuf) -> ! {
    panic!("File {} is not parsable", path.to_str().unwrap())
}
