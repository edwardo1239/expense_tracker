use std::io;

use crate::models::error::InputError;


pub fn leer_data() -> Result<String, InputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(InputError::with_cause(
            "Failed to read from stdin", Box::new(e))
        ),
    }
}