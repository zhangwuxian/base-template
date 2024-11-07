use crate::errors::CommonError;
use std::fs;
use std::path::Path;

pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

pub fn create_fold(fold: &str) -> Result<(), CommonError> {
    if !Path::new(fold).exists() {
        fs::create_dir_all(fold)?
    }
    Ok(())
}

pub fn read_file(file_path: &str) -> Result<String, CommonError> {
    if !Path::new(file_path).exists() {
        return Err(CommonError::CommonError(format!(
            "File {file_path} does not exist"
        )));
    }
    Ok(fs::read_to_string(file_path)?)
}
