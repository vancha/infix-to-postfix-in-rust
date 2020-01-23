extern crate failure;
extern crate gio;
extern crate gtk;

use failure::Error;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Box, Button, ButtonExt, ContainerExt, Entry, WidgetExt};
use libhandy;
use libhandy::prelude::*;
use std::env;
use std::error;

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

fn postfixEvaluation() -> f64 {
    6 as f64
}
fn isFunction(c: &String) -> bool {
    match c.as_str() {
        "*" => true,
        "+" => true,
        "-" => true,
        "/" => true,
        "^" => true,
        _   => false,

    }

}
fn isLeftParen(c: String)-> bool {
    match c.as_str() {
        "(" => true,
        _ => false,
    }
}

fn isOperator(c: String) -> bool {
    false
}

fn isNumber(c: &String) -> bool {
    match c.as_str() {
        "1" => true,
        "2" => true,
        "3" => true,
        "5" => true,
        "6" => true,
        "7" => true,
        "8" => true,
        "9" => true,
        "10" => true,
        _ => false,
    }
}

/**
 * This method implements the shunting yard algorithm for converting infix to postfix
 * */
fn infix_to_postfix(characters: Vec<String>) {
    let mut outputqueue: std::collections::VecDeque<String> = std::collections::VecDeque::new();
    let mut operatorstack: Vec<String> = vec![];
    for x in characters {
    if(isNumber(&x)) {
        outputqueue.push_back(x.clone());
    }
    if(isFunction(&x)) {
        operatorstack.push(x);
    }
 
    
        /*match x {
            true => {
                outputqueue.push(x);
            }
            false => {
                

                if x == "(" {
                    operatorstack.push(std::string::String::from("("));
                }
                if x == ")" {
                    
                }
            }
        }*/
    }
    while !operatorstack.is_empty() { 
        outputqueue.push_back(operatorstack.pop().unwrap());
    }

    println!("before: {:?}",outputqueue);
    /*while !outputqueue.is_empty() {
        let val = outputqueue.pop().unwrap();
        print!(" {:?} ", val);
    }*/
}

fn main() -> Result<(), Error> {
    gtk::init()?;
    let uiapp = gtk::Application::new(Some("gtk-rs.test.app"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        let buttons: Vec<gtk::Button> = vec![];
        win.set_default_size(320, 200);
        win.set_title("Calculator");
        let arrowup: libhandy::Arrows = libhandy::Arrows::new();
        let arrowclone = arrowup.clone();
        let container = Box::new(gtk::Orientation::Vertical, 5);
        let displayrow = Box::new(gtk::Orientation::Horizontal, 5);
        let buttonrow1 = Box::new(gtk::Orientation::Horizontal, 5);
        let buttonrow2 = Box::new(gtk::Orientation::Horizontal, 5);
        let buttonrow3 = Box::new(gtk::Orientation::Horizontal, 5);
        let buttonrow4 = Box::new(gtk::Orientation::Horizontal, 5);

        let mut x: Vec<String> = vec![];
        x.push("1".to_string());
        x.push("+".to_string());
        x.push("1".to_string());
        infix_to_postfix(x);

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

        //buttonrow1.add(&button7);
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

        /*let label = gtk::Label::new(Some("tet"));
        let label2 = label.clone();
        let button = gtk::Button::new_with_label("test");
        button.connect_clicked(move |but| { label2.set_label("Button has been clicked!");arrowup.animate(); });
        container.add(&arrowclone);
        container.add(&button);
        container.add(&label);
        win.add(&container);*/
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
    Ok(())
}
