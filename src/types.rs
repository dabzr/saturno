#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperator {
    pub lhs: Formula,
    pub rhs: Formula
}

#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    Atom(String),
    Not(Box<Formula>),
    Or(Box<BinaryOperator>),
    And(Box<BinaryOperator>),
    Implies(Box<BinaryOperator>)
}


pub fn atom(s: &str) -> Formula {
    Formula::Atom(s.to_string())
}
pub fn not(formula: Formula) -> Formula {
    Formula::Not(Box::new(formula))
}
pub fn or(lhs: Formula, rhs: Formula) -> Formula {
    Formula::Or(binop(lhs, rhs))
}
pub fn and(lhs: Formula, rhs: Formula) -> Formula {
    Formula::And(binop(lhs, rhs))
}
pub fn implies(lhs: Formula, rhs: Formula) -> Formula {
    Formula::Implies(binop(lhs, rhs))
}

fn binop(lhs: Formula, rhs:Formula) -> Box<BinaryOperator> {
    Box::new(BinaryOperator {lhs, rhs})
}

pub fn to_str(formula: Formula) -> String {
    match formula {
        Formula::Atom(s) => s,
        Formula::Not(subformula) => format!("(¬{})", to_str(*subformula)),
        Formula::Implies(bop) => to_str_binary_op(*bop, " → "),
        Formula::Or(bop) => to_str_binary_op(*bop, " ∨ "),
        Formula::And(bop) => to_str_binary_op(*bop, " ∧ "),
    }
}

fn to_str_binary_op(bop: BinaryOperator, op: &str) -> String {
    format!("({}{}{})", to_str(bop.lhs), op, to_str(bop.rhs))
}


