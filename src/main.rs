mod types;
use types::*;
mod funcs;

fn main() {
    let formula1 = not(atom("B"));
    let formula2 = not(atom("B"));
    let formula3 = and(atom("A"), atom("B"));
    let formula4 = or(atom("C"), atom("D"));
    let formula5 = implies(atom("A"), not(atom("B")));
    println!("formula1 == formula2: {}", formula1 == formula2);
    println!("formula1 == formula3: {}", formula1 == formula3);
    
    println!("formula1: {}", formula1.to_str());
    println!("formula2: {}", formula2.to_str());    
    println!("formula3: {}", formula3.to_str());
    println!("formula4: {}", formula4.to_str());
    println!("formula5: {}", formula5.to_str());    
    
    println!("Tamanho da formula 1: {} ", formula1.length());
    println!("Tamanho da formula 2: {} ", formula2.length());
    println!("Tamanho da formula 3: {} ", formula3.length());
    println!("Tamanho da formula 4: {} ", formula4.length());
    println!("Tamanho da formula 5: {} ", formula5.length());
}
