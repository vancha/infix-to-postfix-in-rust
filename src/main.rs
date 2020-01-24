extern crate failure;
extern crate gio;
extern crate gtk;

use failure::Error;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Box, ContainerExt, WidgetExt};
//use libhandy;
//use libhandy::prelude::*;
use std::env;
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


#[derive( Clone)]
enum TokenType {
    Number,
    Operator,
    LeftParenthesis,
    RightParenthesis,
}
#[derive( Clone)]
struct Token {
    tokentype: TokenType,
    numbervalue: i32,
    stringvalue: String,
}
impl Token {
    fn new(tokentype: TokenType, numbervalue: i32, stringvalue: String) -> Self {
        Token {tokentype, numbervalue, stringvalue }
        
    }
    fn is_operator(&self) -> bool {
        match self.tokentype {
            TokenType::Operator => true,
            _ => false,
        }
            
    }
    fn is_number(&self) -> bool {
        match self.tokentype {
            TokenType::Number => true,
            _ => false,
        }
    }
    fn is_left_parenthesis(&self) -> bool  {
        match self.tokentype {
            TokenType::LeftParenthesis => true,
            _ => false,
        }
    }
    
    /* this is horrible, change this */
    fn has_greater_precedence_than(&self, other: &Token)-> bool {
        match self.stringvalue.as_str() {
            "*" => {
                match other.stringvalue.as_str() {
                    "+" => true,
                    "-" => true,
                    _ => false,
                }
            },
            "-" => {
                match other.stringvalue.as_str() {
                    _ => false,
                }    
            },
            "/" => {
                match other.stringvalue.as_str() {
                    "*" => false,
                    _ => true,
                }
            },
            "+" => {
                match other.stringvalue.as_str() {
                    "-" => true,
                    _ => false,
                }
            },
            "(" => {
                match other.stringvalue.as_str() {
                    _ => true,
                }
            },
            ")" => {
                match other.stringvalue.as_str() {
                    _ => true,
                }
            },
            _ => {
                return false
            }
        }
    }
    
    fn is_right_parenthesis(&self) -> bool  {
       match self.tokentype {
            TokenType::RightParenthesis => true,
            _ => false,
        }
    }
    
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
        match self.tokentype {
            TokenType::Number => write!(f,"{}",self.numbervalue),
            _ => write!(f,"{}",self.stringvalue),
        }
    }
}

fn infix_to_postfix(token_list: Vec<&Token>)->std::collections::VecDeque<&Token> {
    let mut outputqueue: std::collections::VecDeque<&Token> = std::collections::VecDeque::new();
    let mut operatorstack: Vec<&Token> = Vec::new();
    
    for token in token_list {
        println!("Reading token.");
        if token.is_number() {
            println!("Token is number, pushing to queue");
            outputqueue.push_back(token);
        }
        if token.is_operator() {
            println!("Token is operator");
            while(!operatorstack.is_empty() && operatorstack.last().unwrap().has_greater_precedence_than(token)) {
                println!("token on stack has higher precedence, pushing the one on stack to queue");
                outputqueue.push_back(operatorstack.pop().unwrap());
            }
            println!("pushing operator to stack");
            operatorstack.push(token);
        }
        
        if token.is_left_parenthesis() {
            println!("Token is left paren, pushing to stack");
            operatorstack.push(token);
        }
        if token.is_right_parenthesis() {
            println!("Token is right paren");
            while(!operatorstack.last().unwrap().is_left_parenthesis()) {
                println!("Token is right paren");
                outputqueue.push_back(operatorstack.pop().unwrap());    
            }
            operatorstack.pop();
        }
    }
    while(!operatorstack.is_empty()) {
        println!("there's an operator on the stack left, pushing it to the queue");
        outputqueue.push_back(operatorstack.pop().unwrap());
    }
    outputqueue
}

fn main() -> Result<(), Error> {
    gtk::init()?;
    let uiapp = gtk::Application::new(Some("gtk-rs.test.app"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(320, 200);
        win.set_title("Calculator");
        //let arrowup: libhandy::Arrows = libhandy::Arrows::new();
        //let arrowclone = arrowup.clone();
        let container = Box::new(gtk::Orientation::Vertical, 5);
        let displayrow = Box::new(gtk::Orientation::Horizontal, 5);
        let buttonrow1 = Box::new(gtk::Orientation::Horizontal, 5);
        let buttonrow2 = Box::new(gtk::Orientation::Horizontal, 5);
        let buttonrow3 = Box::new(gtk::Orientation::Horizontal, 5);
        let buttonrow4 = Box::new(gtk::Orientation::Horizontal, 5);

        let mut x: Vec<&Token> = vec![];
        let t1 = Token::new(TokenType::Number, 1,std::string::String::from(""));
        let t2 = Token::new(TokenType::Operator, 0,std::string::String::from("+"));
        let t3 = Token::new(TokenType::Number, 1,std::string::String::from(""));
        let t4 = Token::new(TokenType::Operator, 0,std::string::String::from("*"));
        let t5 = Token::new(TokenType::Number, 1,std::string::String::from(""));
        let t6 = Token::new(TokenType::Operator, 0,std::string::String::from("-"));
        let t7 = Token::new(TokenType::Number, 1,std::string::String::from(""));
        x.push(&t1);
        x.push(&t2);
        x.push(&t3);
        x.push(&t4);
        x.push(&t5);
        x.push(&t6);
        x.push(&t7);
        let things = infix_to_postfix(x);
        println!("{:?}",things);

        let display = gtk::Entry::new();
        display.set_size_request(330, 10);
        let button7 = gtk::Button::new_with_label("7");
        let button8 = gtk::Button::new_with_label("8");
        let button9 = gtk::Button::new_with_label("9");
        let buttondivide = gtk::Button::new_with_label("/");
        let buttonundo = gtk::Button::new_with_label("<");
        let buttonreset = gtk::Button::new_with_label("C");

        let button4 = gtk::Button::new_with_label("4");
        let button5 = gtk::Button::new_with_label("5");
        let button6 = gtk::Button::new_with_label("6");
        let buttonmultiply = gtk::Button::new_with_label("*");
        let buttonopeningbrace = gtk::Button::new_with_label("(");
        let buttonclosingbrace = gtk::Button::new_with_label(")");

        let button1 = gtk::Button::new_with_label("1");
        let button2 = gtk::Button::new_with_label("2");
        let button3 = gtk::Button::new_with_label("3");
        let buttonminus = gtk::Button::new_with_label("-");
        let buttonsquare = gtk::Button::new_with_label("^2");
        let buttonroot = gtk::Button::new_with_label("√");

        let button0 = gtk::Button::new_with_label("0");
        let buttoncomma = gtk::Button::new_with_label(",");
        let buttonpercent = gtk::Button::new_with_label("%");
        let buttonplus = gtk::Button::new_with_label("+");
        let buttonequals = gtk::Button::new_with_label("=");

        displayrow.pack_start(&display, true, true, 5);

        buttonrow1.pack_start(&button7, true, true, 5);
        buttonrow1.pack_start(&button8, true, true, 5);
        buttonrow1.pack_start(&button9, true, true, 5);
        buttonrow1.pack_start(&buttondivide, true, true, 5);
        buttonrow1.pack_start(&buttonundo, true, true, 5);
        buttonrow1.pack_start(&buttonreset, true, true, 5);

        buttonrow2.pack_start(&button4, true, true, 5);
        buttonrow2.pack_start(&button5, true, true, 5);
        buttonrow2.pack_start(&button6, true, true, 5);
        buttonrow2.pack_start(&buttonmultiply, true, true, 5);
        buttonrow2.pack_start(&buttonopeningbrace, true, true, 5);
        buttonrow2.pack_start(&buttonclosingbrace, true, true, 5);

        buttonrow3.pack_start(&button1, true, true, 5);
        buttonrow3.pack_start(&button2, true, true, 5);
        buttonrow3.pack_start(&button3, true, true, 5);
        buttonrow3.pack_start(&buttonminus, true, true, 5);
        buttonrow3.pack_start(&buttonsquare, true, true, 5);
        buttonrow3.pack_start(&buttonroot, true, true, 5);

        buttonrow4.pack_start(&button0, true, true, 5);
        buttonrow4.pack_start(&buttoncomma, true, true, 5);
        buttonrow4.pack_start(&buttonpercent, true, true, 5);
        buttonrow4.pack_start(&buttonplus, true, true, 5);
        buttonrow4.pack_start(&buttonequals, true, true, 5);

        container.pack_start(&displayrow, true, true, 5);
        container.pack_start(&buttonrow1, true, true, 5);
        container.pack_start(&buttonrow2, true, true, 5);
        container.pack_start(&buttonrow3, true, true, 5);
        container.pack_start(&buttonrow4, true, true, 5);

        win.add(&container);
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
    Ok(())
}
