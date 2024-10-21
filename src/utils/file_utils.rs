use std::{fs::File, io::BufReader, path::Path};

use serde_json::{json, Map, Value};

use crate::models::error::{JsonFileError, JsonFileErrorKind};

pub enum JsonStructure {
    Object(Map<String, Value>),
    Array(Vec<Value>),
}

pub fn open_json(path: &str) -> Result<JsonStructure, JsonFileError> {
    let path = Path::new(path);
    //se abre el file
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            let kind = match err.kind() {
                std::io::ErrorKind::NotFound => JsonFileErrorKind::FileNotFound,
                std::io::ErrorKind::PermissionDenied => JsonFileErrorKind::PermissionDenied,
                _ => JsonFileErrorKind::ReadError,
            };

            return Err(JsonFileError::new(
                kind,
                &format!("No se pudo abrir el archivo: {}", err),
                path.to_string_lossy().as_ref(),
            ));
        }
    };
    //se parsea el json a hashmap
    let reader = BufReader::new(file);
    let json_data: Value = match serde_json::from_reader(reader) {
        Ok(json_data) => json_data,
        Err(err) => {
            return Err(JsonFileError::new(
                JsonFileErrorKind::ParseError,
                &format!("Error al parsear el JSON: {}", err),
                path.to_string_lossy().as_ref(),
            ))
        }
    };

    //se determina que tipo de estructura JSON se devuelve
    match json_data {
        Value::Object(map) => Ok(JsonStructure::Object(map)),
        Value::Array(vec) => Ok(JsonStructure::Array(vec)),
        _ => Err(JsonFileError::new(
            JsonFileErrorKind::ParseError,
            "El JSON no es ni un objeto ni un array",
            path.to_string_lossy().as_ref(),
        )),
    }
}

pub fn save_json(path: &str, json_data: JsonStructure) -> Result<(), JsonFileError> {
    let path = Path::new(&path);
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(err) => {
            let kind = match err.kind() {
                std::io::ErrorKind::NotFound => JsonFileErrorKind::FileNotFound,
                std::io::ErrorKind::PermissionDenied => JsonFileErrorKind::PermissionDenied,
                _ => JsonFileErrorKind::WriteError, // Podrías usar un tipo de error general para escritura
            };

            return Err(JsonFileError::new(
                kind,
                &format!("No se pudo modificar el archivo: {}", err),
                path.to_string_lossy().as_ref(),
            ));
        }
    };

    let json_value = match json_data {
        JsonStructure::Object(map) => json!(map),
        JsonStructure::Array(vec) => json!(vec),
    };


    match serde_json::to_writer(&mut file, &json_value) {
        Ok(_) => (),
        Err(err) => {
            return Err(JsonFileError::new(
                JsonFileErrorKind::WriteError,
                &format!("Error al escribir el archivo: {}", err),
                path.to_string_lossy().as_ref(),
            ))
        }
    }
    Ok(())
}
