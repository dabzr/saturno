use saturno::types::*;

fn main () {
    let formula = not(or(atom("p"), not(or(atom("q"), atom("s")))));
    let formula1 = or(and(atom("p"), or(atom("q"), and(atom("r"), atom("s")))), atom("t"));
    println!("{}", formula.to_cnf_naive());
    println!("{}", formula.to_cnf_naive().is_cnf());
    println!("{}", formula1.to_cnf_naive());
    println!("{}", formula1.to_cnf_naive().is_cnf());
}
