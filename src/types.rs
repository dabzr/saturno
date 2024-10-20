#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Formula {
    Atom(String),
    Not(Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
}

use std::fmt;
use Formula::*;

impl Formula {
    pub fn to_string(&self) -> String {
        match self {
            Atom(s) => s.clone(),
            Not(inner) => format!("(¬{})", inner),
            Implies(lhs, rhs) => format!("({} → {})", lhs, rhs),
            Or(lhs, rhs) => format!("({} ∨ {})", lhs, rhs),
            And(lhs, rhs) => format!("({} ∧ {})", lhs, rhs),
        }
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn atom(s: &str) -> Formula {
    Atom(s.to_string())
}
pub fn not(formula: Formula) -> Formula {
    Not(Box::new(formula))
}
pub fn or(lhs: Formula, rhs: Formula) -> Formula {
    Or(Box::new(lhs), Box::new(rhs))
}
pub fn and(lhs: Formula, rhs: Formula) -> Formula {
    And(Box::new(lhs), Box::new(rhs))
}
pub fn implies(lhs: Formula, rhs: Formula) -> Formula {
    Implies(Box::new(lhs), Box::new(rhs))
}
