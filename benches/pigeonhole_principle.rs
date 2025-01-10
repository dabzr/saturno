use criterion::{criterion_group, criterion_main, Criterion};
use saturno::types::Formula;
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

fn bench_dpll(c: &mut Criterion) {
    let formula = create_php_formula(3);
    c.bench_function("dpll", |b| {
        b.iter(|| {
            formula.sat_dpll();
        });
    });
}

criterion_group!(benches, bench_dpll);
criterion_main!(benches);
