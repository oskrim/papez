use std::sync::Mutex;
use rpds::HashTrieMap;

#[derive(Debug, PartialEq)]
enum Ty {
    TVar(Tvar),
    TArr(Box<Ty>, Box<Ty>),
}

#[derive(Debug, PartialEq)]
enum Tvar {
    AVar(usize),
    Link(Box<Ty>),
}

#[derive(Debug, PartialEq)]
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

fn unlink(t: &Ty) -> Ty {
    match t {
        Ty::TVar(Tvar::AVar(x)) => Ty::TVar(Tvar::AVar(*x)),
        Ty::TVar(Tvar::Link(t)) => unlink(t),
        Ty::TArr(t1, t2) => Ty::TArr(Box::new(unlink(t1)), Box::new(unlink(t2))),
    }
}

fn occurs(x: usize, t: &Ty) -> bool {
    match t {
        Ty::TVar(Tvar::AVar(y)) => x == *y,
        Ty::TVar(Tvar::Link(t)) => occurs(x, t),
        Ty::TArr(t1, t2) => occurs(x, t1) || occurs(x, t2),
    }
}

// fn infer(env: &HashTrieMap<String, usize>, term: Term) -> (Ty, Vec<Teq>) {
//     match term {
//         Term::Var(x) => (Ty::TVar(*env.get(&x).unwrap()), vec![]),
//         Term::Abs(x, t) => {
//             let ax = fresh();
//             let env2 = env.insert(x, ax);
//             let (at, et) = infer(&env2, *t);
//             (Ty::TArr(Box::new(Ty::TVar(ax)), Box::new(at)), et)
//         }
//         Term::App(t, u) => {
//             let (at, et) = infer(env, *t);
//             let (au, eu) = infer(env, *u);
//             let ax = fresh();
//             let mut eret = vec![Teq(at, Ty::TArr(Box::new(au), Box::new(Ty::TVar(ax))))];
//             eret.extend(et);
//             eret.extend(eu);
//             (Ty::TVar(ax), eret)
//         }
//     }
// }

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

    let unlinked = unlink(&Ty::TVar(Tvar::Link(Box::new(Ty::TVar(Tvar::AVar(0))))));
    assert!(unlinked == Ty::TVar(Tvar::AVar(0)));
    assert!(occurs(0, &unlinked));
}
