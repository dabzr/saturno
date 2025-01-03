use crate::and;
use crate::implies;
use crate::not;
use crate::or;
use crate::Formula;
use Formula::*;

impl Formula {
    pub fn to_cnf_naive(&self) -> Formula {
        self.replace_implies()
            .de_morgan()
            .remove_double_neg()
            .distribute()
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
            Or(lhs, rhs) => match lhs.as_ref() {
                And(l, r) => {
                    let rd = rhs.distribute();
                    and(or(l.distribute(), rd.clone()), or(r.distribute(), rd))
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
