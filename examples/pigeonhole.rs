use saturno::types::*;

fn create_php_formula(n: usize) -> Formula {
    let mut gamma = atom("neutral1");
    let mut delta = atom("neutral2"); 

    for i in 1..=n + 1 {
        let clause = (1..=n)
            .map(|j| atom(&format!("p{}_{}", i, j)))
            .reduce(or)
            .unwrap(); 
        gamma = and(gamma, clause);
    }

    for i in 1..=n + 1 { 
        for k in (i + 1)..=n + 1 {
            for j in 1..=n { 
                let clause = and(
                    atom(&format!("p{}_{}", i, j)),
                    atom(&format!("p{}_{}", k, j)),
                );
                delta = or(delta, clause);
            }
        }
    }
    implies(gamma, delta)
}

fn main() {
    let formula = create_php_formula(1);
    println!("{}", formula.clone());
    println!("{}", formula.to_cnf_tseitin());
    println!("{}", formula.sat_dpll_tseitin())
}
