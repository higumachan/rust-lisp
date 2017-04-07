#[macro_use]
extern crate nom;

#[macro_use]
extern crate lazy_static;


use nom::IResult;

use std::str;

#[macro_use]
mod lisp;

use lisp::core;
use lisp::parser;

macro_rules! cons {
    ($($x:expr),*) => (
        core::Value::from_iter(vec![$($x),*])
    );
    ($($x:expr,)*) => (core::Value::from_iter(vec![$($x),*]))
}

fn main() {

    /*
  let res: Value = Value::T_Cons({
      car: Value::T_Int(1),
      cdr: None,
  });
  */
    // assert_eq!(core::Value::T_Int(1), core::Value::T_Int(1));
    let x: core::Value = core::Value::from_iter(&vec![
    core::Value::TInt(12),
    core::Value::from_iter(&vec![core::Value::TInt(100), core::Value::TInt(101),]),
    core::Value::TInt(13)]);
    println!("{}", x);
    println!("{}", core::Value::from_iter(&vec![]));

    let x: core::Value = core::Value::from_iter(&vec![core::Value::TSymbol(core::SymbolType::Car as
                                                                           core::SymbolType),
                                                      core::Value::from_iter(&vec![
      core::Value::TSymbol(core::SymbolType::Quote as core::SymbolType),
      core::Value::from_iter(&vec![
        core::Value::TInt(100),
        core::Value::TInt(101),
      ]),
    ])]);
    println!("{}", x);
    println!("{}", core::eval(&x).unwrap());

    let x: core::Value = core::Value::from_iter(&vec![core::Value::TSymbol(core::SymbolType::Cdr as
                                                                           core::SymbolType),
                                                      core::Value::from_iter(&vec![
      core::Value::TSymbol(core::SymbolType::Quote as core::SymbolType),
      core::Value::from_iter(&vec![
        core::Value::TInt(100),
        core::Value::TInt(101),
      ]),
    ])]);
    println!("{}", core::eval(&x).unwrap());
    let c = cons![core::Value::TInt(100), core::Value::TInt(100)];
    println!("{}", c.pretty_print());

    match parser::s_expr(b"(1 2 (3 4))") {
        IResult::Done(_, result) => println!("{}", result),
        IResult::Error(e) => println!("error :{}", e),
        IResult::Incomplete(_) => println!("in complete"),
    }
    match parser::s_expr(b"(car (quote (1 2 3)))") {
        IResult::Done(r, result) => println!("{} {}", result, core::eval(&result).unwrap()),
        IResult::Error(e) => println!("error :{}", e),
        IResult::Incomplete(_) => println!("in complete"),
    }

    match parser::s_expr(b"(cdr '(1 2 3))") {
        IResult::Done(r, result) => println!("{} {}", result, core::eval(&result).unwrap()),
        IResult::Error(e) => println!("error :{}", e),
        IResult::Incomplete(_) => println!("in complete"),
    }

    match parser::s_expr(b"(\"test\" \"nadeko\")") {
        IResult::Done(r, result) => println!("{}", result),
        IResult::Error(e) => println!("error :{}", e),
        IResult::Incomplete(_) => println!("in complete"),
    }
}
