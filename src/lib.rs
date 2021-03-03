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
            Ok(value) => Self {
                tokentype: TokenType::Number(value),
            },
            _ => match token_type {
                "(" => Self {
                    tokentype: TokenType::LeftParenthesis,
                },
                ")" => Self {
                    tokentype: TokenType::RightParenthesis,
                },
                "^" => Self {
                    tokentype: TokenType::Operator("^".to_string()),
                },
                "*" => Self {
                    tokentype: TokenType::Operator("*".to_string()),
                },
                "/" => Self {
                    tokentype: TokenType::Operator("/".to_string()),
                },
                "+" => Self {
                    tokentype: TokenType::Operator("+".to_string()),
                },
                "-" => Self {
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

    // checks for precedence of operators
    fn has_greater_precedence_than(&self, other: &Token) -> bool {
        let mut precedence = HashMap::new();
        precedence.insert("^".to_string(), 4);
        precedence.insert("*".to_string(), 3);
        precedence.insert("/".to_string(), 3);
        precedence.insert("+".to_string(), 2);
        precedence.insert("-".to_string(), 2);

        let self_token_value = match &self.tokentype {
            TokenType::Operator(val) => val,
            TokenType::LeftParenthesis => "(",
            _ => panic!("Should be called only on self operator-stack"),
        };

        let other_token_value = match &other.tokentype {
            TokenType::Operator(val) => val,
            _ => panic!("Should be called only on operator-stack"),
        };

        return precedence.get::<str>(&self_token_value)
            >= precedence.get::<str>(&other_token_value)
            && &self_token_value != other_token_value;
    }

    // extracts operator-value from the token
    fn operator_value(&self) -> &str {
        match &self.tokentype {
            TokenType::Operator(val) => val,
            _ => panic!("Can only extract from operator"),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tokentype)
    }
}

fn str_to_token(infix: &[&str]) -> Vec<Token> {
    let mut token_list: Vec<Token> = vec![];

    for item in infix {
        let token = Token::new(item);
        token_list.push(token);
    }

    token_list
}

/// Turns `&[&str]` of infix notation, into a postfix notation.
///
/// Does not perform the actual evaluation expression, but can be used to change an expression from
/// infix notation, to postfix for easier evaluation.
///
/// To perform calculation, use crate "`postfix`".
/// # Example:
/// ```
/// use infixtopostfix;
/// assert_eq!(infixtopostfix::infix_to_postfix(&["1","+","1"]),&["1","1","+"]);
/// ```
pub fn infix_to_postfix<'a>(infix_list: &'a [&str]) -> Vec<&'a str> {
    // "postfix" representation of tokens (not to be returned)
    let mut tokenstack: Vec<Token> = Vec::new();
    let mut operatorstack: Vec<Token> = Vec::new();
    let token_list: Vec<Token> = str_to_token(&infix_list);

    // Shunting yard algorithm - START
    // https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    for token in token_list {
        if token.is_number() {
            tokenstack.push(token);
        } else if token.is_operator() {
            while !operatorstack.is_empty()
                && operatorstack
                    .last()
                    .unwrap()
                    .has_greater_precedence_than(&token)
                && operatorstack.last().unwrap().operator_value() != "(".to_string()
            {
                tokenstack.push(operatorstack.pop().unwrap());
            }
            operatorstack.push(token);
        } else if token.is_left_parenthesis() {
            operatorstack.push(token);
        } else if token.is_right_parenthesis() {
            while !operatorstack.last().unwrap().is_left_parenthesis() {
                tokenstack.push(operatorstack.pop().unwrap());
            }
            operatorstack.pop();
        }
    }
    while !operatorstack.is_empty() {
        tokenstack.push(operatorstack.pop().unwrap());
    }
    // Shunting yard algorithm - END

    // list to be returned
    let mut output: Vec<&str> = Vec::new();

    // Adds "references to the elements in the original input"
    // to "the list to be returned"
    for item in tokenstack {
        let mut index = 0;
        match item.tokentype {
            TokenType::Operator(val) => {
                for element in infix_list {
                    if *element == &val {
                        output.push(infix_list[index]);
                        break;
                    }
                    index += 1;
                }
            }
            TokenType::Number(num) => {
                for element in infix_list {
                    if *element == &num.to_string() {
                        output.push(infix_list[index]);
                        break;
                    }
                    index += 1;
                }
            }
            _ => panic!("Only numbers and operators can be the value at this point"),
        };
    }
    // let output = output; // even though return type should be immutable, the output is mutable, so is this statement necessary?
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_work() {
        assert_eq!(infix_to_postfix(&["1", "+", "1"]), &["1", "1", "+"]);
    }
    #[test]
    fn parenthesis_work() {
        assert_eq!(
            infix_to_postfix(&["1", "*", "(", "7", "-", "2", ")"]),
            &["1", "7", "2", "-", "*"]
        );
    }
    #[test]
    fn nested_parenthesis_work() {
        assert_eq!(
            infix_to_postfix(&["1", "*", "(", "7", "-", "2", "+", "(", "1", "+", "1", ")", ")"]),
            &["1", "7", "2", "-", "1", "1", "+", "+", "*"]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            infix_to_postfix(&[
                "3", "+", "4", "*", "2", "/", "(", "1", "-", "5", ")", "^", "2", "^", "3"
            ]),
            &["3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+"]
        );
    }
}
