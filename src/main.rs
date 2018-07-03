extern crate clap;
extern crate colored;

// use std::fs;
use std::path::Path;
use std::process;
// use std::fs::File;
// use std::io::prelude::*;

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



// fn parse(path: String) -> Vec<Node> 
// {
//     let mut contents = String::new();
//     let mut f = File::open(path).expect("Could not read file !");
//     f.read_to_string(&mut contents).expect("something went wrong reading the file");

//     let mut chars = contents.chars().rev().collect::<String>();
//     let mut buffer:String = String::from("");
//     let mut is_parsing_property = false;
//     let mut nodes:Vec<Node> = Vec::new();

//     let mut parent_selector: Vec<String> = Vec::new();
//     let mut current_selector: String = String::new();
//     let mut current_property: String = String::new();
//     let mut current_properties: Vec<Property> = Vec::new();

//     while let Some(top) = chars.pop() {
//         match top {
//             '{' => { 
//                 current_selector = buffer.clone();
//                 parent_selector.push(buffer.clone());
//                 buffer = String::new();
//             },

//             '}' => {
//                 // If we are parsing a property, (no trailing ;)
//                 if is_parsing_property {
//                    current_properties.push(Property {
//                         name: current_property.clone(), 
//                         value: buffer
//                     });

//                     buffer = String::new();
//                 }
               
//                 // Create the node
//                 // nodes.push(Node {
//                 //     selector: parent_selector.clone().into_iter().fold(String::new(), |previous, current| if prevformat!("{} {}",  previous, current)),
//                 //     properties: current_properties
//                 // });

//                 parent_selector.pop();
//                 current_properties = Vec::new();
//             }, 

//             ':' => {
//                 current_property = buffer;
//                 is_parsing_property = true;
//                 buffer = String::new();
//             }

//             ';' => {
//                 if is_parsing_property {
//                     current_properties.push(Property {
//                         name: current_property.clone(), 
//                         value: buffer
//                     });

//                     buffer = String::new();
//                 }

//                 is_parsing_property = false;
//             }

//             'a' ... 'z' => buffer.push(top),

//             _   => () //println!("Is not a matched pattern : {}", top)
//         }
//     }

//     return nodes;
// }

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

fn tokenize(content: String) -> Vec<Token>
{
    let mut result:Vec<Token> = Vec::new();
    let mut chars = content.chars().rev().collect::<String>();
    let mut buffer: String = String::new();

    while let Some(value) = chars.pop() {

        match value {
            ' ' => {},

            '{' => { 
                result.push(Token(TokenType::Selector, Some(buffer.clone())));
                result.push(Token(TokenType::OpenBrace, None));
                buffer = String::new();
            },

            '}' => {
                result.push(Token(TokenType::CloseBrace, None));
            },

            ':' => {
                result.push(Token(TokenType::VariableName, Some(buffer.clone())));
                buffer = String::new();
            },

            ';' => {
                result.push(Token(TokenType::VariableValue, Some(buffer.clone())));
                buffer = String::new();
            },

            _ => {
                buffer.push(value);
            }
        }     
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



struct Token(TokenType, Option<String>);

#[derive(PartialEq, Debug)]
enum TokenType {
    Selector,
    VariableName,
    VariableValue, 
    OpenBrace, 
    CloseBrace
}



#[test]
fn test_basic_parsing() {  
    let result = tokenize("a { color: blue; }".to_string());
    assert_eq!(result.len(), 5);
    assert_eq!(TokenType::Selector, result.get(0).unwrap().0);
    assert_eq!("a".to_string(), result.get(0).unwrap().1.unwrap());
}
