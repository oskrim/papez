#[derive(Debug)]
enum Ty {
    TVar(usize),
    TArr(Box<Ty>, Box<Ty>),
}

fn main() {
    let mut n: usize = 0;
    let mut fresh = || {
        n += 1;
        Ty::TVar(n - 1)
    };

    let x = fresh();
    let y = fresh();
    let z = Ty::TArr(Box::new(x), Box::new(y));
    println!("z {:?}", z);
}
