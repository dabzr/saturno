use criterion::{criterion_group, criterion_main, Criterion};
use saturno::types::Formula;
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

fn create_php_formula_cnf(n: usize) -> Formula {
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


fn bench_dpll(c: &mut Criterion) {
    let formula = create_php_formula_cnf(3);
    c.bench_function("dpll", |b| {
        b.iter(|| {
            formula.sat_dpll_naive();
        });
    });
    let formula2 = create_php_formula(2);
    c.bench_function("cnf naive", |b| {
        b.iter(|| {formula2.to_cnf_naive()})

    });
    c.bench_function("cnf tseitin", |b| {
        b.iter(|| {formula2.to_cnf_tseitin()})

    });
}

criterion_group!(benches, bench_dpll);
criterion_main!(benches);
