#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct BinaryOperator {
    pub lhs: Formula,
    pub rhs: Formula
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Formula {
    Atom(String),
    Not(Box<Formula>),
    Or(Box<BinaryOperator>),
    And(Box<BinaryOperator>),
    Implies(Box<BinaryOperator>)
}

impl Formula {
    pub fn to_str(&self) -> String {
        match self {
            Formula::Atom(s) => s.clone(),
            Formula::Not(subformula) => format!("(¬{})", subformula.to_str()),
            Formula::Implies(bop) => Self::to_str_bop(*bop.clone(), " → "),
            Formula::Or(bop) => Self::to_str_bop(*bop.clone(), " ∨ "),
            Formula::And(bop) => Self::to_str_bop(*bop.clone(), " ∧ "),
        }
    }

    fn to_str_bop(bop: BinaryOperator, op: &str) -> String { format!("({}{}{})", bop.lhs.to_str(), op, bop.rhs.to_str()) }
}


pub fn atom(s: &str) -> Formula { Formula::Atom(s.to_string()) }
pub fn not(formula: Formula) -> Formula { Formula::Not(Box::new(formula)) }
pub fn or(lhs: Formula, rhs: Formula) -> Formula { Formula::Or(binop(lhs, rhs)) }
pub fn and(lhs: Formula, rhs: Formula) -> Formula { Formula::And(binop(lhs, rhs)) }
pub fn implies(lhs: Formula, rhs: Formula) -> Formula { Formula::Implies(binop(lhs, rhs)) }

fn binop(lhs: Formula, rhs:Formula) -> Box<BinaryOperator> { Box::new(BinaryOperator {lhs, rhs}) }


