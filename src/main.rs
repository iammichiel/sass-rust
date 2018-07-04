extern crate clap;
extern crate colored;

// use std::fs;
use std::path::Path;
use std::process;
// use std::fs::File;
// use std::io::prelude::*;

use colored::*;
use clap::{Arg, App};

mod tokenizer;
mod parser;

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

        // let result = parse(input_file.to_string());
        // let formatted_result = format(result, String::from(""));
        // print!("{}", formatted_result);
}


// The real program
struct Node {
    selector: String, 
    properties: Vec<Property>
}

struct Property {
    name: String, 
    value: String
}

fn format(nodes: Vec<Node>, _style: String) -> String 
{
    let mut result = String::new();

    for node in &nodes {
        result.push_str(&format!("{} {{\n", node.selector));
        for property in &node.properties {
            result.push_str(&format!("  {}: {};\n", property.name, property.value));
        }

        result.push_str("}\n");
    }

    return result;
}




// #[test]
// #[ignore]
// fn test_sass_specs() {
//     let paths = fs::read_dir("./sass-specs/spec/basic/").unwrap();

//     for path in paths {
//         let p =  path.unwrap().path();
//         let full_input_path = format!("{}/input.scss", p.display());

//         let input = parse(full_input_path);
//         let input_result = format(input, "".to_string());

//         let mut output = String::new();
//         let full_output_path: String = format!("{}/expected_output.css", p.display());
        
//         let mut f = File::open(full_output_path).expect("could not read value");
//         f.read_to_string(&mut output).expect("something went wrong reading the file");

//         assert_eq!(input_result, output);
//     }
// }





