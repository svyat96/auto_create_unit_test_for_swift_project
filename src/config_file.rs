use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

/// Reads the configuration from a file.
///
/// If the file does not exist, it is created with default values.
///
/// # Arguments
///
/// * `path` - Path to the configuration file.
///
/// # Returns
///
/// Result containing the `InitFile` structure or an `std::io::Error`.
pub fn read_config(path: &Path) -> Result<InitFile, std::io::Error> {
    if !path.exists() {
        // Create the configuration file with default values if it does not exist.
        create_init_file(path)?;
    }

    // Read the contents of the configuration file.
    let config_str = fs::read_to_string(path)?;

    // Deserialize the contents of the file into an `InitFile` structure.
    let config: InitFile = serde_json::from_str(&config_str)?;

    Ok(config)
}

/// Creates the `InitFile.json` file with default values.
///
/// # Arguments
///
/// * `path` - Path to the file to be created.
///
/// # Returns
///
/// Result of the operation as `std::io::Result`.
fn create_init_file(path: &Path) -> std::io::Result<()> {
    // Create an `InitFile` structure with default values.
    let config_str = serde_json::to_string_pretty(&InitFile::default())?;

    // Write the structure to the file in JSON format.
    fs::write(path, config_str)?;

    Ok(())
}

/// Structure for storing configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct InitFile {
    /// Name of the project to which tests are connected.
    pub tested_project: String,
    /// Path to which paths for test folders are added.
    pub parent_path: String,
    /// Name of the folder where tests are saved.
    pub folder_tests_name: String,
    /// Name of the folder with tests without association with working files.
    pub folder_unresolved_tests: String,
    /// Path to the project files.
    pub folder_with_files_project: String,
    /// Names of folders/files to be skipped.
    pub folder_file_exceptions: Vec<String>,
    /// File extensions to be processed.
    pub file_extension: Vec<String>,
}

/// Implementation of the `Default` trait for the `InitFile` structure, which returns the structure with default values.
impl Default for InitFile {
    fn default() -> Self {
        InitFile {
            tested_project: String::from("Project to which tests are connected"),
            parent_path: String::from("Path to which paths for test folders are added"),
            folder_tests_name: String::from("Folder where tests are saved"),
            folder_unresolved_tests: String::from("Folder with tests without association with working files"),
            folder_with_files_project: String::from("Path to project files"),
            folder_file_exceptions: vec![String::from("Names of folders/files to be skipped")],
            file_extension: vec![String::from(".swift")],
        }
    }
}

/// Implementation of the `PartialEq` trait for the `InitFile` structure, which compares two structures for equality.
impl PartialEq for InitFile {
    fn eq(&self, other: &Self) -> bool {
        self.tested_project == other.tested_project
            && self.parent_path == other.parent_path
            && self.folder_tests_name == other.folder_tests_name
            && self.folder_unresolved_tests == other.folder_unresolved_tests
            && self.folder_with_files_project == other.folder_with_files_project
            && self.folder_file_exceptions == other.folder_file_exceptions
            && self.file_extension == other.file_extension
    }
}

/// Structure for storing data about a unit test.
pub struct UnitSwift {
    /// Project name.
    pub project_name: String,
    /// Unit test class name.
    pub class_name: String,
    /// Unit test creation date.
    pub created_date: String,
}

/// Implementation of methods for the `UnitSwift` structure.
impl UnitSwift {
    /// Converts `UnitSwift` data to JSON format.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` containing the data in JSON format.
    pub fn get_json(&self) -> serde_json::Value {
        json!({
            "class_name": &self.class_name,
            "created_date": &self.created_date,
            "project_name": &self.project_name,
        })
    }
}

/// Structure for storing folder paths.
pub struct FolderPaths {
    /// Parent path.
    pub parent: PathBuf,
    /// Path to the source folder.
    pub sources: PathBuf,
    /// Path to the test folder.
    pub tests: PathBuf,
    /// Path to the unresolved tests folder.
    pub unresolved: PathBuf,
}

/// Implementation of the `From` trait for converting `InitFile` to `FolderPaths`.
impl From<InitFile> for FolderPaths {
    /// Converts the `InitFile` structure to `FolderPaths`.
    ///
    /// # Arguments
    ///
    /// * `value` - The `InitFile` structure.
    ///
    /// # Returns
    ///
    /// A `FolderPaths` structure.
    fn from(value: InitFile) -> Self {
        FolderPaths::from(&value) // Call another `From` method for a reference to `InitFile`.
    }
}

/// Implementation of the `From` trait for converting a reference to `InitFile` to `FolderPaths`.
impl From<&InitFile> for FolderPaths {
    /// Converts a reference to `InitFile` to `FolderPaths`.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the `InitFile` structure.
    ///
    /// # Returns
    ///
    /// A `FolderPaths` structure.
    fn from(value: &InitFile) -> Self {
        let parent_path = PathBuf::from(&value.parent_path); // Create a `PathBuf` from the parent path.

        FolderPaths {
            parent: parent_path.clone(), // Save the parent path.
            sources: parent_path.join(&value.folder_with_files_project), // Form the path to the source folder.
            tests: parent_path.join(&value.folder_tests_name), // Form the path to the test folder.
            unresolved: parent_path.join(&value.folder_unresolved_tests), // Form the path to the unresolved tests folder.
        }
    }
}
