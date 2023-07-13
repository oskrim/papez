use rpds::HashTrieMap;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
enum Neutral {
    Var(String),
    App(Rc<Neutral>, Rc<Value>),
    Ind(Rc<Neutral>, Rc<Value>, Rc<Value>, Rc<Value>),
    J(
        Rc<Value>,
        Rc<Value>,
        Rc<Value>,
        Rc<Value>,
        Rc<Value>,
        Rc<Neutral>,
    ),
}

#[derive(PartialEq, Debug)]
enum Value {
    Abs(HashTrieMap<String, Rc<Value>>, String, Rc<Expr>),
    Pi(Rc<Value>, Rc<Value>),
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
        Value::Succ(l) => vapp(
            vapp(Rc::clone(&s), Rc::clone(&l)),
            f(Rc::clone(&l), a, z, s),
        ),
        Value::Neutral(n) => Rc::new(Value::Neutral(Rc::new(Neutral::Ind(
            Rc::clone(n),
            a,
            z,
            s,
        )))),
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
        Expr::Pi(x, a, e) => {
            let e2 = Rc::new(Value::Abs(env.clone(), x.to_string(), e.clone()));
            Rc::new(Value::Pi(eval(env, a.clone()), e2))
        }
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
        }
        Expr::Id(a, t, u) => {
            let a2 = eval(env.clone(), a.clone());
            let t2 = eval(env.clone(), t.clone());
            let u2 = eval(env, u.clone());
            Rc::new(Value::Id(a2, t2, u2))
        }
        Expr::Refl(t) => {
            let t2 = eval(env, t.clone());
            Rc::new(Value::Refl(t2))
        }
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
        },
    }
}

fn neutral(k: usize, n: Rc<Neutral>) -> Rc<Expr> {
    match n.as_ref() {
        Neutral::Var(x) => Rc::new(Expr::Var(x.to_string())),
        Neutral::App(u, v) => Rc::new(Expr::App(
            neutral(k, Rc::clone(&u)),
            readback(k, Rc::clone(&v)),
        )),
        Neutral::Ind(n, a, z, s) => Rc::new(Expr::Ind(
            neutral(k, Rc::clone(&n)),
            readback(k, Rc::clone(&a)),
            readback(k, Rc::clone(&z)),
            readback(k, Rc::clone(&s)),
        )),
        Neutral::J(a, p, r, t, u, e) => Rc::new(Expr::J(
            readback(k, Rc::clone(&a)),
            readback(k, Rc::clone(&p)),
            readback(k, Rc::clone(&r)),
            readback(k, Rc::clone(&t)),
            readback(k, Rc::clone(&u)),
            neutral(k, Rc::clone(&e)),
        )),
    }
}

fn fresh(k: usize) -> String {
    format!("@{}", k)
}

fn readback(k: usize, v: Rc<Value>) -> Rc<Expr> {
    match v.as_ref() {
        Value::Abs(env, x, e) => {
            let y = fresh(k);
            let ny = Rc::new(Value::Neutral(Rc::new(Neutral::Var(y))));
            let env2 = env.insert(x.to_string(), ny);
            let result = eval(env2, Rc::clone(&e));
            Rc::new(Expr::Abs(x.to_string(), readback(k + 1, result)))
        }
        Value::Pi(a, b) => {
            let x = fresh(k);
            let y = readback(k, a.clone());
            let arg = Rc::new(Value::Neutral(Rc::new(Neutral::Var(x.to_string()))));
            let z = readback(k + 1, vapp(Rc::clone(&b), arg));
            Rc::new(Expr::Pi(x.to_string(), y, z))
        }
        Value::Type(i) => Rc::new(Expr::Type(*i)),
        Value::Nat => Rc::new(Expr::Nat),
        Value::Zero => Rc::new(Expr::Zero),
        Value::Succ(n) => Rc::new(Expr::Succ(readback(k, Rc::clone(&n)))),
        Value::Id(a, t, u) => Rc::new(Expr::Id(
            readback(k, Rc::clone(&a)),
            readback(k, Rc::clone(&t)),
            readback(k, Rc::clone(&u)),
        )),
        Value::Refl(t) => Rc::new(Expr::Refl(readback(k, Rc::clone(&t)))),
        Value::Neutral(t) => neutral(k, t.clone()),
    }
}

fn veq(k: usize, u: Rc<Value>, v: Rc<Value>) -> bool {
    readback(k, u) == readback(k, v)
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
        assert_eq!(result, Rc::new(Value::Nat));
    }

    #[test]
    fn test_eval() {
        let env = HashTrieMap::new().insert("x".to_string(), Rc::new(Value::Nat));

        let x = Rc::new(Expr::Var("x".to_string()));
        let result = eval(env.clone(), x);
        assert_eq!(*result.as_ref(), Value::Nat);

        let y = Rc::new(Expr::Var("y".to_string()));
        let result = eval(env.clone(), y);
        assert_eq!(result, Rc::new(Value::Neutral(Rc::new(Neutral::Var("y".to_string())))));
    }

    #[test]
    fn test_veq() {
        let u = Rc::new(Value::Nat);
        let v = Rc::new(Value::Nat);
        assert!(veq(0, u, v));

        let u = Rc::new(Value::Succ(Rc::new(Value::Zero)));
        let v = eval(HashTrieMap::new(), Rc::new(Expr::Succ(Rc::new(Expr::Zero))));
        assert!(veq(0, u, v));
    }
}

fn main() {
    println!("main")
}
