use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::Local;
use handlebars::{template, Handlebars};

mod config_file;

use config_file::*;
use serde_json::json;

fn main() {
    let path_buf = PathBuf::from("./InitFile.json");

    println!("Path_buf: {:#?}", path_buf);

    let init_file: InitFile = read_init_file(&path_buf, create_init_file).unwrap();

    let parent_path = PathBuf::from(&init_file.parent_path);

    let sources_folder = parent_path.join(&init_file.folder_with_files_project);
    let test_folder = parent_path.join(&init_file.folder_tests_name);
    let unresolved_folder = parent_path.join(&init_file.folder_unresolved_tests);

    println!("Sources_folder: {:#?}", sources_folder);
    println!("Test_folder: {:#?}", test_folder);
    println!("Unresolved_folder: {:#?}", unresolved_folder);

    match traverse_directory(&sources_folder, &init_file) {
        Ok(_) => println!("{:#?}", init_file),
        Err(err) => println!("{:#?}", err),
    }
}

// Функция для рекурсивного обхода папок
fn traverse_directory(path: &PathBuf, init_file: &InitFile) -> Result<(), std::io::Error> {
    // Получаем список записей в папке
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if !is_valid_name(&path, init_file) {
            continue;
        }

        // Проверяем, является ли запись папкой
        if path.is_dir() && !is_hidden(&path) {
            println!("Директория: {}", path.display());
            // Рекурсивно вызываем функцию для подпапки
            traverse_directory(&path, init_file)?;
        } else if path.is_file() && !is_hidden(&path) {
            println!("Файл: {}", path.display());
            // Здесь можно добавить обработку файлов
            create_unit_test_if_need(&path, init_file);
        }
    }

    return Ok(());
}

fn create_unit_test_if_need(path: &Path, init_file: &InitFile) {
    let parent_path = PathBuf::from(&init_file.parent_path);
    let sources_head = parent_path.join(&init_file.folder_with_files_project);

    let mut test_path = parent_path.join(&init_file.folder_tests_name);
    let mut unresolved_path = parent_path.join(&init_file.folder_unresolved_tests);

    match path.parent().unwrap().strip_prefix(&sources_head) {
        Ok(stripped_path) => {
            test_path.push(stripped_path);
            unresolved_path.push(stripped_path);
        }
        Err(err) => {
            println!("Strip prefix error: {}", err);
            return;
        }
    }

    println!("Finale test_path: {:#?}", test_path);
    println!("Finale unresolver_path: {:#?}", unresolved_path);

    let unit_test_name =
        path.file_stem().unwrap().to_str().unwrap().to_string() + &String::from("Tests.swift");
    let unit_test_pathbuf = test_path.join(&unit_test_name);

    let unit_test_path = unit_test_pathbuf.as_path();

    println!("unit_test_path: {:#?}", unit_test_path);

    if unit_test_path.exists() {
        return;
    } else {
        let class_name = Path::new(&unit_test_name)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let now = Local::now();
        let formatted_date = now.format("%d/%m/%Y").to_string();

        let unit_swift = UnitSwift {
            project_name: init_file.tested_project.clone(),
            class_name: class_name,
            created_date: formatted_date,
        };

        let template_path = Path::new("./ExampleSwiftUnitTest.swift");

        match _create_unit_test(template_path, unit_swift) {
            Ok(result) => {
                println!("Result: {:#?}", result);
                panic!("Something went wrong!");
            }
            Err(err) => {
                println!("_create_unit_test err: {:#?}", err);
                panic!("Something went wrong!");
            }
        };
    }

    println!("is have unit_test: {}", unit_test_path.exists());
    println!("is have swift file in project: {}", path.exists());
}

fn _create_unit_test(
    unit_test_path: &Path,
    unit_swift: UnitSwift,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();
    let template_name = &unit_swift.project_name;

    handlebars.register_template_file(template_name, unit_test_path)?;

    let data = json!({
        "class_name": &unit_swift.class_name,
        "created_date": &unit_swift.created_date,
        "project_name": &unit_swift.project_name,
    });

    let result = handlebars.render(template_name, &data)?;

    Ok(result)
}

fn is_valid_name(path: &PathBuf, init_file: &InitFile) -> bool {
    if path.is_dir() {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        return !init_file.folder_file_exceptions.contains(&file_name);
    } else if path.is_file() {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let exception_name: bool = !init_file.folder_file_exceptions.contains(&file_name);
        let file_extension: String;

        match path.extension() {
            Some(value) => file_extension = value.to_str().unwrap_or("Empty").to_string(),
            None => file_extension = String::from("Empty"),
        }

        let exception_type: bool = init_file.file_extension.contains(&file_extension);
        return exception_name && exception_type;
    }

    return false;
}

// Функция для проверки, является ли файл скрытым
fn is_hidden(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

struct UnitSwift {
    project_name: String,
    class_name: String,
    created_date: String,
}
