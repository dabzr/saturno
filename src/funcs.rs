use crate::Formula;
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
