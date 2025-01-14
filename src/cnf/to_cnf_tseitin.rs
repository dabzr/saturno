use crate::types::and;
use crate::types::not;
use crate::types::or;
use crate::types::Formula;
use Formula::*;
use rand::Rng;
use std::collections::HashSet;

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789"; 
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    random_string
}

impl Formula {
    pub fn to_cnf_tseitin(&self) -> Formula {
        self
            .replace_implies()
            .de_morgan()
            .remove_double_neg()
            .transform()
    }

    fn transform(&self) -> Formula {
        let subs: HashSet<Formula> = self
            .subformulas()
            .iter()
            .filter(|x| !matches!(x, Formula::Atom { .. })) 
            .cloned()  
            .collect();
        let mut count = 0;
        let hash = generate_random_string(16);
        let mut formula = self.clone();
        for subform in &subs {
            let val = Atom(format!("{}_{}", hash, count));
            count+=1;
            let lits = subform.literals();
            let ors = or(not(val.clone()), lits.clone().into_iter().reduce(|acc, x| or(acc, x)).unwrap());
            formula = if count != 0 {and(formula, ors)} else {ors};
            for literal in &lits {
                formula = and(formula, or(not(literal.clone()), val.clone()))
            }
        }
        formula
    }
}
