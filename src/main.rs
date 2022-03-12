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

    let schema =  match serde_dhall::from_str(include_str!("schema.dhall")).parse() {
        Ok(value) => value,
        Err(e) => {
            error!("Error reading Dhall file: {}", e);
            ::std::process::exit(1);
        }
    };

    let config: Value = match serde_dhall::from_file(&filename)
        .type_annotation(&schema)
        .parse() {
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

    let template_name = "resume.tex";

    let mut tera = Tera::default();

    match  tera.add_raw_template(template_name, include_str!("templates/resume.tex")) {
        Err(e) => {
            error!("Error adding template: {}", e);
            ::std::process::exit(1);
        }
        _ => {}
    };

    let rendered = match tera.render(template_name, &context) {
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
