use crate::Formula;
use Formula::*;
use std::collections::HashSet;
use std::vec::Vec;
use std::rc::Rc;
use crate::not;

impl Formula {
    pub fn sat_dpll(&self) -> bool {
        let lst = 
            match self.cnf_to_list_naive() {
                Err(()) => self.to_cnf_naive().cnf_to_list_naive().unwrap(),
                Ok(v) => v
            };
        let item = lst
                    .last()
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap();
        item.sat_rec(lst.clone(), HashSet::new()) || item.negate().sat_rec(lst, HashSet::new())
    }
    fn sat_rec(&self, mut clauses: Vec<HashSet<Rc<Formula>>>, mut visited_literals: HashSet<Rc<Formula>>) -> bool {
        if visited_literals.contains(&self.negate()) {
            return false;
        }
        if clauses.is_empty() {
            return true;
        }
        visited_literals.insert(Rc::new(self.clone()));
        let last = clauses.last_mut().unwrap();
        if last.contains(self) {
            clauses.pop();
            return self.sat_rec(clauses, visited_literals); 
        }
        if last.contains(self.negate().as_ref()) {
            let nset = HashSet::from([self.negate()]);
            if *last == nset.clone() {
                return false;
            }
            last.remove(self.negate().as_ref());
        }
        let next_literal = last.iter().next().cloned().unwrap();
        return next_literal.sat_rec(clauses, visited_literals);
    }

    pub fn negate(&self) -> Rc<Formula> {
        match self {
            Not(inner) => Rc::clone(inner),
            _ => Rc::new(not(self.clone()))
        }
    }
}
