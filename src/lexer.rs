#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Literal Types
    Number,
    Identifier,

    // Keywords
    Let,

    // Grouping * Operators
    BinaryOperator,
    Equals,
    OpenParen,
    CloseParen,

    // End of File
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

pub fn tokenize(source_code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut source = source_code.chars().collect::<Vec<char>>();

    while source.len() > 0 {
        let c = source[0];
        match c {
            '0'..='9' => {
                // Number
                let mut number = String::new();
                while source.len() > 0 && source[0].is_digit(10) {
                    number.push(source.remove(0));
                }
                tokens.push(Token {
                    value: number,
                    token_type: TokenType::Number,
                });
            }
            'a'..='z' | 'A'..='Z' => {
                // Identifier
                let mut identifier = String::new();
                while source.len() > 0 && source[0].is_alphanumeric() {
                    identifier.push(source.remove(0));
                }
                tokens.push(Token {
                    value: identifier,
                    token_type: TokenType::Identifier,
                });
            }
            '+' | '-' | '*' | '/' => {
                // Binary Operator
                tokens.push(Token {
                    value: c.to_string(),
                    token_type: TokenType::BinaryOperator,
                });
                source.remove(0);
            }
            '=' => {
                // Equals
                tokens.push(Token {
                    value: c.to_string(),
                    token_type: TokenType::Equals,
                });
                source.remove(0);
            }
            '(' => {
                // Open Paren
                tokens.push(Token {
                    value: c.to_string(),
                    token_type: TokenType::OpenParen,
                });
                source.remove(0);
            }
            ')' => {
                // Close Paren
                tokens.push(Token {
                    value: c.to_string(),
                    token_type: TokenType::CloseParen,
                });
                source.remove(0);
            }
            _ => {
                // Ignore
                source.remove(0);
            }
        }
    }

    tokens.push(Token {
        value: "".to_string(),
        token_type: TokenType::EOF,
    });
    return tokens;
}
