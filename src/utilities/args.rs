use std::collections::HashMap as Map;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Implements the map files function, which takes a directory path
/// and returns a map of alphabetical order to file name.
///
/// # Arguments
/// * `path` - A path to a directory.
///
/// # Errors
/// Returns an error if the directory cannot be read.
pub fn map_files(path: &PathBuf) -> Result<Map<u64, String>, Box<dyn std::error::Error>> {
    let mut map = Map::new();
    let mut files = fs::read_dir(path)?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    files.retain(|f| f.to_str().unwrap().ends_with(".yaml"));
    files.sort();

    for (i, file) in files.iter().enumerate() {
        map.insert(i as u64, file.to_str().unwrap().to_string());
    }

    Ok(map)
}

/// Implements the name function which takes a file number and uses map_files to match it to a name
/// in the directory.
///
/// # Arguments
/// * `file_num` - A number corresponding to a file in the directory.
/// * `path` - A path to a directory.
///
/// # Errors
/// Returns an error if the directory cannot be read.
/// Returns an error if the file number does not exist.
pub fn map_name(file_num: &u64, path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let files = map_files(path)?;
    let file_name = files.get(file_num).unwrap();
    Ok(file_name.to_string())
}

/// Implements the map_num function which takes a file name and uses map_files to match it to a num
///
/// # Arguments
/// * `file_name` - A name corresponding to a file in the directory.
/// * `path` - A path to a directory.
///
/// # Errors
/// Returns an error if the directory cannot be read.
/// Returns an error if the file name does not exist.
pub fn map_num(file_name: &str, path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let files = map_files(path)?;
    let file_num = files.iter().find(|(_, name)| name == &file_name).unwrap().0;
    Ok(*file_num)
}

/// Implements the arg_parse function which takes either a vector of file nums, or file names and parses them
/// It returns a map of file nums to file names generated from the map_files function.
///
/// e.g. 1-3,5,7-9 or file1,file2,file3 or file1,2,3,file4
///
/// # Arguments
/// * `args` - A vector of strings corresponding to file names or file numbers.
/// * `path` - A path to a directory.
///
/// # Errors
/// Returns an error if the directory cannot be read.
/// Returns an error if the file name does not exist.
/// Returns an error if the file number does not exist.
pub fn arg_parse(
    args: &Vec<String>,
    path: &PathBuf,
) -> Result<Map<u64, String>, Box<dyn std::error::Error>> {
    let mut map = Map::new();
    let mut names = Vec::new();
    let mut nums = Vec::new();

    for arg in args {
        if arg.contains("-") {
            let range: Vec<&str> = arg.split("-").collect();
            if range.len() == 2
                && range[0].parse::<u64>().is_ok()
                && range[1].parse::<u64>().is_ok()
            {
                let start = range[0].parse::<u64>()?;
                let end = range[1].parse::<u64>()?;
                for i in start..=end {
                    nums.push(i);
                }
            } else {
                names.push(arg.to_string());
            }
        } else if arg.parse::<u64>().is_ok() {
            nums.push(arg.parse::<u64>()?);
        } else {
            names.push(arg.to_string());
        }
    }

    for name in names {
        let num = map_num(&name, path)?;
        map.insert(num, name);
    }
    for num in nums {
        let name = map_name(&num, path)?;
        map.insert(num, name);
    }

    Ok(map)
}

/// Implements the arg_parse_env function which takes a lists of comma seperated items, where
/// the first value is the playbook, and the rest are environment variables.
/// it returns file nums as a primary key, file names as a secondary key, and a vector of environment variables.
///
/// # Arguments
/// * `args` - A vector of strings corresponding to file names or file numbers and env vars
/// * `path` - A path to a directory.
///
/// # Errors
/// Returns an error if the directory cannot be read.
/// Returns an error if the file name does not exist.
/// Returns an error if the file number does not exist.
/// Returns an error if the env var is misformatted.
pub fn arg_parse_env(
    args: &Vec<String>,
    path: &PathBuf,
) -> Result<Map<u64, (String, Map<String, String>)>, Box<dyn std::error::Error>> {
    let mut map = Map::new();
    for arg in args {
        if arg.contains(",") {
            let env_vars: Vec<&str> = arg.split(",").collect();
            if env_vars.len() > 1 {
                let playbook = env_vars[0];
                let mut env_map = Map::new();
                for env_var in env_vars.iter().skip(1) {
                    let env_var: Vec<&str> = env_var.split("=").collect();
                    if env_var.len() == 2 {
                        env_map.insert(env_var[0].to_string(), env_var[1].to_string());
                    } else {
                        return Err("Environment variable must be in the format KEY=VALUE".into());
                    }
                }
                let playbooks = arg_parse(&vec![playbook.to_string()], path)?;
                for (num, name) in playbooks {
                    map.insert(num, (name, env_map.clone()));
                }
            }
        } else {
            let playbooks = arg_parse(&vec![arg.to_string()], path)?;
            for (num, name) in playbooks {
                map.insert(num, (name, Map::new()));
            }
        }
    }
    Ok(map)
}
