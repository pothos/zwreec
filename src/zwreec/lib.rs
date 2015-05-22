#![feature(plugin)]
#![plugin(rustlex)]
#[allow(plugin_as_library)]
extern crate rustlex;
#[macro_use] extern crate log;
extern crate time;
extern crate term;

pub mod frontend;
pub mod backend;
pub use backend::zcode::zfile;
pub mod utils;

use frontend::codegen;

use std::io::{Read,Write};


pub fn compile<R: Read, W: Write>(input: &mut R, output: &mut W) {
    // compile

    // tokenize
    let tokens = frontend::lexer::lex(input);

    println!("");
    for token in tokens.iter() {
    	debug!("{:?}", token);
    }

    // parse tokens and create ast
    let ast = frontend::parser::parse_tokens(tokens);
    ast.print();

    // create code
    codegen::generate_zcode(ast, output);
}

#[test]
fn it_works() {
}
