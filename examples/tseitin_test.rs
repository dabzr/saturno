use saturno::types::*;

fn main() {
    let formula = not(atom("p"));
    println!("{}", formula.to_cnf_tseitin());
}
