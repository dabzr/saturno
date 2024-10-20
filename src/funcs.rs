use crate::and;
use crate::implies;
use crate::not;
use crate::or;
use crate::Formula;
use std::collections::HashSet;
use Formula::*;

impl Formula {
    pub fn length(&self) -> usize {
        match self {
            Atom(_) => 1,
            Not(inner) => inner.length() + 1,
            Or(lhs, rhs) | And(lhs, rhs) | Implies(lhs, rhs) => lhs.length() + rhs.length() + 1,
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

    pub fn number_of_atoms(&self) -> i32 {
        match self {
            Atom(_) => 1,
            Not(inner) => inner.number_of_atoms(),
            Or(lhs, rhs) | And(lhs, rhs) | Implies(lhs, rhs) => {
                lhs.number_of_atoms() + rhs.number_of_atoms()
            }
        }
    }
    pub fn number_of_connectives(&self) -> i32 {
        match self {
            Atom(_) => 0,
            Not(inner) => inner.number_of_connectives() + 1,
            Or(lhs, rhs) | And(lhs, rhs) | Implies(lhs, rhs) => {
                lhs.number_of_connectives() + rhs.number_of_connectives() + 1
            }
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Atom(_) => true,
            Not(inner) => matches!(**inner, Atom(_)),
            _ => false,
        }
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

    pub fn is_clause(&self) -> bool {
        match self {
            Or(lhs, rhs) => lhs.is_clause() && rhs.is_clause(),
            _ => self.is_literal(),
        }
    }

    pub fn is_nnf(&self) -> bool {
        match self {
            Atom(_) => true,
            Not(inner) => inner.is_literal(),
            Or(lhs, rhs) | And(lhs, rhs) => lhs.is_nnf() && rhs.is_nnf(),
            Implies(_, _) => false,
        }
    }

    pub fn is_cnf(&self) -> bool {
        match self {
            And(lhs, rhs) => (lhs.is_clause() || lhs.is_cnf()) && (rhs.is_clause() || rhs.is_cnf()),
            _ => false,
        }
    }

    pub fn is_term(&self) -> bool {
        match self {
            And(lhs, rhs) => lhs.is_clause() && rhs.is_clause(),
            _ => self.is_literal(),
        }
    }

    pub fn is_dnf(&self) -> bool {
        match self {
            Or(lhs, rhs) => (lhs.is_term() || lhs.is_dnf()) && (rhs.is_term() || rhs.is_dnf()),
            _ => false,
        }
    }
    pub fn is_dnnf(&self) -> bool {
        if !self.is_nnf() {
            return false;
        }
        match self {
            And(lhs, rhs) => {
                if lhs.atoms().intersection(&rhs.atoms()).count() == 0 {
                    return lhs.is_dnnf() && rhs.is_dnnf();
                }
                return false;
            }
            Or(lhs, rhs) => lhs.is_dnnf() && rhs.is_dnnf(),
            _ => true,
        }
    }
}
