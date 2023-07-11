use std::sync::Mutex;
use rpds::HashTrieMap;

enum Expr {
    Var(String),
    Abs(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Pi(String, Box<Expr>, Box<Expr>),
    Type(usize),
    Nat,
    Zero,
    Succ(Box<Expr>),
    Ind(Box<Expr>, Box<Expr>, Box<Expr>),
    Id(Box<Expr>, Box<Expr>, Box<Expr>),
    Refl(Box<Expr>),
    J(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
}

fn main() {
}
