use std::sync::Mutex;
use rpds::HashTrieMap;

#[derive(Debug)]
enum Ty {
    TVar(usize),
    TArr(Box<Ty>, Box<Ty>),
}

#[derive(Debug)]
enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

#[derive(Debug)]
struct Teq(Ty, Ty);

lazy_static::lazy_static! {
    static ref N: Mutex<usize> = Mutex::new(0 as usize);
}

fn fresh<'a>() -> usize {
    let mut n = N.lock().unwrap();
    *n += 1;
    *n - 1
}

fn infer(env: &HashTrieMap<String, usize>, term: Term) -> (Ty, Vec<Teq>) {
    match term {
        Term::Var(x) => (Ty::TVar(*env.get(&x).unwrap()), vec![]),
        Term::Abs(x, t) => {
            let ax = fresh();
            let env2 = env.insert(x, ax);
            let (at, et) = infer(&env2, *t);
            (Ty::TArr(Box::new(Ty::TVar(ax)), Box::new(at)), et)
        }
        Term::App(t, u) => {
            let (at, et) = infer(env, *t);
            let (au, eu) = infer(env, *u);
            let ax = fresh();
            let mut eret = vec![Teq(at, Ty::TArr(Box::new(au), Box::new(Ty::TVar(ax))))];
            eret.extend(et);
            eret.extend(eu);
            (Ty::TVar(ax), eret)
        }
    }
}

fn main() {
    // λf.λx.f x
    let term = Term::Abs(
        "f".to_string(),
        Box::new(Term::Abs(
            "x".to_string(),
            Box::new(Term::App(
                Box::new(Term::Var("f".to_string())),
                Box::new(Term::Var("x".to_string())),
            )),
        )),
    );

    let (ty, eqs) = infer(&HashTrieMap::new(), term);
    println!("ty {:?}", ty);
    println!("eqs {:?}", eqs);
}
