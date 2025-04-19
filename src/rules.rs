use crate::graphs::*;

pub struct Rule<'a> {
    pub lhs: Expression<'a>,
    pub rhs: Expression<'a>
}

impl<'a> Rule<'a> {

    pub fn rules() -> Vec<Rule<'a>> {
        return vec![
            // Commutativity of addition
            Rule {
                lhs: Expression::add(Expression::meta_variable("a"), Expression::meta_variable("b")),
                rhs: Expression::add(Expression::meta_variable("b"), Expression::meta_variable("a")),
            },
            // Identity element of addition
            Rule {
                lhs: Expression::add(Expression::meta_variable("a"), Expression::constant(0)),
                rhs: Expression::meta_variable("a"),
            },
            // Commutativity of multiplication
            Rule {
                lhs: Expression::multiply(Expression::meta_variable("a"), Expression::meta_variable("b")),
                rhs: Expression::multiply(Expression::meta_variable("b"), Expression::meta_variable("a")),
            },
            // Identity element of multiplication
            Rule {
                lhs: Expression::multiply(Expression::meta_variable("a"), Expression::constant(1)),
                rhs: Expression::meta_variable("a"),
            },
            // Multiplication by zero
            Rule {
                lhs: Expression::multiply(Expression::meta_variable("a"), Expression::constant(0)),
                rhs: Expression::constant(0),
            },
            // Associativity of addition
            Rule {
                lhs: Expression::add(
                    Expression::add(Expression::meta_variable("a"), Expression::meta_variable("b")),
                    Expression::meta_variable("c"),
                ),
                rhs: Expression::add(
                    Expression::meta_variable("a"),
                    Expression::add(Expression::meta_variable("b"), Expression::meta_variable("c")),
                ),
            },
            // Associativity of multiplication
            Rule {
                lhs: Expression::multiply(
                    Expression::multiply(Expression::meta_variable("a"), Expression::meta_variable("b")),
                    Expression::meta_variable("c"),
                ),
                rhs: Expression::multiply(
                    Expression::meta_variable("a"),
                    Expression::multiply(Expression::meta_variable("b"), Expression::meta_variable("c")),
                ),
            },
            // Associativity of multiplication and division
            Rule {
                lhs: Expression::divide(
                    Expression::multiply(Expression::meta_variable("a"), Expression::meta_variable("b")),
                    Expression::meta_variable("c"),
                ),
                rhs: Expression::multiply(
                    Expression::meta_variable("a"),
                    Expression::divide(Expression::meta_variable("b"), Expression::meta_variable("c")),
                ),
            },
            // Distributivity of multiplication over addition (left)
            Rule {
                lhs: Expression::multiply(
                    Expression::meta_variable("a"),
                    Expression::add(Expression::meta_variable("b"), Expression::meta_variable("c")),
                ),
                rhs: Expression::add(
                    Expression::multiply(Expression::meta_variable("a"), Expression::meta_variable("b")),
                    Expression::multiply(Expression::meta_variable("a"), Expression::meta_variable("c")),
                ),
            },
            // Distributivity of multiplication over addition (right)
            Rule {
                lhs: Expression::multiply(
                    Expression::add(Expression::meta_variable("b"), Expression::meta_variable("c")),
                    Expression::meta_variable("a"),
                ),
                rhs: Expression::add(
                    Expression::multiply(Expression::meta_variable("b"), Expression::meta_variable("a")),
                    Expression::multiply(Expression::meta_variable("c"), Expression::meta_variable("a")),
                ),
            },
            // Negation of zero
            Rule {
                lhs: Expression::negate(Expression::constant(0)),
                rhs: Expression::constant(0),
            },
            // Negation of a negation
            Rule {
                lhs: Expression::negate(Expression::negate(Expression::meta_variable("a"))),
                rhs: Expression::meta_variable("a"),
            },
            // Subtraction as addition with negation
            Rule {
                lhs: Expression::subtract(Expression::meta_variable("a"), Expression::meta_variable("b")),
                rhs: Expression::add(Expression::meta_variable("a"), Expression::negate(Expression::meta_variable("b"))),
            },
            // Division by one
            Rule {
                lhs: Expression::divide(Expression::meta_variable("a"), Expression::constant(1)),
                rhs: Expression::meta_variable("a"),
            },
            // Division by the same constant
            Rule {
                lhs: Expression::divide(Expression::meta_variable("a"), Expression::meta_variable("a")),
                rhs: Expression::constant(1),
            },
        ];
    }

}