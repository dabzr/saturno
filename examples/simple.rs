use saturno::types::*;

fn main() {
    let formula = implies(and(atom("A"), atom("B")), or(atom("C"), not(atom("D"))));
    let cnf = formula.to_cnf_naive();
    println!("{}", cnf);
}
