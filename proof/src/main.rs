use std::sync::Mutex;
use rpds::HashTrieMap;

#[derive(Debug, PartialEq, Clone)]
enum Ty {
    TVar(Tvar),
    TArr(Box<Ty>, Box<Ty>),
}

#[derive(Debug, PartialEq, Clone)]
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

fn unlink<'a>(t: &Ty) -> Ty {
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

fn unify(a: &'static mut Ty, b: &'static mut Ty) {
    let ua: Ty = unlink(a);
    match (ua, b.clone()) {
        (Ty::TVar(v), b) => {
            match v {
                Tvar::Link(_t) => panic!("unification failed"),
                Tvar::AVar(x) => {
                    if occurs(x, &b) {
                        panic!("unification failed")
                    } else {
                        *a = Ty::TVar(Tvar::Link(Box::new(b)))
                    }
                }
            }
        },
        (ua, Ty::TVar(_)) => {
            *b = Ty::TVar(Tvar::Link(Box::new(ua)))
        },
        (Ty::TArr(_, _), Ty::TArr(_, _)) => {
            let (a1, a2) = match a {
                Ty::TArr(a1, a2) => (&mut **a1, &mut **a2),
                _ => panic!("unification failed"),
            };
            let (b1, b2) = match b {
                Ty::TArr(b1, b2) => (&mut **b1, &mut **b2),
                _ => panic!("unification failed"),
            };
            unify(&mut *a1, &mut *a2);
            unify(&mut *b1, &mut *b2);
        },
    }
}

static mut tyvec: Vec<Ty> = vec![];

fn infer(env: &HashTrieMap<String, usize>, term: Term) -> Ty {
    match term {
        Term::Var(x) => Ty::TVar(Tvar::AVar(*env.get(&x).unwrap())),
        Term::Abs(x, t) => {
            let ax = fresh();
            let env2 = env.insert(x, ax);
            let b = infer(&env2, *t);
            Ty::TArr(Box::new(Ty::TVar(Tvar::AVar(ax))), Box::new(b))
        }
        Term::App(t, u) => {
            let iu = infer(env, *u);
            let b = fresh();
            let ab = Ty::TArr(Box::new(iu), Box::new(Ty::TVar(Tvar::AVar(b))));
            let it = infer(env, *t);
            unsafe {
                tyvec.push(it);
                tyvec.push(ab);
                let nn = tyvec.len();
                unify(&mut tyvec[nn-2], &mut tyvec[nn-1]);
            }
            Ty::TVar(Tvar::AVar(b))
        }
    }
}

fn main() {
    let unlinked = unlink(&Ty::TVar(Tvar::Link(Box::new(Ty::TVar(Tvar::AVar(0))))));
    assert!(unlinked == Ty::TVar(Tvar::AVar(0)));
    assert!(occurs(0, &unlinked));

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

    let env: HashTrieMap<String, usize> = HashTrieMap::new();
    let result = infer(&env, term);
    println!("{:?}", result);
}
