use crate::and;
use crate::implies;
use crate::not;
use crate::or;
use crate::Formula;
use std::collections::HashMap;
use Formula::*;
type Interpretation = HashMap<String, bool>;

impl Formula {
    pub fn evaluate(&self, interpretation: &Interpretation) -> bool{
        match self {
            Atom(s) => {
                match interpretation.get(s) {
                    Some(x) => *x,
                    None => false
                }
            },
            Not(inner) =>  !inner.evaluate(interpretation),
            Or(lhs, rhs) => lhs.evaluate(interpretation) || rhs.evaluate(interpretation),
            And(lhs, rhs) => lhs.evaluate(interpretation) && rhs.evaluate(interpretation),
            Implies(lhs, rhs) => !lhs.evaluate(interpretation) || rhs.evaluate(interpretation)
        }
    }
    pub fn create_truth_table() {

    }
}
