use std::{
    error::Error,
    io::{self, Write},
    process,
};

use expense_tracker::{
    models::{
        config::Config,
        expense::Expense,
    },
    utils::io_utils::leer_data,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        if let Some(source) = e.source() {
            eprintln!("Caused by:{source}")
        }
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    loop {
        print!("expense-tracker ");
        io::stdout().flush()?;
        let input = leer_data()?;
        let config = match Config::build(&input) {
            Ok(config) => config,
            Err(err) => {
                // Imprimir el error si Config::build falla
                eprintln!("{}", err);
                continue; // Continuar con el siguiente ciclo del bucle
            }
        };
        match config.comando.as_str() {
            "add" => {
                match Expense::build(&config.descripcion, config.amount) {
                    Ok(_) => {
                        println!("Expense added successfully")
                    },
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                }
            },
            "list" => {
                match Expense::list() {
                    Ok(_) => {},
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                }
            },
            "summary" => {
                match Expense::summary(&config.descripcion) {
                    Ok(_) => {},
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                }
            },
            "delete" => {
                match Expense::delete(&config.descripcion) {
                    Ok(_) => {
                        println!("Expense deleted successfully")
                    },
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                }
            }
            _ => {
                eprintln!("Error: Comando invalido");
                continue;
            }
        }
    }
}
