use rpds::HashTrieMap;
use std::rc::Rc;

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
    NApp(Rc<Neutral>, Rc<Value>),
    NInd(Rc<Neutral>, Value, Value, Value),
    NJ(Value, Value, Value, Value, Value, Rc<Neutral>),
}

enum Value {
    VAbs(HashTrieMap<String, Rc<Value>>, String, Rc<Expr>),
    VPi(Rc<Value>, Rc<dyn FnOnce(Value) -> Value>),
    VType(usize),
    VNat,
    VZero,
    VSucc(Rc<Value>),
    VId(Rc<Value>, Rc<Value>, Rc<Value>),
    VRefl(Rc<Value>),
    VNeutral(Rc<Neutral>),
}

fn vapp(u: Rc<Value>, v: Rc<Value>) -> Rc<Value> {
    match u.as_ref() {
        Value::VAbs(env, x, e) => {
            let env2 = env.insert(x.to_string(), Rc::clone(&v));
            eval(env2, Rc::clone(&e))
        }
        Value::VNeutral(n) => Rc::new(Value::VNeutral(Rc::new(Neutral::NApp(Rc::clone(n), v)))),
        _ => panic!("vapp"),
    }
}

fn eval(env: HashTrieMap<String, Rc<Value>>, t: Rc<Expr>) -> Rc<Value> {
    match t.as_ref() {
        Expr::Var(x) => match env.get(x) {
            Some(v) => Rc::clone(v),
            None => Rc::new(Value::VNeutral(Rc::new(Neutral::NVar(x.to_string())))),
        },
        Expr::Abs(x, e) => Rc::new(Value::VAbs(env.clone(), x.to_string(), e.clone())),
        _ => panic!("eval"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vapp() {
        let f = Value::VAbs(
            HashTrieMap::new(),
            "x".to_string(),
            Rc::new(Expr::Var("x".to_string())),
        );
        let v = Value::VNat;
        let result = vapp(Rc::new(f), Rc::new(v));
        match result.as_ref() {
            Value::VNat => (),
            _ => panic!("test_vapp"),
        }
    }

    #[test]
    fn test_eval() {
        let env = HashTrieMap::new().insert("x".to_string(), Rc::new(Value::VNat));

        let x = Rc::new(Expr::Var("x".to_string()));
        let result = eval(env.clone(), x);
        match result.as_ref() {
            Value::VNat => (),
            _ => panic!("test_eval x"),
        }

        let y = Rc::new(Expr::Var("y".to_string()));
        let result = eval(env.clone(), y);
        match result.as_ref() {
            Value::VNeutral(_) => (),
            _ => panic!("test_eval y"),
        }
    }
}

fn main() {
    println!("main")
}
