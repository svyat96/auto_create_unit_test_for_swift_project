use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json;

// Функция для создания файла инициализации
pub fn create_init_file<T>(
    path: &PathBuf,
    model: T,
) -> std::io::Result<T>
where
    T: ?Sized + Serialize + Default,
{
    // Создание файла (с перезаписью, если существует)
    let mut file = File::create(path)?;

    // Сериализация структуры в JSON
    let json_data = serde_json::to_string(&model)?;
    
    // Запись JSON-данных в файл
    file.write_all(json_data.as_bytes())?;

    return Ok(model)
}

// Функция для чтения файла инициализации
pub fn read_init_file<T, F>(
    path: &PathBuf,
    create_file: F,
) -> std::io::Result<T>
where
    T: serde::de::DeserializeOwned + Default,
    F: Fn(&PathBuf, T) -> std::io::Result<T>,
{
    // Открытие файла
    let file = File::open(path)?;

    // Создание буферизованного ридера
    let reader = BufReader::new(file);

    // Десериализация JSON-данных в структуру InitFile
    match serde_json::from_reader(reader) {
        Ok(init_file) => return Ok(init_file),
        Err(_) => return create_file(path, T::default()),
    }
}

// Структура для хранения токена Telegram-бота
#[derive(Serialize, Deserialize, Debug)]
pub struct InitFile {
    pub tested_project: String,
    pub parent_path: String,
    pub folder_tests_name: String,
    pub folder_unresolved_tests: String,
    pub folder_with_files_project: String,
    pub folder_file_exceptions: Vec<String>,
    pub file_extension: Vec<String>,
}

impl Default for InitFile {
    fn default() -> Self {
        return InitFile{
            tested_project: String::from("Проект к которому подключаем тесты"),
            parent_path: String::from("Путь к которому добавляются пути для тестовых папок"),
            folder_tests_name: String::from("Куда сохранять тесты"),
            folder_unresolved_tests: String::from("Папка с тестами без ассоциации с рабочими файлами"),
            folder_with_files_project: String::from("Путь до файлов проекта"),
            folder_file_exceptions: vec![ String::from("Названия папок/файлов которые нужно пропускать")],
            file_extension: vec![String::from(".swift")]
        }
    }
}

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

#[cfg(test)]

mod tests {
    use std::io::Error;

    use super::*;

    #[test]
    fn test_create_init_file() {
        // Создаём временный файл для теста
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("init.json");
    
        // Структура с тестовыми данными
        let init_data: InitFile = create_init_file(&file_path, InitFile::default()).unwrap();
    
        // Проверяем, что файл создан
        assert!(file_path.exists());
    
        // Проверяем содержимое файла
        let file_content = std::fs::read_to_string(file_path).unwrap();
        let deserialized_data: InitFile = serde_json::from_str(&file_content).unwrap();
        assert_eq!(deserialized_data, init_data);
    }
    
    #[test]
    fn test_read_init_file() {
        // Создаём временный файл с тестовыми данными
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("init.json");
        // Структура с тестовыми данными
        let init_data: InitFile = create_init_file(&file_path, InitFile::default()).unwrap();
    
        // Вызываем функцию чтения файла
        let read_data: InitFile = read_init_file(&file_path, |_, _| Err(get_error())).unwrap();
    
        // Проверяем, что данные прочитаны корректно
        assert_eq!(read_data, init_data);
    }

    fn get_error() -> std::io::Error {
        return Error::new(std::io::ErrorKind::NotFound, "Файл не найден!")
    }
    
    #[test]
    fn test_read_init_file_error() {
        // Пытаемся прочитать несуществующий файл
        let result: Result<InitFile, _> = read_init_file(&PathBuf::from("nonexistent_file.json"), |_, _| Err(get_error()));
    
        // Проверяем, что возникла ошибка
        assert!(result.is_err());
    }
    
    #[test]
    fn test_init_file_default() {
        // Создаём структуру с помощью default метода
        let init_data = InitFile::default();
    
        // Проверяем значения полей структуры
        assert_eq!(init_data.tested_project, "Проект к которому подключаем тесты");
        assert_eq!(init_data.parent_path, "Путь к которому добавляются пути для тестовых папок");
        assert_eq!(init_data.folder_tests_name, "Куда сохранять тесты");
        assert_eq!(init_data.folder_unresolved_tests, "Папка с тестами без ассоциации с рабочими файлами");
        assert_eq!(init_data.folder_with_files_project, "Путь до файлов проекта");
        assert_eq!(init_data.folder_file_exceptions[0], "Названия папок/файлов которые нужно пропускать");
        assert_eq!(init_data.file_extension[0], ".swift");
    }
}