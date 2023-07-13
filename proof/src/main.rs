use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use rpds::HashTrieMap;

enum Expr {
    Var(String),
    Abs(String, Rc<Expr>),
    App(Rc<Expr>, Rc<Expr>),
    Pi(String, Rc<Expr>, Rc<Expr>),
    Type(usize),
    Nat,
    Zero,
    Succ(Rc<Expr>),
    Ind(Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Id(Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Refl(Rc<Expr>),
    J(Rc<Expr>, Rc<Expr>, Rc<Expr>, Rc<Expr>, Rc<Expr>, Rc<Expr>),
}

enum Neutral {
    NVar(String),
    NApp(Rc<Neutral>, Value),
    NInd(Rc<Neutral>, Value, Value, Value),
    NJ(Value, Value, Value, Value, Value, Rc<Neutral>),
}

enum Value {
    VAbs(Rc<dyn Fn(Value) -> Value>),
    VPi(Rc<Value>, Rc<dyn Fn(Value) -> Value>),
    VType(usize),
    VNat,
    VZero,
    VSucc(Rc<Value>),
    VId(Rc<Value>, Rc<Value>, Rc<Value>),
    VRefl(Rc<Value>),
    VNeutral(Rc<Neutral>),
}

fn vapp(u: Value, v: Value) -> Value {
    match u {
        Value::VAbs(f) => f(v),
        Value::VNeutral(n) => Value::VNeutral(Rc::new(Neutral::NApp(n, v))),
        _ => panic!("vapp"),
    }
}

fn eval(env: &'static mut HashMap<String, Rc<Value>>, t: Rc<Expr>) -> Rc<Value> {
    match t.as_ref() {
        Expr::Var(x) => {
            match env.get(x) {
                Some(v) => Rc::clone(v),
                None => Rc::new(Value::VNeutral(Rc::new(Neutral::NVar(x.to_string()))))
            }
        },
        _ => panic!("eval"),
    }
}

fn main() {
}
