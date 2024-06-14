# Auto Unit Test Generator

This project is designed to automatically create and maintain correspondence between the source files of a Swift project and their unit tests. It generates empty files with appropriate names in the correct folder structure and automates the synchronization between the hierarchy of source files and unit tests.

## Features

- **File Creation**: Automatically creates empty files with appropriate names in the correct folder structure.
- **Hierarchy Maintenance**: Automates the maintenance of correspondence between the hierarchy of source files and unit tests.

## How It Works

### Meta-programming Process

1. **Program Initialization**:
    - The program starts in a directory containing `Sources`, `Tests`, and `Unresolved` folders.

2. **Traversing the Sources Directory**:
    - The program iterates through files and directories in `Sources` and retrieves paths.

3. **Processing Files and Directories**:
    - **Directory**: Recursively processes the directory.
    - **Swift File**: Generates a unit test name based on the structure, class, or enum name.
        - Unit test name: `StructNameTests.swift`
    - Files and directories starting with `.` or not having the `.swift` extension are ignored.

4. **Switching to the Tests Directory**:
    - After traversing all files in the `Sources` directory, the program switches to the corresponding directory in `Tests`.
    - Compares the number of files in the current `Sources` and `Tests` directories:
        - If the file counts match, the directory is skipped.
        - If the file counts do not match, extra tests in `Tests` are moved to `Unresolved`.

### Handling Edge Cases

1. **File Renaming**:
    - **Problem**: If a file in `Sources` is renamed, its unit test in `Tests` becomes "extra" and is moved to `Unresolved`. A new unit test with the correct name is created in `Tests`.
    - **Solution**: Track not only file movements but also renaming. Use a map where the key is the original file name and the value is the new name and path.

2. **Folder Structure Changes**:
    - **Problem**: If the folder structure in `Sources` changes (e.g., a folder is renamed or moved), the folder structure in `Tests` is not automatically updated.
    - **Solution**: Track changes in the `Sources` folder structure and apply corresponding changes to `Tests`. This may require analyzing the folder hierarchy.

3. **Git-ignored Files**:
    - **Problem**: Handling files ignored by the version control system.
    - **Solution**: Exclude files specified in `.gitignore` from processing.

4. **File Movement**:
    - **Problem**: When a file is moved from `Sources` to `Unresolved`, its unit test also ends up in `Unresolved`. However, the program continues to create a new empty unit test in `Tests`, leading to duplication and inconsistency.
    - **Solution**: Use a map (associative array) to store information about moved files. Before creating a unit test in `Tests`, the program checks for a corresponding file in `Unresolved` using this map. If found, a new unit test is not created, preventing duplication.

### Important Aspects

- **Templating**: Unit tests are created using the `ExampleSwiftUnitTest.swift` template, rendered with the Handlebars library.
- **File Operations**: The program extensively uses file system operations to read, write, and create directories and files.
- **File and Directory Name Validation**: Functions are implemented to ensure that files and directories meet the specified criteria (e.g., not hidden, correct extension).

## What's Done

- Automatic creation of unit test files based on the structure of source files.
- Recursive traversal of directories to maintain the hierarchy of unit tests.
- Handling basic edge cases such as file renaming and structure changes.

## What's Missing

- **Enhanced Renaming and Movement Tracking**: Implementing a map to track file renaming and movement to ensure unit tests are correctly associated with their source files.
- **Folder Structure Synchronization**: More robust handling of changes in the folder structure to keep `Tests` in sync with `Sources`.
- **Git Ignore Handling**: Properly excluding files specified in `.gitignore` from processing.
- **Comprehensive Error Handling**: More extensive error handling and logging to ensure stability and easier debugging.
- **Documentation and User Guide**: Comprehensive documentation and user guide for easy setup and use.

## Getting Started

### Prerequisites

- Rust (for running the project)

### Running the Project

1. Clone the repository.
2. Navigate to the project directory.
3. Ensure you have a `Sources`, `Tests`, and `Unresolved` directory in the project root.
4. Run the project using Cargo:
   ```sh
    cargo run
   ```

## Contributions

Contributions are welcome! Please feel free to submit a pull request or open an issue for any bugs or feature requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contacts

**Sporykhin Svyatoslav** svyat1996@gmail.com
