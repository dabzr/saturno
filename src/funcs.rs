use crate::Formula;
//use std::vec::Vec;
//use std::collections::HashSet;

pub fn length(formula: Formula) -> i32 {
    /*Determines the length of a formula in propositional logic.*/
    match formula {
        Formula::Atom(_) => 1,
        Formula::Not(inner) => length(*inner) + 1,
        Formula::Or(bop) | Formula::And(bop) | Formula::Implies(bop) => length(bop.lhs) + length(bop.rhs) + 1
    }
}
/*
pub fn subformulas(formula: Formula) -> Vec<i32> {
    /*Returns the set of all subformulas of a formula.

    For example, observe the piece of code below.

    my_formula = Implies(Atom('p'), Or(Atom('p'), Atom('s')))
    for subformula in subformulas(my_formula):
        print(subformula)

    This piece of code prints p, s, (p v s), (p → (p v s))
    (Note that there is no repetition of p)
    */
    
}
//  we have shown in class that, for all formula A, len(subformulas(A)) <= length(A).


pub fn atoms(formula: Formula) -> HashSet<String> {
    /*Returns the set of all atoms occurring in a formula.

    For example, observe the piece of code below.

    my_formula = Implies(Atom('p'), Or(Atom('p'), Atom('s')))
    for atom in atoms(my_formula):
        print(atom)

    This piece of code above prints: p, s
    (Note that there is no repetition of p)
    */
}

pub fn number_of_atoms(formula: Formula) -> i32 {
    /*Returns the number of atoms occurring in a formula.
    For instance,
    number_of_atoms(Implies(Atom('q'), And(Atom('p'), Atom('q'))))

    must return 3 (Observe that this function counts the repetitions of atoms)
    */
}

pub fn number_of_connectives(formula: Formula) -> i32 {
    /*Returns the number of connectives occurring in a formula.*/
}

pub fn is_literal(formula: Formula) -> bool {
    /*Returns True if formula is a literal. It returns False, otherwise*/
}

pub fn substitution(formula: Formula, old_subformula: Formula, new_subformula: Formula) -> Formula {
    /*Returns a new formula obtained by replacing all occurrences
    of old_subformula in the input formula by new_subformula.*/
}

pub fn is_clause(formula: Formula) -> bool {
    /*Returns True if formula is a clause. It returns False, otherwise*/
    // ======== REMOVE THIS LINE AND INSERT YOUR CODE HERE ========
}

pub fn is_negation_normal_form(formula: Formula) -> bool {
    /*Returns True if formula is in negation normal form.
    Returns False, otherwise.*/
    // ======== REMOVE THIS LINE AND INSERT YOUR CODE HERE ========
}

pub fn is_cnf(formula: Formula) -> bool {
    /*Returns True if formula is in conjunctive normal form.
    Returns False, otherwise.*/
    // ======== REMOVE THIS LINE AND INSERT YOUR CODE HERE ========
}

pub fn is_term(formula: Formula) -> bool {
    /*Returns True if formula is a term. It returns False, otherwise*/
    // ======== REMOVE THIS LINE AND INSERT YOUR CODE HERE ========
}

pub fn is_dnf(formula: Formula) -> bool {
    /*Returns True if formula is in disjunctive normal form.
    Returns False, otherwise.*/
    // ======== REMOVE THIS LINE AND INSERT YOUR CODE HERE ========
}

pub fn is_decomposable_negation_normal_form(formula: Formula) -> bool{
    /*Returns True if formula is in decomposable negation normal form.
    Returns False, otherwise.*/
    // ======== REMOVE THIS LINE AND INSERT YOUR CODE HERE ========
}

*/
