use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde_json::json;

use crate::utils::file_utils::{open_json, save_json, JsonStructure};
use serde::{Serialize, Deserialize};
use super::error::{ExpenseError, ExpenseErrorKind};

#[derive(Serialize, Deserialize)]
pub struct Expense {
    pub id: String,
    pub description: String,
    pub amount: f64,
    pub created_at: DateTime<Utc>,
}

impl Expense {
    pub fn build(description: &str, amount: f64) -> Result<(), ExpenseError> {
        if description == "".to_string() {
            return Err(ExpenseError::new(
                ExpenseErrorKind::EmptyDescription,
                "La descripcion no puede estar vacia",
                "Expense::build",
            ));
        }

        if amount <= 0.0 {
            return Err(ExpenseError::new(
                ExpenseErrorKind::InvalidAmountType,
                "El monto debe ser mayor a 0",
                "Expense::build",
            ));
        }

        let now = SystemTime::now();
        let datetime: DateTime<Utc> = DateTime::from(now);

        let mut id_json = match open_json("./DB/ids.json") {
            Ok(JsonStructure::Object(map)) => map,
            Ok(JsonStructure::Array(vec)) => {
                return Err(ExpenseError::new(
                    ExpenseErrorKind::CreateError, // Asignar un tipo de error específico en `ExpenseError`
                    &format!("Error en archivo JSON: es un array {:?}", vec), // Usar el mensaje de error original
                    "Expense::build",
                ));
            }
            Err(err) => {
                // Mapeo del error `JsonFileError` al `ExpenseError`
                return Err(ExpenseError::new(
                    ExpenseErrorKind::CreateError, // Asignar un tipo de error específico en `ExpenseError`
                    &format!("Error en archivo JSON: {}", err), // Usar el mensaje de error original
                    "Expense::build",
                ));
            }
        };

        let id = match id_json.get("id").and_then(|id| id.as_i64()) {
            Some(id_data) => id_data,
            None => {
                return Err(ExpenseError::new(
                    ExpenseErrorKind::CreateError,
                    "La clave id no fue encontrada en el archivo json",
                    "Expense:build",
                ));
            }
        };

        let new_id = id + 1;
        id_json.insert("id".to_string(), json!(new_id));

        // Aquí está el cambio principal: envolvemos id_json en JsonStructure::Object
        let json_structure = JsonStructure::Object(id_json);

        //se guarda el nuevo id
        match save_json("./DB/ids.json", json_structure) {
            Ok(_) => (),
            Err(err) => {
                return Err(ExpenseError::new(
                    ExpenseErrorKind::CreateError, // Asignar un tipo de error específico en `ExpenseError`
                    &format!("Error en archivo JSON: {}", err), // Usar el mensaje de error original
                    "Expense::build",
                ));
            }
        }

        //se crea la estructura
        let expense = Expense {
            id: id.to_string(),
            description: description.to_string(),
            amount,
            created_at: datetime
        };

        //se obtiene la base de datos
        let mut expenses_json = match open_json("./DB/expenses.json") {
            Ok(JsonStructure::Array(vec)) => vec,
            Ok(JsonStructure::Object(map)) => {
                return Err(ExpenseError::new(
                    ExpenseErrorKind::CreateError, // Asignar un tipo de error específico en `ExpenseError`
                    &format!("Error en archivo JSON: es un array {:?}", map), // Usar el mensaje de error original
                    "Expense::build",
                ));
            }
            Err(err) => {
                return Err(ExpenseError::new(
                    ExpenseErrorKind::CreateError, // Asignar un tipo de error específico en `ExpenseError`
                    &format!("Error en archivo JSON: {}", err), // Usar el mensaje de error original
                    "Expense::build",
                ));
            }
        };

        // se agrega el elemento al array
        expenses_json.push(json!(expense));

        let json_structure = JsonStructure::Array(expenses_json);

        //se guarda el nuevo archivo .json
        match save_json("./DB/expenses.json", json_structure) {
            Ok(_) => (),
            Err(err) => {
                return Err(ExpenseError::new(
                    ExpenseErrorKind::CreateError, // Asignar un tipo de error específico en `ExpenseError`
                    &format!("Error en archivo JSON: {}", err), // Usar el mensaje de error original
                    "Expense::build",
                ));
            }
        }

        Ok(())
    }
}
