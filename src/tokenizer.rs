pub struct Token(pub TokenType, pub Option<String>);

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Selector,
    VariableName,
    VariableValue, 
    OpenBrace, 
    CloseBrace, 
    PropertyName, 
    PropertyValue
}

pub fn tokenize(content: String) -> Vec<Token>
{
    let mut result:Vec<Token> = Vec::new();
    let mut chars = content.chars().rev().collect::<String>();
    let mut buffer: String = String::new();
    
    let mut is_parsing_variable = false;

    while let Some(value) = chars.pop() {

        println!("Parsing variable : {} {}", value, is_parsing_variable);

        match value {
            ' ' => {},

            '$' => {
                is_parsing_variable = true;
            },

            '{' => { 
                result.push(Token(TokenType::Selector, Some(buffer.clone())));
                result.push(Token(TokenType::OpenBrace, None));
                buffer = String::new();
            },

            '}' => {
                result.push(Token(TokenType::CloseBrace, None));
            },

            ':' => {
                if is_parsing_variable {
                    result.push(Token(TokenType::VariableName, Some(buffer.clone())));
                } else {
                    result.push(Token(TokenType::PropertyName, Some(buffer.clone())));
                }

                buffer = String::new();
            },

            ';' => {
                if is_parsing_variable {
                    result.push(Token(TokenType::VariableValue, Some(buffer.clone())));    
                } else {
                    result.push(Token(TokenType::PropertyValue, Some(buffer.clone())));
                }

                is_parsing_variable = false;
                buffer = String::new();
            },

            _ => {
                buffer.push(value);
            }
        }     
    }

    return result;
}

#[test]
#[ignore]
fn test_basic_parsing() {  
    let result = tokenize("a { color: \nblue; }".to_string());
    assert_eq!(result.len(), 5);

    assert_eq!(TokenType::Selector, result.get(0).unwrap().0);
    assert_eq!(TokenType::OpenBrace, result.get(1).unwrap().0);
    assert_eq!(TokenType::PropertyName, result.get(2).unwrap().0);
    assert_eq!(TokenType::PropertyValue, result.get(3).unwrap().0);
    assert_eq!(TokenType::CloseBrace, result.get(4).unwrap().0);
}

#[test]
#[ignore]
fn test_empty_parsing() {
    let result = tokenize("".to_string());
    assert_eq!(result.len(), 0);
}

#[test]
#[ignore]
fn test_nested_parsing() {
    let result = tokenize("body { a { color: blue; } }".to_string());

    assert_eq!(result.len(), 8);
    assert_eq!(TokenType::Selector, result.get(0).unwrap().0);
    assert_eq!(TokenType::OpenBrace, result.get(1).unwrap().0);
    assert_eq!(TokenType::Selector, result.get(2).unwrap().0);
    assert_eq!(TokenType::OpenBrace, result.get(3).unwrap().0);
    assert_eq!(TokenType::PropertyName, result.get(4).unwrap().0);
    assert_eq!(TokenType::PropertyValue, result.get(5).unwrap().0);
    assert_eq!(TokenType::CloseBrace, result.get(6).unwrap().0);
    assert_eq!(TokenType::CloseBrace, result.get(7).unwrap().0);
}

#[test]
fn test_variable_parsing() {
    let result = tokenize("$name: blue; a { color: $name; }".to_string());


    assert_eq!(result.len(), 7);
    assert_eq!(TokenType::VariableName, result.get(0).unwrap().0);
    assert_eq!(TokenType::VariableValue, result.get(1).unwrap().0);
    assert_eq!(TokenType::Selector, result.get(2).unwrap().0);
    assert_eq!(TokenType::OpenBrace, result.get(3).unwrap().0);
    assert_eq!(TokenType::PropertyName, result.get(4).unwrap().0);
    assert_eq!(TokenType::PropertyValue, result.get(5).unwrap().0);
    assert_eq!(TokenType::CloseBrace, result.get(6).unwrap().0);
}