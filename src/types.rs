use std::fmt;
use std::rc::Rc;
use Formula::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Formula {
    Atom(String),
    Not(Rc<Formula>),
    Or(Rc<Formula>, Rc<Formula>),
    And(Rc<Formula>, Rc<Formula>),
    Implies(Rc<Formula>, Rc<Formula>),
}

// pub type Interpretation = HashMap<Formula, bool>;

// pub enum SAT {
//     Satisfiable(Interpretation),
//     Unsatisfiable()
// }

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
    Not(Rc::new(formula))
}
pub fn or(lhs: Formula, rhs: Formula) -> Formula {
    Or(Rc::new(lhs), Rc::new(rhs))
}
pub fn and(lhs: Formula, rhs: Formula) -> Formula {
    And(Rc::new(lhs), Rc::new(rhs))
}
pub fn implies(lhs: Formula, rhs: Formula) -> Formula {
    Implies(Rc::new(lhs), Rc::new(rhs))
}
