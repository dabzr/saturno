use crate::and;
use crate::implies;
use crate::not;
use crate::or;
use crate::Formula;
use Formula::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

impl Formula {
    pub fn to_set_naive(&self) -> Result<HashSet<HashSet<Formula>>, ()> {
        if !self.is_cnf() {
            return Error()
        }
        Ok()
    }
    fn get_all_clauses(&self) -> HashSet<Formula> {
        fn acc_clauses(f: Formula, mut s: HashSet<Formula>) -> HashSet<Formula> {
            match f {
                And(lhs, rhs) => {
                    s.insert(*lhs.as_ref());
                    acc_clauses(*rhs.as_ref(), s)
                },
                Atom(_) | Or(_, _) => {
                    s.insert(f);
                    s
                },
                _ => s
            }
        }
        let mut set = HashSet::new();
        acc_clauses(*self, set)
    }
}

trait FormulaSetExt {
    fn get_all_literals(&self) -> HashSet<HashSet<Formula>>;
}

impl FormulaSetExt for HashSet<Formula> {
    fn get_all_literals(&self) -> HashSet<HashSet<Formula>> {
        fn literals_in_clause(f: Formula, mut s: HashSet<Formula>) -> HashSet<Formula> {
            match f {
                Or(lhs, rhs) => {
                    s.insert(*lhs.as_ref());
                    literals_in_clause(*rhs.as_ref(), s)
                }
                Atom(_) => {
                    s.insert(f);
                    s
                }
                _ => s
            }
        }
        self
            .iter()
            .map(|&x| literals_in_clause(x, HashSet::new()))
            .collect()
    }
}


#[derive(Debug)]
struct SetWrapper<T>(HashSet<T>);

impl<T> SetWrapper<T> {
    fn new() -> Self {
        SetWrapper(HashSet::new())
    }

    fn insert(&mut self, value: T) {
        self.0.insert(value);
    }
}

impl<T> PartialEq for SetWrapper<T> 
where
    T: Eq, // Ensure that the items inside the set are comparable
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for SetWrapper<T> where T: Eq {}

impl<T> Hash for SetWrapper<T> 
where
    T: Hash, // Ensure that the items inside the set are hashable
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in &self.0 {
            item.hash(state);
        }
    }
}

