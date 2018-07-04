use std::collections::HashMap;

use super::tokenizer::{Token, tokenize, TokenType};

struct Node {
    pub selector: String, 
    pub properties: Vec<Property>
}

struct Property {
    pub property: String, 
    pub value: String
}

fn parse(mut tokens: Vec<Token>) -> Vec<Node> {
    tokens.reverse();

    let mut result = Vec::new();
    let mut parent_selectors:Vec<String> = Vec::new();
    
    let mut variables:HashMap<String, String> = HashMap::new();
    let mut current_properties:Vec<Property> = Vec::new();
    let mut current_property_name:String = String::new();

    let mut current_variable_name:Option<String> = None;

    while let Some(token) = tokens.pop() {
        let token_type = token.0;
        let token_value = token.1.clone();
        println!("{:?} - {:?}", token_type, token_value);

        match token_type {

            TokenType::VariableName => {
                current_variable_name = Some(token.1.unwrap());
            },

            TokenType::VariableValue => {
                variables.entry(current_variable_name.unwrap().clone()).or_insert(token.1.unwrap());
                current_variable_name = None;
            }, 

            TokenType::PropertyVariableValue => {
                let value = variables.get(token.1.unwrap().clone());

                current_properties.push(Property { 
                    property: current_property_name.clone(),
                    value: token.1.unwrap().clone()
                });
                current_property_name = String::new();

            }, 

            TokenType::Selector => {
                parent_selectors.push(token.1.unwrap().clone());
            },

            TokenType::OpenBrace => {},

            TokenType::PropertyName => {
                current_property_name = token.1.unwrap().clone();
            }, 

            TokenType::PropertyValue => {
                current_properties.push(Property { 
                    property: current_property_name.clone(),
                    value: token.1.unwrap().clone()
                });
                current_property_name = String::new();
            },

            TokenType::CloseBrace => {
                let full_selector = parent_selectors.clone().iter().fold(String::new(), |previous, current| 
                    if previous != "" {
                        return format!("{} {}", previous, current);
                    } else {
                        return current.clone();
                    }
                );

                result.push(Node {
                    selector: full_selector.clone(), 
                    properties: current_properties
                });

                parent_selectors.pop();
                current_properties = Vec::new();
            },

            _ => {}
        }
    }

    return result;
}

#[test]
fn test_basic_parser() {
    let tokens = tokenize("a { color: blue; }".to_string());
    let result = parse(tokens);

    assert_eq!(1, result.len());
    assert_eq!("a", result.get(0).unwrap().selector);
    assert_eq!(1, result.get(0).unwrap().properties.len());
    assert_eq!("color", result.get(0).unwrap().properties.get(0).unwrap().property);
    assert_eq!("blue", result.get(0).unwrap().properties.get(0).unwrap().value);
}

#[test]
fn test_variable_parser() {
    let tokens = tokenize("$color: blue; a { color: $color; }".to_string());
    let result = parse(tokens);

    assert_eq!(1, result.len());
    assert_eq!(1, result.get(0).unwrap().properties.len());
    assert_eq!("color", result.get(0).unwrap().properties.get(0).unwrap().property);
    assert_eq!("blue", result.get(0).unwrap().properties.get(0).unwrap().value);

}
