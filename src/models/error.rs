use std::{error::Error, fmt};

// Input error
#[derive(Debug)]
pub struct InputError {
    message: String,
    cause: Option<Box<dyn Error>>,
}

impl InputError {
    pub fn new(msg: &str) -> InputError {
        InputError {
            message: msg.to_string(),
            cause: None,
        }
    }

    pub fn with_cause(msg: &str, cause: Box<dyn Error>) -> InputError {
        InputError {
            message: msg.to_string(),
            cause: Some(cause),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref())
    }
}

//Errores creando la estructura del comando
// Config error
#[derive(Debug)]
pub enum ConfigErrorKind {
    EmptyInput,
    InvalidCommand,
    MissingArguments,
    InvalidTypeData,
}

#[derive(Debug)]
pub struct ConfigError {
    kind: ConfigErrorKind,
    message: String,
}

impl ConfigError {
    pub fn new(kind: ConfigErrorKind, message: &str) -> Self {
        ConfigError {
            kind,
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> &ConfigErrorKind {
        &self.kind
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error de configuración: {}", self.message)
    }
}

impl Error for ConfigError {}

//Errores relacionados con el item gasto
#[derive(Debug)]

pub enum ExpenseErrorKind {
    CreateError,
    EmptyDescription,
    InvalidAmountType,
    ReadError
}
#[derive(Debug)]
pub struct ExpenseError {
    kind: ExpenseErrorKind,
    message: String,
    location: String,
}

impl ExpenseError {
    pub fn new(kind: ExpenseErrorKind, message: &str, location: &str) -> Self {
        ExpenseError {
            kind,
            message: message.to_string(),
            location: location.to_string(),
        }
    }

    pub fn kind(&self) -> &ExpenseErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &String {
        &self.message
    }
    pub fn location(&self) -> &String {
        &self.location
    }
}

impl fmt::Display for ExpenseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error en '{}': {}", self.location, self.message)
    }
}

impl Error for ExpenseError {}

//Errores abriendo archivos json
#[derive(Debug)]

pub enum JsonFileErrorKind {
    FileNotFound,     // El archivo no existe o no se encuentra
    PermissionDenied, // No hay permisos para acceder al archivo
    ReadError,        // Error al leer el archivo
    WriteError,       // Error al escribir en el archivo
    ParseError,       // Error al analizar (parsear) el contenido JSON
    InvalidFormat,    // El formato JSON es inválido
    ModifyError,      // Error al intentar modificar el archivo
    DeleteError,      // Error al intentar eliminar el archivo
    CreateError,      // Error al intentar crear el archivo
}
#[derive(Debug)]

pub struct JsonFileError {
    kind: JsonFileErrorKind,
    message: String,
    location: String,
}

impl JsonFileError {
    pub fn new(kind: JsonFileErrorKind, message: &str, location: &str) -> Self {
        JsonFileError {
            kind,
            message: message.to_string(),
            location: location.to_string(),
        }
    }

    // Métodos para acceder a los campos de `JsonFileError`
    pub fn kind(&self) -> &JsonFileErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn location(&self) -> &String {
        &self.location
    }
}

// Implementar el trait `fmt::Display` para mostrar mensajes de error legibles
impl fmt::Display for JsonFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error en '{}': {}", self.location, self.message)
    }
}


// Implementar el trait `Error` para permitir el manejo de errores
impl Error for JsonFileError {}
