use crate::types::not;
use crate::types::Formula;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::vec::Vec;
use Formula::*;

type Clauses = Vec<HashSet<Rc<Formula>>>;

impl Formula {
    pub fn sat_dpll(&self) -> bool {
        let lst = match self.cnf_to_list() {
            Err(()) => self.to_cnf_naive().cnf_to_list().unwrap(),
            Ok(v) => v,
        };
        lst.unit_propagate().pure_literal_elimination().dpll()
    }

    pub fn negate(&self) -> Rc<Formula> {
        match self {
            Not(inner) => Rc::clone(inner),
            _ => Rc::new(not(self.clone())),
        }
    }
}

trait DPLLSolve {
    fn dpll(&self) -> bool;
}

trait DPLLHeuristics {
    fn unit_propagate(&self) -> Clauses;
    fn pure_literal_elimination(&self) -> Clauses;
    fn choose_literal(&self) -> Rc<Formula>;
}

impl DPLLSolve for Clauses {
    fn dpll(&self) -> bool {
        if self.is_empty() {
            return true;
        }
        if self.iter().any(|x| x.is_empty()) {
            return false;
        }
        let literal = self.choose_literal();
        let clauses_left: Clauses = self
            .iter()
            .filter(|clause| !clause.contains(&literal))
            .cloned()
            .collect();
        let clauses_right: Clauses = self
            .iter()
            .filter(|clause| !clause.contains(&literal.negate()))
            .cloned()
            .collect();
        return clauses_left.dpll() || clauses_right.dpll();
    }
}

impl DPLLHeuristics for Clauses {
    fn unit_propagate(&self) -> Clauses {
        let mut clauses = self.clone();
        loop {
            let unit_clauses: Clauses = clauses
                .iter()
                .filter(|&clause| clause.len() == 1)
                .cloned()
                .collect();
            if unit_clauses.is_empty() {
                break;
            }
            let unit_literal: Rc<Formula> = unit_clauses[0].iter().next().unwrap().clone();
            clauses = clauses
                .into_iter()
                .filter(|clause| !clause.contains(unit_literal.as_ref()))
                .map(|mut clause| {
                    clause.remove(unit_literal.negate().as_ref());
                    clause
                })
                .collect();
        }
        clauses
    }
    fn pure_literal_elimination(&self) -> Clauses {
        let mut clauses = self.clone();

        let all_literals: HashSet<_> = clauses
            .iter()
            .flat_map(|clause| clause.iter())
            .cloned()
            .collect();

        let pure_literals: HashSet<_> = all_literals
            .iter()
            .filter(|literal| !all_literals.contains(literal.negate().as_ref()))
            .cloned()
            .collect();

        for pure_literal in pure_literals {
            clauses = clauses
                .into_iter()
                .filter(|clause| !clause.contains(&pure_literal))
                .collect();
        }
        clauses
    }
    fn choose_literal(&self) -> Rc<Formula> {
        let mut literal_counts = HashMap::new();
        for clause in self {
            for literal in clause {
                *literal_counts.entry(literal).or_insert(0) += 1;
            }
        }
        literal_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(literal, _)| literal)
            .expect("No literals found")
            .clone()
    }
}
