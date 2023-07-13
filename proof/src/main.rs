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
    Ind(Rc<Expr>, Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Id(Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Refl(Rc<Expr>),
    J(Rc<Expr>, Rc<Expr>, Rc<Expr>, Rc<Expr>, Rc<Expr>, Rc<Expr>),
}

enum Neutral {
    Var(String),
    App(Rc<Neutral>, Rc<Value>),
    Ind(Rc<Neutral>, Value, Value, Value),
    J(Rc<Value>, Rc<Value>, Rc<Value>, Rc<Value>, Rc<Value>, Rc<Neutral>),
}

enum Value {
    Abs(HashTrieMap<String, Rc<Value>>, String, Rc<Expr>),
    Pi(Rc<Value>, HashTrieMap<String, Rc<Value>>, String, Rc<Expr>),
    Type(usize),
    Nat,
    Zero,
    Succ(Rc<Value>),
    Id(Rc<Value>, Rc<Value>, Rc<Value>),
    Refl(Rc<Value>),
    Neutral(Rc<Neutral>),
}

fn vapp(u: Rc<Value>, v: Rc<Value>) -> Rc<Value> {
    match u.as_ref() {
        Value::Abs(env, x, e) => {
            let env2 = env.insert(x.to_string(), Rc::clone(&v));
            eval(env2, Rc::clone(&e))
        }
        Value::Neutral(n) => Rc::new(Value::Neutral(Rc::new(Neutral::App(Rc::clone(n), v)))),
        _ => panic!("vapp"),
    }
}

fn f(k: Rc<Value>, a: Rc<Value>, z: Rc<Value>, s: Rc<Value>) -> Rc<Value> {
    match k.as_ref() {
        Value::Zero => Rc::clone(&z),
        Value::Succ(l) => vapp(vapp(Rc::clone(&s), Rc::clone(&l)), f(Rc::clone(&l), a, z, s)),
        _ => panic!("Ind"),
    }
}

fn eval(env: HashTrieMap<String, Rc<Value>>, t: Rc<Expr>) -> Rc<Value> {
    match t.as_ref() {
        Expr::Var(x) => match env.get(x) {
            Some(v) => Rc::clone(v),
            None => Rc::new(Value::Neutral(Rc::new(Neutral::Var(x.to_string())))),
        },
        Expr::Abs(x, e) => Rc::new(Value::Abs(env, x.to_string(), e.clone())),
        Expr::App(e1, e2) => vapp(eval(env.clone(), e1.clone()), eval(env, e2.clone())),
        Expr::Pi(x, a, e) => Rc::new(Value::Pi(eval(env.clone(), a.clone()), env, x.to_string(), e.clone())),
        Expr::Type(n) => Rc::new(Value::Type(*n)),
        Expr::Nat => Rc::new(Value::Nat),
        Expr::Zero => Rc::new(Value::Zero),
        Expr::Succ(e) => Rc::new(Value::Succ(eval(env, e.clone()))),
        Expr::Ind(n, a, z, s) => {
            let n2 = eval(env.clone(), n.clone());
            let a2 = eval(env.clone(), a.clone());
            let z2 = eval(env.clone(), z.clone());
            let s2 = eval(env.clone(), s.clone());
            f(n2, a2, z2, s2)
        },
        Expr::Id(a, t, u) => {
            let a2 = eval(env.clone(), a.clone());
            let t2 = eval(env.clone(), t.clone());
            let u2 = eval(env, u.clone());
            Rc::new(Value::Id(a2, t2, u2))
        },
        Expr::Refl(t) => {
            let t2 = eval(env, t.clone());
            Rc::new(Value::Refl(t2))
        },
        Expr::J(a, p, r, t, u, e) => match eval(env.clone(), e.clone()).as_ref() {
            Value::Neutral(n) => Rc::new(Value::Neutral(Rc::new(Neutral::J(
                eval(env.clone(), a.clone()),
                eval(env.clone(), p.clone()),
                eval(env.clone(), r.clone()),
                eval(env.clone(), t.clone()),
                eval(env.clone(), u.clone()),
                n.clone(),
            )))),
            _ => panic!("J"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vapp() {
        let f = Value::Abs(
            HashTrieMap::new(),
            "x".to_string(),
            Rc::new(Expr::Var("x".to_string())),
        );
        let v = Value::Nat;
        let result = vapp(Rc::new(f), Rc::new(v));
        match result.as_ref() {
            Value::Nat => (),
            _ => panic!("test_vapp"),
        }
    }

    #[test]
    fn test_eval() {
        let env = HashTrieMap::new().insert("x".to_string(), Rc::new(Value::Nat));

        let x = Rc::new(Expr::Var("x".to_string()));
        let result = eval(env.clone(), x);
        match result.as_ref() {
            Value::Nat => (),
            _ => panic!("test_eval x"),
        }

        let y = Rc::new(Expr::Var("y".to_string()));
        let result = eval(env.clone(), y);
        match result.as_ref() {
            Value::Neutral(_) => (),
            _ => panic!("test_eval y"),
        }
    }
}

fn main() {
    println!("main")
}
