use super::error::{ConfigError, ConfigErrorKind};

#[derive(Debug)]
pub struct Config {
    pub comando: String,
    pub descripcion: String,
    pub amount: f64,
}

impl Config {
    pub fn build(input: &str) -> Result<Config, ConfigError> {
        let mut parts = input.split_whitespace();
        let mut descripcion = String::new();
        let mut description_flag = false;
        let mut amount_flag = false;
        let mut amount: f64 = 0.0;
        let comando = parts
            .next()
            .ok_or(ConfigError::new(
                ConfigErrorKind::EmptyInput,
                "No se proporcionó ningún comando",
            ))?
            .to_string();

        match comando.as_str() {
            "add" => loop {
                let item = parts.next();
                if item.is_none() {
                    break;
                }
                let item = item.unwrap();

                if item == "--description" {
                    description_flag = true;
                    amount_flag = false;
                } else if item == "--amount" {
                    description_flag = false;
                    amount_flag = true;
                } else if description_flag {

                    if !descripcion.is_empty() {
                        descripcion.push(' ');
                    }
                    descripcion.push_str(item);
                } else if amount_flag {
                    amount = item.parse().map_err(|_| {
                        ConfigError::new(
                            ConfigErrorKind::InvalidTypeData,
                            "Ingrese un numero por favor"
                        )
                    })?;
                } 
            },
            _ => {
                return Err(ConfigError::new(
                    ConfigErrorKind::InvalidCommand,
                    "Comando invalido",
                ));
            }
        }

        Ok(Config {
            comando,
            descripcion,
            amount: amount,
        })
    }
}

