extern crate clap;
extern crate colored;

use std::path::Path;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::collections::LinkedList;

use colored::*;
use clap::{Arg, App};


fn main() {  
    let matches = App::new("Rust-sass")
        .version("1.0")
        .author("Michiel")
        .about("Sass compiler in Rust")
        .arg(Arg::with_name("precision")
            .long("precision")
            .help("Np clue what this does")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("t")
            .long("output")
            .help("The style for the outputted CSS : expanded/compressed")
            .takes_value(true))
            // Has to be one of the four values : nested, expanded, compact, compressed
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .takes_value(true)
            .required(true))
        .get_matches();

        let input_file = matches.value_of("INPUT").unwrap();
        if !Path::new(input_file).exists() {
            println!("{} ({}) {}", "Input file".red(), input_file.red(), "cannot be read !".red());
            process::exit(1);
        }

        let mut f = File::open(input_file).expect("Could not read file !");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");

        let result = compile(contents);
        let formatted_result = format(result, String::from(""));
        print!("{}", formatted_result);
}

// The real program
struct Node {
    selector: String, 
    properties: LinkedList<Property>
}

struct Property {
    name: String, 
    value: String
}

fn compile(contents: String) -> String 
{
    if contents == "" {
        return "".to_string();
    }

    


    return contents;
}

fn format(contents: String, style: String) -> String 
{
    // TODO Implement the style output
    return contents;
}
