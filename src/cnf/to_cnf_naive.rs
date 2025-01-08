use crate::types::and;
use crate::types::implies;
use crate::types::not;
use crate::types::or;
use crate::types::Formula;
use Formula::*;

impl Formula {
    pub fn to_cnf_naive(&self) -> Formula {
        let mut formula = self.replace_implies();
        while formula.can_de_morgan() {
            formula = formula.de_morgan();
        }
        formula = formula.remove_double_neg();
        while !formula.is_cnf() {
            formula = formula.distribute();
        }
        formula
    }

    fn replace_implies(&self) -> Formula {
        match self {
            Implies(lhs, rhs) => or(not(lhs.replace_implies()), rhs.replace_implies()),
            Not(inner) => not(inner.replace_implies()),
            Or(lhs, rhs) => or(lhs.replace_implies(), rhs.replace_implies()),
            And(lhs, rhs) => and(lhs.replace_implies(), rhs.replace_implies()),
            Atom(_) => self.clone(),
        }
    }

    fn can_de_morgan(&self) -> bool {
        match self {
            Not(inner) => {
                match inner.as_ref() {
                    And(_, _) | Or(_, _) => true,
                    _ => false || inner.can_de_morgan()
                }
            },
            And(lhs, rhs) | Or(lhs, rhs) | Implies(lhs, rhs) => false || lhs.can_de_morgan() || rhs.can_de_morgan(),
            Atom(_) => false, 
        }
    }

    fn de_morgan(&self) -> Formula {
        match self {
            Not(inner) => match inner.as_ref() {
                And(lhs, rhs) => or(not(lhs.de_morgan()), not(rhs.de_morgan())),
                Or(lhs, rhs) => and(not(lhs.de_morgan()), not(rhs.de_morgan())),
                _ => not(inner.de_morgan()),
            },
            And(lhs, rhs) => and(lhs.de_morgan(), rhs.de_morgan()),
            Or(lhs, rhs) => or(lhs.de_morgan(), rhs.de_morgan()),
            Implies(lhs, rhs) => implies(lhs.de_morgan(), rhs.de_morgan()),
            Atom(_) => self.clone(),
        }
    }

    fn remove_double_neg(&self) -> Formula {
        match self {
            Not(inner) => match inner.as_ref() {
                Not(i) => i.remove_double_neg(),
                _ => not(inner.remove_double_neg()),
            },
            And(lhs, rhs) => and(lhs.remove_double_neg(), rhs.remove_double_neg()),
            Or(lhs, rhs) => or(lhs.remove_double_neg(), rhs.remove_double_neg()),
            Implies(lhs, rhs) => implies(lhs.remove_double_neg(), rhs.remove_double_neg()),
            Atom(_) => self.clone(),
        }
    }
    fn distribute(&self) -> Formula {
        match self {
            Or(lhs, rhs) => {
                match (lhs.as_ref(), rhs.as_ref()) {
                    (And(l, r), _) => {
                        let rd = rhs.distribute();
                        and(or(l.distribute(), rd.clone()), or(r.distribute(), rd))
                    }
                    (_, And(l, r)) => {
                        let ld = lhs.distribute();
                        and(or(ld.clone(), l.distribute()), or(ld, r.distribute()))
                    }
                    _ => or(lhs.distribute(), rhs.distribute()),
                }
            }
            Not(inner) => not(inner.distribute()),
            And(lhs, rhs) => and(lhs.distribute(), rhs.distribute()),
            Implies(lhs, rhs) => implies(lhs.distribute(), rhs.distribute()),
            Atom(_) => self.clone(),
        }
    }
}
