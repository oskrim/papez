use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use rpds::HashTrieMap;

enum Expr {
    Var(String),
    Abs(String, Rc<RefCell<Expr>>),
    App(Rc<RefCell<Expr>>, Rc<RefCell<Expr>>),
    Pi(String, Rc<RefCell<Expr>>, Rc<RefCell<Expr>>),
    Type(usize),
    Nat,
    Zero,
    Succ(Rc<RefCell<Expr>>),
    Ind(Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Rc<RefCell<Expr>>),
    Id(Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Rc<RefCell<Expr>>),
    Refl(Rc<RefCell<Expr>>),
    J(Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Rc<RefCell<Expr>>),
}

enum Neutral {
    NVar(String),
    NApp(Rc<RefCell<Neutral>>, Value),
    NInd(Rc<RefCell<Neutral>>, Value, Value, Value),
    NJ(Value, Value, Value, Value, Value, Rc<RefCell<Neutral>>),
}

enum Value {
    VAbs(Rc<dyn Fn(Value) -> Value>),
    VPi(Rc<RefCell<Value>>, Rc<dyn Fn(Value) -> Value>),
    VType(usize),
    VNat,
    VZero,
    VSucc(Rc<RefCell<Value>>),
    VId(Rc<RefCell<Value>>, Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    VRefl(Rc<RefCell<Value>>),
    VNeutral(Rc<RefCell<Neutral>>),
}

fn vapp(u: Value, v: Value) -> Value {
    match u {
        Value::VAbs(f) => f(v),
        Value::VNeutral(n) => Value::VNeutral(Rc::new(RefCell::new(Neutral::NApp(n, v)))),
        _ => panic!("vapp"),
    }
}

fn eval(env: &'static mut HashMap<String, Rc<RefCell<Value>>>, t: Rc<RefCell<Expr>>) -> Rc<RefCell<Value>> {
    match &*t.borrow() {
        Expr::Var(x) => {
            match env.get(x) {
                Some(v) => Rc::clone(v),
                None => Rc::new(RefCell::new(Value::VNeutral(Rc::new(RefCell::new(Neutral::NVar(x.to_string()))))))
            }
        },
        _ => panic!("eval"),
    }
}

fn main() {
}
