use std::env;
use std::fs;
use serde_json::Value;
use tera::Context;
use tera::Tera;
use log::{info, error};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    info!("Reading Dhall file: {}", filename);

    let config: Value = match serde_dhall::from_file(&filename).parse() {
        Ok(value) => value,
        Err(e) => {
            error!("Error reading Dhall file: {}", e);
            ::std::process::exit(1);
        }
    };

    let context = match Context::from_serialize(&config) {
        Ok(context) => context,
        Err(e) => {
            error!("Error generating context: {}", e);
            ::std::process::exit(1);
        }
    };

    let tera = match Tera::new("templates/*.tex") {
        Ok(tera) => tera,
        Err(e) => {
            error!("Error reading templates: {}", e);
            ::std::process::exit(1);
        }
    };

    let rendered = match tera.render("resume.tex", &context) {
        Ok(x) => x,
        Err(e) => {
            error!("Error rendering: {}", e);
            ::std::process::exit(1);
        }
    };

    match fs::write("resume.tex", rendered) {
        Err(e) => {
            error!("Error writing: {}", e);
            ::std::process::exit(1);
        }
        _ => {}
    }
}
