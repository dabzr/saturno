mod types;
use types::*;
mod funcs;
mod semantics;

fn main() {
    let formula1 = atom("p"); // p
    let formula2 = atom("q"); // q
    let formula3 = and(formula1.clone(), formula2.clone()); // (p /\ q)
    let formula4 = and(atom("p"), atom("s")); // (p /\ s)
    let formula5 = not(and(atom("p"), atom("s"))); // (¬(p /\ s))
    let formula6 = or(not(and(atom("p"), atom("s"))), atom("q")); // ((¬(p /\ s)) v q)
    let formula7 = implies(not(and(atom("p"), atom("s"))), and(atom("q"), atom("r"))); // ((¬(p /\ s)) -> (q /\ r))
    let formula8 = implies(
        not(and(atom("p"), atom("s"))),
        and(atom("q"), not(and(atom("p"), atom("s")))),
    );
    // ((¬(p /\ s)) -> (q /\ (¬(p /\ s))))

    let formula9 = not(not(atom("q")));
    println!("formula1 == formula3: {}", formula1 == formula3);
    println!("formula1 == formula2: {}", formula1 == formula2);
    println!(
        "formula3 == (p ∧ q) {}",
        formula3 == and(atom("p"), atom("q"))
    );

    println!("formula1: {}", formula1);
    println!("formula2: {}", formula2);
    println!("formula3: {}", formula3);
    println!("formula4: {}", formula4);
    println!("formula5: {}", formula5);
    println!("formula6: {}", formula6);
    println!("formula7: {}", formula7);
    println!("formula8: {}", formula8);
    println!("formula9: {}", formula9);

    println!("length of formula1: {}", formula1.length());
    println!("length of formula3: {}", formula3.length());

    println!("length of formula7: {}", formula7.length());
    println!("subformulas of formula7: {:?}", formula7.subformulas());

    for subformula in formula7.subformulas() {
        print!("{}, ", subformula);
    }

    println!("");

    println!("length of formula8: {}", formula8.length());
    print!("subformulas of formula8: ");
    for subformula in formula8.subformulas() {
        print!("{}, ", subformula);
    }
    println!("");
    //  we have shown in class that for all formula A, len(subformulas(A)) <= length(A):
    // for example, for formula8:
    println!(
        "number of subformulas of formula8: {}",
        formula8.subformulas().len()
    );
    println!(
        "formula8.subformulas().len() <= formula8.length(): {}",
        formula8.subformulas().len() <= formula8.length()
    );
    print!("Atoms of formula8: ");
    for atom in formula8.atoms() {
        print!("{} ", atom);
    }
    println!("");
    println!(
        "Number of atoms of formula8 is: {}",
        formula8.number_of_atoms()
    );
    println!(
        "Number of connectives of formula8 is: {}",
        formula8.number_of_connectives()
    );
    println!("formula8 is literal: {}", formula8.is_literal());
    println!("formula1 is literal: {}", formula1.is_literal());
    println!("formula9 is literal: {}", formula9.is_literal());
    println!("formula5 is literal: {}", formula5.is_literal());

    let old_subformula = not(and(atom("p"), atom("s")));
    let new_subformula = and(atom("p"), atom("q"));
    println!("formula8: {}", formula8);
    println!("old_subformula: {}", old_subformula);
    println!("new_subformula: {}", new_subformula);

    let formula8_clone = formula8.clone();
    let new_formula8 = formula8_clone.replace(old_subformula, new_subformula);
    println!("new formula8: {}", new_formula8);

    let clause = or(atom("p"), or(atom("q"), not(atom("s")))).is_clause();
    println!("is_clause(p ∨ q ∨ ¬s): {}", clause);

    let clause = or(atom("p"), and(atom("q"), atom("s"))).is_clause();
    println!("is_clause(p ∨ q ∧ s): {}", clause);

    let clause = not(atom("p")).is_clause();
    println!("is_clause(¬p): {}", clause);

    println!("is formula8 is negation normal form? {}", formula8.is_nnf());
    let nnf = or(atom("p"), or(atom("q"), not(atom("s")))).is_nnf();
    println!("is_nnf(p ∨ q ∨ ¬s): {}", nnf);

    let cnf = and(
        not(atom("p")),
        and(or(atom("p"), not(atom("q"))), or(atom("s"), atom("p"))),
    );
    println!("is_cnf({}): {}", cnf, cnf.is_cnf());

    let cnf = or(
        not(atom("p")),
        and(or(atom("p"), not(atom("q"))), and(atom("s"), atom("p"))),
    );
    println!("is_cnf({}): {}", cnf, cnf.is_cnf());

    let dnnf = or(
        and(or(atom("a"), not(atom("b"))), or(atom("c"), atom("d"))),
        and(or(atom("a"), atom("b")), or(not(atom("c")), not(atom("d")))),
    );

    println!("is_dnnf({}): {}", dnnf, dnnf.is_dnnf());
}
