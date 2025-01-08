use saturno::types::*;

fn create_php_formula(n: usize) -> Formula {
    let mut gamma = atom("neutral1");
    let mut delta = atom("neutral2");
    for i in 1..=n + 1 {
        let clause = (1..=n)
            .map(|j| atom(&format!("p{}_{}", i, j))) 
            .reduce(|acc, lit| or(acc, lit)) 
            .unwrap();
        gamma = and(gamma, clause);
    }

    for j in 1..=n {
        for i in 1..=n + 1 {
            for k in i + 1..=n + 1 {
                let clause = or(
                    not(atom(&format!("p{}_{}", i, j))),
                    not(atom(&format!("p{}_{}", k, j))),
                );
                delta = and(delta, clause); 
            }
        }
    }

    implies(gamma, delta)
}

fn main() {
    let n = 3; 
    let formula = create_php_formula(n);
    let f_sat = formula.sat_dpll();
    println!("A fórmula é satisfatível? {}", f_sat);
}
