use crate::types::Formula;
use crate::types::and;
use crate::types::implies;
use crate::types::not;
use crate::types::or;
use std::collections::HashSet;
use Formula::*;

impl Formula {
    pub fn atoms(&self) -> HashSet<Formula> {
        let mut set: HashSet<Formula> = HashSet::new();
        match self {
            Atom(_) => {
                set.insert(self.clone());
            }
            Not(inner) => {
                set.extend(inner.atoms());
            }
            Or(lhs, rhs) | And(lhs, rhs) | Implies(lhs, rhs) => {
                set.extend(lhs.atoms());
                set.extend(rhs.atoms());
            }
        }
        return set;
    }
    
    pub fn literals(&self) -> HashSet<Formula> {
        if self.is_literal() {
            return HashSet::from([self.clone()])
        }
        match self {
            And(lhs, rhs) | Or(lhs, rhs) | Implies(lhs, rhs) => lhs.literals().union(&rhs.literals()).cloned().collect(),
            Not(inner) => inner.literals(),
            Atom(_) => HashSet::from([self.clone()])
        }
    }

    pub fn subformulas(&self) -> HashSet<Formula> {
        let mut set: HashSet<Formula> = HashSet::new();
        set.insert(self.clone());
        match self {
            Atom(_) => {}
            Not(inner) => {
                set.extend(inner.subformulas());
            }
            Or(lhs, rhs) | And(lhs, rhs) | Implies(lhs, rhs) => {
                set.extend(lhs.subformulas());
                set.extend(rhs.subformulas());
            }
        }
        return set;
    }
     pub fn replace(&self, old_subf: Formula, new_subf: Formula) -> Formula {
        if *self == old_subf {
            return new_subf;
        }
        match self {
            Atom(_) => self.clone(),
            Not(inner) => not(inner.replace(old_subf, new_subf)),
            Or(lhs, rhs) => or(
                lhs.replace(old_subf.clone(), new_subf.clone()),
                rhs.replace(old_subf, new_subf),
            ),
            And(lhs, rhs) => and(
                lhs.replace(old_subf.clone(), new_subf.clone()),
                rhs.replace(old_subf, new_subf),
            ),
            Implies(lhs, rhs) => implies(
                lhs.replace(old_subf.clone(), new_subf.clone()),
                rhs.replace(old_subf, new_subf),
            ),
        }
    }   
    pub fn is_literal(&self) -> bool {
        match self {
            Atom(_) => true,
            Not(inner) => matches!(**inner, Atom(_)),
            _ => false,
        }
    }

    pub fn is_clause(&self) -> bool {
        match self {
            Or(lhs, rhs) => lhs.is_clause() && rhs.is_clause(),
            _ => self.is_literal(),
        }
    }

    pub fn is_cnf(&self) -> bool {
        match self {
            And(lhs, rhs) => (lhs.is_clause() || lhs.is_cnf()) && (rhs.is_clause() || rhs.is_cnf()),
            _ => false,
        }
    }
}
