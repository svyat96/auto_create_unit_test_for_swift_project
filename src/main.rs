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

// Основная функция программы
fn main() {
    // Путь к файлу конфигурации
    let path_buf = PathBuf::from("./InitFile.json");

    // Читаем конфигурацию из файла
    let config = read_config(&path_buf).unwrap();

    // Получаем пути к папкам с исходниками, тестами и неразрешенными тестами
    let folder_paths = FolderPaths::from(&config);

    // Рекурсивно обходим папку с исходниками и создаем unit-тесты
    match traverse_directory(&folder_paths.sources, &config) {
        Ok(_) => println!("{:#?}", config),
        Err(err) => println!("{:#?}", err),
    }
}

// Функция для рекурсивного обхода папки
fn traverse_directory(path: &PathBuf, init_file: &InitFile) -> Result<(), std::io::Error> {
    // Получаем список записей в папке
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        // Проверяем, соответствует ли имя файла/папки требованиям
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
            // Создаем unit-тест для файла, если он еще не существует
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

// Функция для получения пути к unit-тесту и его имени
fn get_unit_test_path(
    path: &Path,
    config: &InitFile,
) -> Result<(PathBuf, String), StripPrefixError> {
    let folder_paths = FolderPaths::from(config);
    let sources_head = &folder_paths.sources; // Путь к папке с исходниками

    // Получаем относительный путь файла исходника
    let stripped_path = path.parent().unwrap().strip_prefix(&sources_head)?;

    // Формируем путь к файлу unit-теста
    let mut test_path = folder_paths.tests.clone();
    test_path.push(stripped_path);

    // Формируем имя файла unit-теста
    let unit_test_name = path.file_stem().unwrap().to_str().unwrap().to_string() + "Tests.swift";
    let unit_test_path = test_path.join(&unit_test_name);

    Ok((unit_test_path, unit_test_name))
}

// Функция для создания unit-теста
fn create_unit_test(
    unit_test_path: &Path,
    config: &InitFile,
    unit_test_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Извлекаем имя класса из имени файла unit-теста
    let class_name = Path::new(unit_test_name)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Получаем текущую дату и форматируем ее
    let now = Local::now();
    let formatted_date = now.format("%d/%m/%Y").to_string();

    // Создаем структуру с данными для шаблона unit-теста
    let unit_swift = UnitSwift {
        project_name: config.tested_project.clone(),
        class_name,
        created_date: formatted_date,
    };

    // Путь к шаблону unit-теста
    let template_path = Path::new("./ExampleSwiftUnitTest.swift");

    // Рендерим шаблон unit-теста
    let result = _render_unit_test_template(template_path, unit_swift)?;

    // Создаем все необходимые директории
    fs::create_dir_all(unit_test_path.parent().unwrap())?;

    // Создаем файл unit-теста и записываем в него сгенерированный код
    let mut file = File::create(unit_test_path)?;
    file.write_all(result.as_bytes())?;
    Ok(())
}

/// Создает unit-тест для указанного файла Swift, если он еще не существует.
fn create_unit_test_if_need(path: &Path, config: &InitFile) -> Result<(), std::io::Error> {
    // Переменные для хранения пути к unit-тесту и его имени
    let unit_test_path: PathBuf;
    let unit_test_name: String;

    // Получаем путь к unit-тесту
    match get_unit_test_path(path, config) {
        Ok(value) => (unit_test_path, unit_test_name) = value,
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                err.to_string(),
            ))
        }
    }

    // Проверяем, существует ли unit-тест
    if !unit_test_path.exists() {
        // Создаем unit-тест
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

// Функция для рендеринга шаблона unit-теста
fn _render_unit_test_template(
    unit_test_path: &Path,
    unit_swift: UnitSwift,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();
    let template_name = &unit_swift.project_name;
    handlebars.register_template_file(template_name, unit_test_path)?;

    // Создаем JSON-объект с данными для шаблона
    let data = unit_swift.get_json();

    // Рендерим шаблон с данными
    let result = handlebars.render(template_name, &data)?;
    Ok(result)
}

// Функция для проверки, соответствует ли имя файла/папки требованиям
fn is_valid_name(path: &PathBuf, config: &InitFile) -> bool {
    let file_name = match path.file_name() {
        Some(name) => name.to_str().unwrap().to_string(),
        None => return false,
    };

    if path.is_dir() {
        // Проверяем, не входит ли имя папки в список исключений
        !config.folder_file_exceptions.contains(&file_name)
    } else if path.is_file() {
        // Проверяем расширение файла и не входит ли имя файла в список исключений
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

// Функция для проверки, является ли файл скрытым
fn is_hidden(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}