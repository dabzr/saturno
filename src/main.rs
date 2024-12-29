mod types;
use types::*;
mod funcs;
mod to_cnf_naive;
mod cnf_to_list;
mod dpll;

fn main() {
    let f = and(atom("p"), atom("q"));
    println!("formula: {}", f.clone());
    println!("formula-cnf: {}", f.sat_dpll());
    let f2 = and(not(atom("p")), atom("p"));
    println!("formula 2: {}", f2.clone());
    println!("formula-cnf 2: {}", f2.sat_dpll());
}
