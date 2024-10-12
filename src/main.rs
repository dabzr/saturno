mod types;
use types::*;

fn main() {
    let formula1 = not(atom("B"));
    let formula2 = not(atom("B"));
    let formula4 = and(atom("A"), atom("B"));
    let formula5 = or(atom("C"), atom("D"));
    let formula6 = implies(atom("A"), not(atom("B")));
    println!("formula1 == formula2: {}", formula1 == formula2);
    println!("formula1 == formula4: {}", formula1 == formula4);
    println!("Avaliação de formula4: {}", to_str(formula4));
    println!("Avaliação de formula5: {}", to_str(formula5));
    println!("Avaliação de formula6: {}", to_str(formula6)); 
}
