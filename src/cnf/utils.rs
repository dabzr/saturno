use crate::types::and;
use crate::types::implies;
use crate::types::not;
use crate::types::or;
use crate::types::Formula;
use Formula::*;

impl Formula {
    pub fn replace_implies(&self) -> Formula {
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
            Not(inner) => match inner.as_ref() {
                And(_, _) | Or(_, _) => true,
                _ => false || inner.can_de_morgan(),
            },
            And(lhs, rhs) | Or(lhs, rhs) | Implies(lhs, rhs) => {
                false || lhs.can_de_morgan() || rhs.can_de_morgan()
            }
            Atom(_) => false,
        }
    }
    pub fn de_morgan(&self) -> Formula {
        let mut formula = self.clone();
        while formula.can_de_morgan() {
            formula = formula.de_morgan_once()
        }
        formula
    }
    fn de_morgan_once(&self) -> Formula {
        match self {
            Not(inner) => match inner.as_ref() {
                And(lhs, rhs) => or(not(lhs.de_morgan_once()), not(rhs.de_morgan_once())),
                Or(lhs, rhs) => and(not(lhs.de_morgan_once()), not(rhs.de_morgan_once())),
                _ => not(inner.de_morgan_once()),
            },
            And(lhs, rhs) => and(lhs.de_morgan_once(), rhs.de_morgan_once()),
            Or(lhs, rhs) => or(lhs.de_morgan_once(), rhs.de_morgan_once()),
            Implies(lhs, rhs) => implies(lhs.de_morgan_once(), rhs.de_morgan_once()),
            Atom(_) => self.clone(),
        }
    }

    pub fn remove_double_neg(&self) -> Formula {
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
}
