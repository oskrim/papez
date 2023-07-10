#[derive(Debug)]
enum Formula {
    Var(String),
    Implication {
        left: Box<Formula>,
        right: Box<Formula>,
    }
}

fn split_implication(formula: Formula) -> (Vec<Formula>, Formula) {
    match formula {
        Formula::Var(x) => (Vec::new(), Formula::Var(x)),
        Formula::Implication { left, right } => {
            let (args, tgt) = split_implication(*right);
            let mut left_args = vec![*left];
            left_args.extend(args);
            (left_args, tgt)
        }
    }
}

// fn provable(f: Formula, )
//     fn inner(f: Formula, seen: Vec::) -> bool {
//         match f {
//             Formula::Var(x) => false,
//             Formula::Implication { left, right } => {
//                 let (args, tgt) = split_implication(*right);
//                 let mut left_args = vec![*left];
//                 left_args.extend(args);
//                 (left_args, tgt)
//             }
//         }
//     }

fn main() {
    let x = Formula::Var("a".to_string());
    let y = Formula::Var("b".to_string());
    let z = Formula::Implication { left: Box::new(x), right: Box::new(y) };
    println!("z {:?}", z);

    let (args, tgt) = split_implication(z);
    println!("args {:?} tgt {:?}", args, tgt);


}
