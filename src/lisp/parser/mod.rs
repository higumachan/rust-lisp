

use nom::{IResult, digit, anychar, alpha, alphanumeric, multispace};
use std::str;
use std::str::FromStr;
use std::string::String;
use std::iter::FromIterator;
use std::iter::Iterator;

named!(
    pub s_expr<super::core::Value>,
    alt!(
        s_expr_true | s_expr_quoted
    )
);

named!(
    s_expr_true<super::core::Value>,
    ws!(delimited!(tag!("("), s_expr_body, tag!(")") ))
);

named!(
    s_expr_quoted<super::core::Value>,
    map!(ws!(delimited!(tag!("'("), s_expr_body, tag!(")") )), |x: super::core::Value| {
        super::core::Value::from_iter(&vec![super::core::Value::TSymbol(super::core::SymbolType::Quote), x])
    })
);

named!(s_expr_body<super::core::Value>,
       do_parse!(
  cons: many0!(do_parse!(v: factor >> many0!(tag!(" ")) >> (v))) >>
  // cdr: many0!(do_parse!(multispace >> v: factor >> (v))) >>
  ({
      /*
      let mut v = vec![car];
      v.extend(cdr);
      for t in &v {
          println!("{}", t);
      }
      */
      super::core::Value::from_iter(&cons)
  })
));

named!(factor<super::core::Value>, alt!(atom | s_expr));

named!(atom<super::core::Value>, alt!(string | integer | symbol));

named!(string<super::core::Value>,
       do_parse!(
  tag!("\"") >>
  ss: map_res!(str_literal, |x: Vec<u8>| { return String::from_utf8(x); }) >>
  tag!("\"") >>
  (super::core::Value::TString(ss))
));

fn is_not_double_quote(chr: u8) -> bool {
    chr != b'\"'
}

named!(str_literal<Vec<u8>>,
    do_parse!(
        ss: take_while!(is_not_double_quote) >>
        (ss.to_vec())
    )
);

named!(integer<super::core::Value>,
       do_parse!(
  sign_v: sign >>
  number: map_res!(map_res!(ws!(digit), str::from_utf8), super::core::IntegerType::from_str) >>
  (super::core::Value::TInt(sign_v * number))
));

named!(symbol<super::core::Value>,
       do_parse!(
  head: alpha >>
  tail: many0!(alt!(alphanumeric | tag!("_") | tag!("!") | tag!("?"))) >>
  ({
    let mut v = head.to_vec();
    for t in tail {
        v.extend(t);
    }
    let key = String::from_utf8(v).unwrap();
    let mut hash = super::core::SYMBOL_HASH.lock().unwrap();

    if !hash.forward().contains_key(&key) {
        let l = super::core::SymbolType::Others(hash.len() as u64);
        hash.insert(key.clone(), l);
    }
    super::core::Value::TSymbol(hash.forward().get(&key).unwrap().clone())
  })
));

named!(sign<super::core::IntegerType>,do_parse!(
  sign_v: opt!(alt!(char!('+') | char!('-'))) >>
  (sign_v.map_or(1, |x: char| { if x == '+' { 1 } else { -1 } }))
));

#[cfg(test)]
mod tests {

    #[test]
    fn integer_expr_test() {
        assert_eq!(super::s_expr(b"(1 2 3)"), IResult::Done(b"", super::core::Value));
    }
}
