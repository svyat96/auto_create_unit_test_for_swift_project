use chrono::Local;
use handlebars::Handlebars;
use std::convert::From;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf, StripPrefixError},
};

mod config_file;

use config_file::*;

/// The main function of the program.
fn main() {
    // Path to the configuration file.
    let path_buf = PathBuf::from("./InitFile.json");

    // Read the configuration from the file.
    println!("Read the configuration from the file: Run");
    let config = read_config(&path_buf).unwrap();

    // Get paths to the source, test, and unresolved test folders.
    let folder_paths = FolderPaths::from(&config);

    // Recursively traverse the source folder and create unit tests.
    println!("Recursively traverse the source folder and create unit tests: Run");
    match traverse_directory(&folder_paths.sources, &config) {
        Ok(_) => println!("Config: {:#?}", config),
        Err(err) => println!("Recursively traverse the source folder and create unit tests: Error:{:#?}", err),
    }
}

/// Recursively traverses the specified directory and creates unit tests.
///
/// # Arguments
///
/// * `path` - Path to the directory.
/// * `init_file` - Configuration file.
fn traverse_directory(path: &PathBuf, init_file: &InitFile) -> Result<(), std::io::Error> {
    // Get the list of entries in the directory.

    match fs::read_dir(path) {
        Err(err) => {
            return err.
        },
        Ok(dir) => {
            for entry in dir {
                let entry = entry?;
                let path = entry.path();
        
                // Check if the file/folder name meets the requirements.
                if !is_valid_name(&path, init_file) {
                    continue;
                }
        
                // Check if the entry is a directory.
                if path.is_dir() && !is_hidden(&path) {
                    println!("Directory: {}", path.display());
                    // Recursively call the function for the subdirectory.
                    traverse_directory(&path, init_file)?;
                } else if path.is_file() && !is_hidden(&path) {
                    println!("File: {}", path.display());
                    // Create a unit test for the file if it does not already exist.
                    match create_unit_test_if_need(&path, init_file) {
                        Err(err) => {
                            panic!("Create unit test!: {}", err)
                        }
                        Ok(v) => v,
                    };
                }
            }
            return Ok(());
        }
    };

    // for entry in fs::read_dir(path)? {
    //     let entry = entry?;
    //     let path = entry.path();

    //     // Check if the file/folder name meets the requirements.
    //     if !is_valid_name(&path, init_file) {
    //         continue;
    //     }

    //     // Check if the entry is a directory.
    //     if path.is_dir() && !is_hidden(&path) {
    //         println!("Directory: {}", path.display());
    //         // Recursively call the function for the subdirectory.
    //         traverse_directory(&path, init_file)?;
    //     } else if path.is_file() && !is_hidden(&path) {
    //         println!("File: {}", path.display());
    //         // Create a unit test for the file if it does not already exist.
    //         match create_unit_test_if_need(&path, init_file) {
    //             Err(err) => {
    //                 panic!("Create unit test!: {}", err)
    //             }
    //             Ok(v) => v,
    //         };
    //     }
    // }
    // return Ok(());
}

/// Gets the path to the unit test and its name.
///
/// # Arguments
///
/// * `path` - Path to the source file.
/// * `config` - Configuration file.
///
/// # Returns
///
/// A tuple containing the path to the unit test and its name.
fn get_unit_test_path(
    path: &Path,
    config: &InitFile,
) -> Result<(PathBuf, String), StripPrefixError> {
    let folder_paths = FolderPaths::from(config);
    let sources_head = &folder_paths.sources; // Path to the source folder.

    // Get the relative path of the source file.
    let stripped_path = path.parent().unwrap().strip_prefix(&sources_head)?;

    // Form the path to the unit test file.
    let mut test_path = folder_paths.tests.clone();
    test_path.push(stripped_path);

    // Form the name of the unit test file.
    let unit_test_name = path.file_stem().unwrap().to_str().unwrap().to_string() + "Tests.swift";
    let unit_test_path = test_path.join(&unit_test_name);

    Ok((unit_test_path, unit_test_name))
}

/// Creates a unit test.
///
/// # Arguments
///
/// * `unit_test_path` - Path to the unit test.
/// * `config` - Configuration file.
/// * `unit_test_name` - Name of the unit test.
///
/// # Returns
///
/// Result of the operation.
fn create_unit_test(
    unit_test_path: &Path,
    config: &InitFile,
    unit_test_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Extract the class name from the unit test file name.
    let class_name = Path::new(unit_test_name)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Get the current date and format it.
    let now = Local::now();
    let formatted_date = now.format("%d/%m/%Y").to_string();

    // Create a structure with data for the unit test template.
    let unit_swift = UnitSwift {
        project_name: config.tested_project.clone(),
        class_name,
        created_date: formatted_date,
    };

    // Path to the unit test template.
    let template_path = Path::new("./ExampleSwiftUnitTest.swift");

    // Render the unit test template.
    let result = _render_unit_test_template(template_path, unit_swift)?;

    // Create all necessary directories.
    fs::create_dir_all(unit_test_path.parent().unwrap())?;

    // Create the unit test file and write the generated code to it.
    let mut file = File::create(unit_test_path)?;
    file.write_all(result.as_bytes())?;
    Ok(())
}

/// Creates a unit test for the specified Swift file if it does not already exist.
///
/// # Arguments
///
/// * `path` - Path to the source file.
/// * `config` - Configuration file.
///
/// # Returns
///
/// Result of the operation.
fn create_unit_test_if_need(path: &Path, config: &InitFile) -> Result<(), std::io::Error> {
    // Variables to store the path to the unit test and its name.
    let unit_test_path: PathBuf;
    let unit_test_name: String;

    // Get the path to the unit test.
    match get_unit_test_path(path, config) {
        Ok(value) => (unit_test_path, unit_test_name) = value,
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                err.to_string(),
            ))
        }
    }

    // Check if the unit test already exists.
    if !unit_test_path.exists() {
        // Create the unit test.
        match create_unit_test(&unit_test_path, config, &unit_test_name) {
            Ok(_) => return Ok(()),
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    err.to_string(),
                ))
            }
        }
    }

    Ok(())
}

/// Renders the unit test template.
///
/// # Arguments
///
/// * `unit_test_path` - Path to the unit test template.
/// * `unit_swift` - Data structure for the template.
///
/// # Returns
///
/// Generated unit test code.
fn _render_unit_test_template(
    unit_test_path: &Path,
    unit_swift: UnitSwift,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();
    let template_name = &unit_swift.project_name;
    handlebars.register_template_file(template_name, unit_test_path)?;

    // Create a JSON object with data for the template.
    let data = unit_swift.get_json();

    // Render the template with data.
    let result = handlebars.render(template_name, &data)?;
    Ok(result)
}

/// Checks if the file/folder name meets the requirements.
///
/// # Arguments
///
/// * `path` - Path to the file or folder.
/// * `config` - Configuration file.
///
/// # Returns
///
/// A boolean indicating whether the name meets the requirements.
fn is_valid_name(path: &PathBuf, config: &InitFile) -> bool {
    let file_name = match path.file_name() {
        Some(name) => name.to_str().unwrap().to_string(),
        None => return false,
    };

    if path.is_dir() {
        // Check if the folder name is in the exceptions list.
        !config.folder_file_exceptions.contains(&file_name)
    } else if path.is_file() {
        // Check the file extension and if the file name is in the exceptions list.
        let file_extension = match path.extension() {
            Some(ext) => ext.to_str().unwrap_or("Empty").to_string(),
            None => String::from("Empty"),
        };
        !config.folder_file_exceptions.contains(&file_name)
            && config.file_extension.contains(&file_extension)
    } else {
        false
    }
}

/// Checks if the file is hidden.
///
/// # Arguments
///
/// * `path` - Path to the file.
///
/// # Returns
///
/// A boolean indicating whether the file is hidden.
fn is_hidden(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}