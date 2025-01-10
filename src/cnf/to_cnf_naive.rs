use crate::types::and;
use crate::types::implies;
use crate::types::not;
use crate::types::or;
use crate::types::Formula;
use Formula::*;

impl Formula {
    pub fn to_cnf_naive(&self) -> Formula {
        let mut formula = self.replace_implies().de_morgan().remove_double_neg();
        while !formula.is_cnf() {
            formula = formula.distribute();
        }
        formula
    }

    fn distribute(&self) -> Formula {
        match self {
            Or(lhs, rhs) => match (lhs.as_ref(), rhs.as_ref()) {
                (And(l, r), _) => {
                    let rd = rhs.distribute();
                    and(or(l.distribute(), rd.clone()), or(r.distribute(), rd))
                }
                (_, And(l, r)) => {
                    let ld = lhs.distribute();
                    and(or(ld.clone(), l.distribute()), or(ld, r.distribute()))
                }
                _ => or(lhs.distribute(), rhs.distribute()),
            },
            Not(inner) => not(inner.distribute()),
            And(lhs, rhs) => and(lhs.distribute(), rhs.distribute()),
            Implies(lhs, rhs) => implies(lhs.distribute(), rhs.distribute()),
            Atom(_) => self.clone(),
        }
    }
}
