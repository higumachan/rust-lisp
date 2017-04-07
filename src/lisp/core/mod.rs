use std::fmt::Display;
use std::fmt;
use std::fmt::Write;
use std::collections::HashMap;
use std::sync::Mutex;
use std::hash::Hash;
use std::clone::Clone;
use std::ops::Deref;

pub type IntegerType = i32;

pub struct DualMap<K, V>
    where K: Eq + Hash + Clone,
          V: Eq + Hash + Clone
{
    _forward: HashMap<K, V>,
    _backward: HashMap<V, K>,
}

impl<K, V> DualMap<K, V>
    where K: Eq + Hash + Clone,
          V: Eq + Hash + Clone
{
    pub fn new() -> DualMap<K, V> {
        return DualMap {
                   _forward: HashMap::new(),
                   _backward: HashMap::new(),
               };
    }


    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self._backward.insert(v.clone(), k.clone());
        self._forward.insert(k.clone(), v.clone())
    }

    pub fn forward(&self) -> &HashMap<K, V> {
        &self._forward
    }

    pub fn backward(&self) -> &HashMap<V, K> {
        &self._backward
    }


    pub fn len(&self) -> usize {
        self._forward.len()
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum SymbolType {
    Cond,
    Car,
    Cdr,
    Quote,
    Cons,
    Add,
    Sub,
    Mul,
    Div,
    Others(u64),
}

lazy_static! {
  pub static ref SYMBOL_HASH: Mutex<DualMap<String, SymbolType>> = {
    let mut v = DualMap::new();
    v.insert("car".to_string(), SymbolType::Car);
    v.insert("cdr".to_string(), SymbolType::Cdr);
    v.insert("quote".to_string(), SymbolType::Quote);
    return Mutex::new(v);
  };
}

impl SymbolType {
    fn to_string(&self) -> &str {
        match *self {
            SymbolType::Car => "Car",
            SymbolType::Cdr => "Cdr",
            SymbolType::Quote => "Quote",
            _ => "unknown",
        }
    }
}


#[derive(Clone, PartialEq)]
pub enum Value {
    TInt(IntegerType),
    TString(String),
    TFloat(f32),
    TDouble(f64),
    TSymbol(SymbolType),
    TCons(Box<Value>, Box<Value>),
    TBoolT,
    TNil,
}

pub enum LispError {
    ConsNoMatchError,
    ConsSyntaxError,
}

impl Value {
    pub fn car(&self) -> Option<&Value> {
        match *self {
            Value::TCons(ref car, ref cdr) => Some(&**car),
            _ => None,
        }
    }

    pub fn cdr(&self) -> Option<&Value> {
        match *self {
            Value::TCons(ref car, ref cdr) => Some(&**cdr),
            _ => None,
        }
    }

    pub fn pretty_print(&self) -> String {
        match *self {
            Value::TInt(i) => i.to_string(),
            Value::TFloat(f) => f.to_string(),
            Value::TString(ref s) => {
                let mut str: String = String::from("\"");
                str.write_str(s);
                str.write_str("\"");
                return str;
            }
            Value::TCons(ref car, ref cdr) => {
                let mut str: String = String::from("(");
                let mut a: &Value = car;
                let mut b: &Value = cdr;
                while (true) {
                    str.write_str(&*a.pretty_print());
                    match *b {
                        Value::TNil => break,
                        _ => {}
                    }
                    str.write_char(' ');
                    a = b.car().unwrap();
                    b = b.cdr().unwrap();
                }
                str.write_char(')');
                return str;
            }
            Value::TNil => "nil".to_string(),
            Value::TSymbol(ref symbol) => {
                SYMBOL_HASH.lock().unwrap().backward().get(&symbol).unwrap().clone()
            } // TODO: undefined err
            _ => "".to_string(),
        }
    }

    pub fn from_iter(iter: &Vec<Value>) -> Value {
        let mut ago: Value;
        let mut res: Value = Value::TNil;
        for v in iter.into_iter().rev() {
            ago = res;
            res = Value::TCons(Box::new(v.clone()), Box::new(ago));
        }
        return res;
    }
}


impl<'a> Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

pub fn eval(cons: &Value) -> Option<&Value> {
    match *cons {
        Value::TCons(ref car, ref cdr) => {
            match **car {
                Value::TSymbol(SymbolType::Car) => {
                    return do_function(&**cdr, |x: &Value| match *x {
                        Value::TCons(ref car, ref cdr) => return Some(&**car),
                        _ => return None,
                    })
                }
                Value::TSymbol(SymbolType::Cdr) => {
                    return do_function(&**cdr, |x: &Value| match *x {
                        Value::TCons(ref car, ref cdr) => return Some(&**cdr),
                        _ => return None,
                    })
                }
                Value::TSymbol(SymbolType::Cond) => return do_cond(&**cdr),
                Value::TSymbol(SymbolType::Quote) => return do_macro(&**cdr, |x: &Value| Some(x)),
                _ => return None,
            }
        }
        _ => return None,
    }

}

fn do_function<F>(cons: &Value, func: F) -> Option<&Value>
    where F: FnOnce(&Value) -> Option<&Value>
{
    return cons.car().and_then(eval).and_then(func);
}

fn do_macro<F>(cons: &Value, func: F) -> Option<&Value>
    where F: FnOnce(&Value) -> Option<&Value>
{
    return cons.car().and_then(func);
}

fn do_cond(cons: &Value) -> Option<&Value> {
    let mut cons_opt = Some(cons);
    while let Some(cond_value) = cons_opt {
        let condition = cond_value.car().and_then(|x| x.car()).and_then(eval).unwrap();
        let true_cons = cond_value.car().and_then(|x| x.cdr()).and_then(|x| x.car());
        cons_opt = cond_value.cdr();
        match *condition {
            Value::TBoolT => return true_cons.and_then(eval),
            Value::TNil => break,
        }
    }
    return None; // Todo: Err ConsNotMatch
}
