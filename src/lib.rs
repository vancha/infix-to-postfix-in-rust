use std::collections::HashMap;
use std::fmt;
/*
 * Consider -
    Stack S
    Char  ch
    Char  element
while(Tokens are Available)
 {
     ch = Read(Token);
     if(ch is Operand)
       {
       Print ch ;
       }
     else
       {
       while(Priority(ch) <= Priority(Top Most Stack))
            {
            element = Pop(S);
            Print(ele);
            }
       Push(S,ch);
       }
}
while(!Empty(S))
{
element = Pop(S);
Print(ele);
}
*/

enum TokenType {
    Number,
    Operator,
    LeftParenthesis,
    RightParenthesis,
}

pub struct Token {
    tokentype: TokenType,
    value: String,
}

impl Token {

    fn new(tokentype: TokenType, value: String) -> Self {
        Token { tokentype, value }
    }

    fn is_operator(&self) -> bool {
    	// this comparision doesn't work -.-
    	// self.tokentype == TokenType::Operator
    	
        match self.tokentype {
            TokenType::Operator => true,
            _ => false,
        }
        
    }

    fn is_number(&self) -> bool {
    	// this comparision doesn't work -.-
    	// self.tokentype == TokenType::Number

        match self.tokentype {
            TokenType::Number => true,
            _ => false,
        }
    }

    fn is_left_parenthesis(&self) -> bool {
    	// this comparision doesn't work -.-
    	// self.tokentype == TokenType::LeftParenthesis

        match self.tokentype {
            TokenType::LeftParenthesis => true,
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

        // return precedence.get(&self.value) >= precedence.get(&other.value);
        
        return precedence.get(&self.value) >= precedence.get(&other.value)
            && &self.value != &other.value;
    }

    fn is_right_parenthesis(&self) -> bool {
    	// this comparision doesn't work -.-
    	// self.tokentype == TokenType::RightParenthesis

        match self.tokentype {
            TokenType::RightParenthesis => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// impl std::cmp::PartialEq for TokenType {
// 	fn eq(&self, otherToken: &TokenType) -> bool {
// 		self == otherToken
// 	}
// }

fn str_to_token(infix: Vec<&str>) -> Vec<Token> {

	let mut token_list: Vec<Token> = vec![];

	for item in infix {
		// let token = lib::Token::new(lib::TokenType::Number, item.to_string());		
		// let token = item.chars().into();
		// let token = item.chars().map(|c| c.into()).collect::<Vec<lib::Token>>()[0];
		let token = item.to_string().parse::<u32>();
		let token = match token {
			Ok(_) => Token::new(TokenType::Number, item.to_string()),
			_ => match &item[..] {
				"(" => Token::new(TokenType::LeftParenthesis, item.to_string()),
				")"  => Token::new(TokenType::RightParenthesis, item.to_string()),
				_ => Token::new(TokenType::Operator, item.to_string())
			}
		};
		token_list.push(token);
		// println!("{}", item);
	}

	token_list
}

pub fn infix_to_postfix(infix_list: Vec<&str>) -> std::collections::VecDeque<Token> {
    let mut outputqueue: std::collections::VecDeque<Token> = std::collections::VecDeque::new();
    let mut operatorstack: Vec<Token> = Vec::new();
    let token_list: Vec<Token> = str_to_token(infix_list);

    // let mut counter = 0;

    for token in token_list {
    	// counter = counter + 1;
    	// println!("counter: {}", counter);
        if token.is_number() {
            outputqueue.push_back(token);
        }
        else if token.is_operator() {
            while !operatorstack.is_empty()
                && operatorstack
                    .last()
                    .unwrap()
                    .has_greater_precedence_than(&token)
                && operatorstack.last().unwrap().value != "("
            {
                // println!(
                //     "{:?} has higher precedence than {:?}",
                //     operatorstack.last().unwrap().value,
                //     token.value
                // );
                outputqueue.push_back(operatorstack.pop().unwrap());
            }
            //println!("pushing operator to stack");
            operatorstack.push(token);
        }

        else if token.is_left_parenthesis() {
            //println!("Token is left paren, pushing to stack");
            operatorstack.push(token);
        }
        else if token.is_right_parenthesis() {
            //println!("Token is right paren");
            //println!("current operatorstack looks like {:?}",operatorstack);
            while !operatorstack.last().unwrap().is_left_parenthesis() {
                //println!("{} is not left paren, so pushing it to outputqueue.",operatorstack.last().unwrap().value);
                outputqueue.push_back(operatorstack.pop().unwrap());
            }
            operatorstack.pop();
        }
    }
    while !operatorstack.is_empty() {
        //println!("there's an operator ({:?}) on the stack left, pushing it to the queue",operatorstack.last().unwrap());
        outputqueue.push_back(operatorstack.pop().unwrap());
    }
    outputqueue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}
