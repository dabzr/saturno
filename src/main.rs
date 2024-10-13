mod types;
use types::*;
mod funcs;

fn main() {
    let formula1 = atom("p");  // p
    let formula2 = atom("q");  // q
    let formula3 = and(formula1.clone(), formula2.clone());  // (p /\ q)
    let formula4 = and(atom("p"), atom("s"));  // (p /\ s)
    let formula5 = not(and(atom("p"), atom("s")));  // (¬(p /\ s))
    let formula6 = or(not(and(atom("p"), atom("s"))), atom("q"));  // ((¬(p /\ s)) v q)
    let formula7 = implies(not(and(atom("p"), atom("s"))), and(atom("q"), atom("r")));  // ((¬(p /\ s)) -> (q /\ r))
    let formula8 = implies(not(and(atom("p"), atom("s"))), and(atom("q"), not(and(atom("p"), atom("s")))));
    // ((¬(p /\ s)) -> (q /\ (¬(p /\ s))))

    let formula9 = not(not(atom("q")));
    println!("formula1 == formula3: {}", formula1 == formula3);
    println!("formula1 == formula2: {}", formula1 == formula2);
    println!("formula3 == (p ∧ q) {}", formula3 == and(atom("p"), atom("q")));

    println!("formula1: {}", formula1.to_str());
    println!("formula2: {}", formula2.to_str());
    println!("formula3: {}", formula3.to_str());
    println!("formula4: {}", formula4.to_str());
    println!("formula5: {}", formula5.to_str());
    println!("formula6: {}", formula6.to_str());
    println!("formula7: {}", formula7.to_str());
    println!("formula8: {}", formula8.to_str());
    println!("formula9: {}", formula9.to_str());

    println!("length of formula1: {}", formula1.length());
    println!("length of formula3: {}", formula3.length());

    println!("length of formula7: {}", formula7.length());
    println!("subformulas of formula7: {:?}", formula7.subformulas());

    for subformula in formula7.subformulas() {
        print!("{:?} ", subformula);
    }

    println!("");

    println!("length of formula8: {}", formula8.length());
    print!("subformulas of formula8: ");
    for subformula in formula8.subformulas() {
        print!("{:?} ", subformula);
    }
    println!("");
    //  we have shown in class that for all formula A, len(subformulas(A)) <= length(A):
    // for example, for formula8:
    println!("number of subformulas of formula8: {}", formula8.subformulas().len());
    println!("formula8.subformulas().len() <= formula8.length(): {}", formula8.subformulas().len() <= formula8.length())
}
