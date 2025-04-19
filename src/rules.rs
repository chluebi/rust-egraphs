use crate::graphs::*;

pub struct Rule<'a> {
    pub lhs: Expression<'a>,
    pub rhs: Expression<'a>
}
impl<'a> Rule<'a> {

    pub fn rules() -> Vec<Rule<'a>> {
        return vec![
            Rule {
                lhs: Expression {
                    t: NodeType::Add,
                    children: vec![
                        Expression {
                            t: NodeType::MetaVar("a"),
                            children: vec![],
                        },
                        Expression {
                            t: NodeType::MetaVar("b"),
                            children: vec![],
                        },
                    ],
                },
                rhs: Expression {
                    t: NodeType::Add,
                    children: vec![
                        Expression {
                            t: NodeType::MetaVar("b"),
                            children: vec![],
                        },
                        Expression {
                            t: NodeType::MetaVar("a"),
                            children: vec![],
                        },
                    ],
                }
            },

            Rule {
                lhs: Expression {
                    t: NodeType::Add,
                    children: vec![
                        Expression {
                            t: NodeType::MetaVar("a"),
                            children: vec![],
                        },
                        Expression {
                            t: NodeType::Const(0),
                            children: vec![],
                        },
                    ],
                },
                rhs: Expression {
                    t: NodeType::MetaVar("a"),
                    children: vec![]
                }
            },
        ];
    }

}