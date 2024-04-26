// Импортируем необходимые модули
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

// Функция для чтения конфигурации из файла
pub fn read_config(path: &Path) -> Result<InitFile, std::io::Error> {
    if !path.exists() {
        // Создаем файл конфигурации с дефолтными значениями, если он не существует
        create_init_file(path)?;
    }

    // Читаем содержимое файла конфигурации
    let config_str = fs::read_to_string(path)?;

    // Десериализуем содержимое файла в структуру InitFile
    let config: InitFile = serde_json::from_str(&config_str)?;

    Ok(config)
}

// Функция для создания файла InitFile.json с дефолтными значениями (пример)
fn create_init_file(path: &Path) -> std::io::Result<()> {
    // Создаем структуру InitFile с дефолтными значениями
    let config_str = serde_json::to_string_pretty(&InitFile::default())?;

    // Записываем структуру в файл в формате JSON
    fs::write(path, config_str)?;

    Ok(())
}

// Структура для хранения конфигурации
#[derive(Serialize, Deserialize, Debug)]
pub struct InitFile {
    pub tested_project: String,              // Название проекта, к которому подключаем тесты
    pub parent_path: String,               // Путь, к которому добавляются пути для тестовых папок
    pub folder_tests_name: String,          // Название папки, куда сохранять тесты
    pub folder_unresolved_tests: String,    // Название папки с тестами без ассоциации с рабочими файлами
    pub folder_with_files_project: String,  // Путь до файлов проекта
    pub folder_file_exceptions: Vec<String>, // Названия папок/файлов, которые нужно пропускать
    pub file_extension: Vec<String>,        // Расширения файлов, которые нужно обрабатывать
}

// Реализация метода Default для структуры InitFile, который возвращает структуру с дефолтными значениями
impl Default for InitFile {
    fn default() -> Self {
        return InitFile {
            tested_project: String::from("Проект к которому подключаем тесты"),
            parent_path: String::from("Путь к которому добавляются пути для тестовых папок"),
            folder_tests_name: String::from("Куда сохранять тесты"),
            folder_unresolved_tests: String::from("Папка с тестами без ассоциации с рабочими файлами"),
            folder_with_files_project: String::from("Путь до файлов проекта"),
            folder_file_exceptions: vec![String::from("Названия папок/файлов которые нужно пропускать")],
            file_extension: vec![String::from(".swift")],
        };
    }
}

// Реализация метода PartialEq для структуры InitFile, который сравнивает две структуры на равенство
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
// Структура для хранения данных о unit-тесте
pub struct UnitSwift {
    pub project_name: String, // Имя проекта
    pub class_name: String,   // Имя класса unit-теста
    pub created_date: String, // Дата создания unit-теста
}

// Реализация методов для структуры UnitSwift
impl UnitSwift {
    // Функция для преобразования данных UnitSwift в формат JSON
    pub fn get_json(&self) -> serde_json::Value {
        return json!({
            "class_name": &self.class_name,
            "created_date": &self.created_date,
            "project_name": &self.project_name,
        });
    }
}

// Структура для хранения путей к папкам
pub struct FolderPaths {
    pub parent: PathBuf,      // Родительский путь
    pub sources: PathBuf,    // Путь к папке с исходниками
    pub tests: PathBuf,       // Путь к папке с тестами
    pub unresolved: PathBuf, // Путь к папке с неразрешенными тестами
}

// Реализация trait From для преобразования InitFile в FolderPaths
impl From<InitFile> for FolderPaths {
    // Преобразуем структуру InitFile в FolderPaths
    fn from(value: InitFile) -> Self {
        return FolderPaths::from(&value); // Вызываем другой метод From для ссылки на InitFile
    }
}

// Реализация trait From для преобразования ссылки на InitFile в FolderPaths
impl From<&InitFile> for FolderPaths {
    fn from(value: &InitFile) -> Self {
        let parent_path = PathBuf::from(&value.parent_path); // Создаем PathBuf из родительского пути

        FolderPaths {
            parent: parent_path.clone(),       // Сохраняем родительский путь
            sources: parent_path.join(&value.folder_with_files_project), // Формируем путь к папке с исходниками
            tests: parent_path.join(&value.folder_tests_name),           // Формируем путь к папке с тестами
            unresolved: parent_path.join(&value.folder_unresolved_tests), // Формируем путь к папке с неразрешенными тестами
        }
    }
}