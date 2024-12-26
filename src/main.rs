mod types;
use types::*;
mod funcs;
mod to_cnf_naive;

fn main() {
    let f = or(implies(atom("p"), atom("q")), atom("p"));
    println!("formula: {}", f.clone());
    println!("formula-cnf: {}", f.to_cnf_naive());
    let f2 = implies(or(atom("p"), atom("q")), not(atom("q")));
    println!("formula 2: {}", f2.clone());
    println!("formula-cnf 2: {}", f2.to_cnf_naive());
}
