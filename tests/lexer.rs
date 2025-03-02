#[cfg(test)]
mod tests {
    use lattice::lexer::{tokenize, TokenType};

    #[test]
    fn test_tokenize_numbers() {
        let tokens = tokenize("123 456".to_string());
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "123");
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "456");
        assert_eq!(tokens[1].token_type, TokenType::Number);
    }

    #[test]
    fn test_tokenize_identifiers() {
        let tokens = tokenize("hello world".to_string());
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "hello");
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].value, "world");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
    }

    #[test]
    fn test_tokenize_operators() {
        let tokens = tokenize("+ - * /".to_string());
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "+");
        assert_eq!(tokens[0].token_type, TokenType::BinaryOperator);
        assert_eq!(tokens[1].value, "-");
        assert_eq!(tokens[1].token_type, TokenType::BinaryOperator);
        assert_eq!(tokens[2].value, "*");
        assert_eq!(tokens[2].token_type, TokenType::BinaryOperator);
        assert_eq!(tokens[3].value, "/");
        assert_eq!(tokens[3].token_type, TokenType::BinaryOperator);
    }

    #[test]
    fn test_tokenize_parentheses() {
        let tokens = tokenize("( )".to_string());
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "(");
        assert_eq!(tokens[0].token_type, TokenType::OpenParen);
        assert_eq!(tokens[1].value, ")");
        assert_eq!(tokens[1].token_type, TokenType::CloseParen);
    }

    #[test]
    fn test_tokenize_equals() {
        let tokens = tokenize("=".to_string());
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "=");
        assert_eq!(tokens[0].token_type, TokenType::Equals);
    }

    #[test]
    fn test_tokenize_mixed() {
        let tokens = tokenize("let x = 10 + 5".to_string());
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].value, "let");
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].value, "x");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "=");
        assert_eq!(tokens[2].token_type, TokenType::Equals);
        assert_eq!(tokens[3].value, "10");
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[4].value, "+");
        assert_eq!(tokens[4].token_type, TokenType::BinaryOperator);
        assert_eq!(tokens[5].value, "5");
        assert_eq!(tokens[5].token_type, TokenType::Number);
    }
}
