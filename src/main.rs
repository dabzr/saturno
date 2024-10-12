mod types;
use types::*;
mod funcs;
use funcs::*;
fn main() {
    let formula1 = not(atom("B"));
    let formula2 = not(atom("B"));
    let formula3 = and(atom("A"), atom("B"));
    let formula4 = or(atom("C"), atom("D"));
    let formula5 = implies(atom("A"), not(atom("B")));
    println!("formula1 == formula2: {}", formula1 == formula2);
    println!("formula1 == formula3: {}", formula1 == formula3);
    
    println!("formula1: {}", to_str(formula1.clone()));
    println!("formula2: {}", to_str(formula2.clone()));    
    println!("formula3: {}", to_str(formula3.clone()));
    println!("formula4: {}", to_str(formula4.clone()));
    println!("formula5: {}", to_str(formula5.clone()));    
    
    println!("Tamanho da formula 1: {} ", length(formula1.clone()));
    println!("Tamanho da formula 2: {} ", length(formula2.clone()));
    println!("Tamanho da formula 3: {} ", length(formula3.clone()));
    println!("Tamanho da formula 4: {} ", length(formula4.clone()));
    println!("Tamanho da formula 5: {} ", length(formula5.clone()));
}
