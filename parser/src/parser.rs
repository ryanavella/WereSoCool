lalrpop_mod!(pub socool);
extern crate colored;
extern crate num_rational;
use crate::ast::*;
use crate::error_handling::handle_parse_error;
use crate::imports::{get_filepath_and_import_name, is_import};
use colored::*;
use num_rational::Rational64;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq, Debug)]
pub struct Init {
    pub f: Rational64,
    pub l: Rational64,
    pub g: Rational64,
    pub p: Rational64,
}

pub type ParseTable = HashMap<String, Op>;

#[derive(Clone, PartialEq, Debug)]
pub struct ParsedComposition {
    pub init: Init,
    pub table: ParseTable,
}

pub fn parse_file(filename: &String, parse_table: Option<ParseTable>) -> ParsedComposition {
    let mut table = if parse_table.is_some() {
        parse_table.unwrap()
    } else {
        HashMap::new()
    };

    let f = File::open(filename);
    let mut composition = String::new();
    let mut imports_needed = vec![];

    match f {
        Ok(f) => {
            let file = BufReader::new(&f);
            for line in file.lines() {
                let l = line.unwrap();
                let copy_l = l.trim_left();
                if copy_l.starts_with("--") {
                    composition.push_str("\n");
                } else if is_import(copy_l.to_string()) {
                    imports_needed.push(copy_l.to_owned());
                    composition.push_str("\n");
                } else {
                    composition.push_str("\n");
                    composition.push_str(&l);
                }
            }
        }
        _ => {
            println!(
                "{} {}\n",
                "\n        File not found:".red().bold(),
                filename.red().bold()
            );
            panic!("File not found");
        }
    }

    for import in imports_needed {
        let (filepath, import_name) = get_filepath_and_import_name(import);
        let parsed_composition = parse_file(&filepath.to_string(), Some(table.clone()));

        for (key, val) in parsed_composition.table {
            let mut name = import_name.clone();
            name.push('.');
            name.push_str(&key);
            table.insert(name, val);
        }
    }

    let init = socool::SoCoolParser::new().parse(&mut table , &composition);

    match init.clone() {
        Ok(init) => ParsedComposition { init, table },
        Err(error) => {
            let location = Arc::new(Mutex::new(Vec::new()));
            error.map_location(|l| location.lock().unwrap().push(l));
            handle_parse_error(location, &composition);
            panic!("Unexpected Token")
        }
    }
}