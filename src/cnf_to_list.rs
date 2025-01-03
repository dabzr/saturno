use crate::Formula;
use std::collections::HashSet;
use std::rc::Rc;
use std::vec::Vec;
use Formula::*;

impl Formula {
    pub fn cnf_to_list_naive(&self) -> Result<Vec<HashSet<Rc<Formula>>>, ()> {
        if !self.is_cnf() {
            return Err(());
        }
        Ok(self.get_all_clauses().get_all_literals())
    }
    fn get_all_clauses(&self) -> HashSet<Rc<Formula>> {
        fn acc_clauses(f: Rc<Formula>, mut s: HashSet<Rc<Formula>>) -> HashSet<Rc<Formula>> {
            match &*f {
                And(lhs, rhs) => {
                    s.insert(lhs.clone());
                    acc_clauses(rhs.clone(), s)
                }
                Atom(_) | Or(_, _) => {
                    s.insert(f);
                    s
                }
                _ => s,
            }
        }
        let set = HashSet::new();
        acc_clauses(Rc::new(self.clone()), set)
    }
}

trait FormulaSetExt {
    fn get_all_literals(&self) -> Vec<HashSet<Rc<Formula>>>;
}

impl FormulaSetExt for HashSet<Rc<Formula>> {
    fn get_all_literals(&self) -> Vec<HashSet<Rc<Formula>>> {
        fn literals_in_clause(f: Rc<Formula>, mut s: HashSet<Rc<Formula>>) -> HashSet<Rc<Formula>> {
            match &*f {
                Or(lhs, rhs) => {
                    s.insert(lhs.clone());
                    literals_in_clause(rhs.clone(), s)
                }
                Atom(_) | Not(_) => {
                    s.insert(f);
                    s
                }
                _ => s,
            }
        }
        self.iter()
            .map(|x| literals_in_clause(x.clone(), HashSet::new()))
            .filter(|x| *x != HashSet::new())
            .collect()
    }
}
