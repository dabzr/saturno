mod types;
use types::*;
mod cnf_to_list;
mod dpll;
mod funcs;
mod to_cnf_naive;

fn main() {
    let f = and(atom("p"), atom("q"));
    println!("is {} satisfiable? {}", f.clone(), f.sat_dpll());
    let f2 = and(not(atom("p")), atom("p"));
    println!("is {} satisfiable? {}", f2.clone(), f2.sat_dpll());
}
