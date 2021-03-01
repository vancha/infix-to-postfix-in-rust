use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum TokenType {
    Number(i32),
    Operator(String),
    LeftParenthesis,
    RightParenthesis,
}
struct Token {
    tokentype: TokenType,
}

impl Token {
    fn new(token_type: &str) -> Self {
        let token = token_type.to_string().parse::<i32>();
        match token {
            Ok(value) => Token {
                tokentype: TokenType::Number(value),
            },
            _ => match token_type {
                "(" => Token {
                    tokentype: TokenType::LeftParenthesis,
                },
                ")" => Token {
                    tokentype: TokenType::RightParenthesis,
                },
                "^" => Token {
                    tokentype: TokenType::Operator("^".to_string()),
                },
                "*" => Token {
                    tokentype: TokenType::Operator("*".to_string()),
                },
                "/" => Token {
                    tokentype: TokenType::Operator("/".to_string()),
                },
                "+" => Token {
                    tokentype: TokenType::Operator("+".to_string()),
                },
                "-" => Token {
                    tokentype: TokenType::Operator("-".to_string()),
                },
                _ => panic!("Don't use invalid values"),
            },
        }
    }

    fn is_operator(&self) -> bool {
        match self.tokentype {
            TokenType::Operator(_) => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self.tokentype {
            TokenType::Number(_) => true,
            _ => false,
        }
    }

    fn is_left_parenthesis(&self) -> bool {
        match self.tokentype {
            TokenType::LeftParenthesis => true,
            _ => false,
        }
    }

    fn is_right_parenthesis(&self) -> bool {
        match self.tokentype {
            TokenType::RightParenthesis => true,
            _ => false,
        }
    }

    /* this is horrible, change this */
    fn has_greater_precedence_than(&self, other: &Token) -> bool {
        let mut precedence = HashMap::new();
        precedence.insert("^".to_string(), 4);
        precedence.insert("*".to_string(), 3);
        precedence.insert("/".to_string(), 3);
        precedence.insert("+".to_string(), 2);
        precedence.insert("-".to_string(), 2);

        let self_token_type = match &self.tokentype {
            TokenType::Operator(val) => val.clone(),
            TokenType::LeftParenthesis => "(".to_string().clone(),
            _ => {
                println!("{:?}", self.tokentype);
                panic!("TokenType should be self operator")
            }
        };

        let other_token_type = match &other.tokentype {
            TokenType::Operator(val) => val,
            _ => panic!("TokenType should be other operator"),
        };

        return precedence.get::<str>(&self_token_type) >= precedence.get::<str>(&other_token_type)
            && &self_token_type != other_token_type;
    }

    fn operator_value(&self) -> String {
        match &self.tokentype {
            TokenType::Operator(val) => val.clone(),
            _ => panic!("Should be called only on operator"),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tokentype)
    }
}

fn str_to_token(infix: Vec<&str>) -> Vec<Token> {
    let mut token_list: Vec<Token> = vec![];

    for item in infix {
        let token = Token::new(item);
        token_list.push(token);
    }

    token_list
}
/// Turns a vec of &str's in infix notation, to a vec of String's in postfix notation
///
/// Does not perform the actual evaluation expression, but can be used to change an expression from
/// infix notation, to postfix for easier evaluation.
/// # Example:
/// ``` 
/// use infixtopostfix;
/// assert_eq!(infixtopostfix::infix_to_postfix(vec!["1","+","1"]),vec!["1","1","+"]);
/// ```
pub fn infix_to_postfix(infix_list: Vec<&str>) -> Vec<String> {
    let mut outputqueue: std::collections::VecDeque<Token> = std::collections::VecDeque::new();
    let mut operatorstack: Vec<Token> = Vec::new();
    let token_list: Vec<Token> = str_to_token(infix_list);

    for token in token_list {
        if token.is_number() {
            outputqueue.push_back(token);
        } else if token.is_operator() {
            while !operatorstack.is_empty()
                && operatorstack
                    .last()
                    .unwrap()
                    .has_greater_precedence_than(&token)
                && operatorstack.last().unwrap().operator_value() != "(".to_string()
            {
                outputqueue.push_back(operatorstack.pop().unwrap());
            }
            operatorstack.push(token);
        } else if token.is_left_parenthesis() {
            operatorstack.push(token);
        } else if token.is_right_parenthesis() {
            while !operatorstack.last().unwrap().is_left_parenthesis() {
                outputqueue.push_back(operatorstack.pop().unwrap());
            }
            operatorstack.pop();
        }
    }

    while !operatorstack.is_empty() {
        outputqueue.push_back(operatorstack.pop().unwrap());
    }

    let mut output: Vec<String> = Vec::new();
    for item in outputqueue {
        let item = match item.tokentype {
            TokenType::Operator(val) => val,
            TokenType::Number(num) => num.to_string(),
            _ => panic!("Nothing else should be the value"),
        };
        output.push(item);
    }
    //let output = output;
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_work() {
        assert_eq!(
            infix_to_postfix(vec!["1", "+", "1"]), 
            vec!["1", "1", "+"]);
    }
    #[test]
    fn parenthesis_work() {
        assert_eq!(
            infix_to_postfix(vec!["1", "*", "(", "7", "-", "2", ")"]),
            vec!["1", "7", "2", "-", "*"]
        );
    }
    #[test]
    fn nested_parenthesis_work() {
        assert_eq!(
            infix_to_postfix(vec![
                "1", "*", "(", "7", "-", "2", "+", "(", "1", "+", "1", ")", ")"
            ]),
            vec!["1", "7", "2", "-", "1", "1", "+", "+", "*"]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            infix_to_postfix(vec![
                "3", "+", "4", "*", "2", "/", "(", "1", "-", "5", ")", "^", "2", "^", "3"
            ]),
            vec!["3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+"]
        );
    }
}
