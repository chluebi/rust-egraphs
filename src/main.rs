mod graphs;
mod rules;

use graphs::*;
use rules::*;

fn main() {
    let mut graph = EGraph::init();
    let expression = Expression {
        t: NodeType::Add,
        children: vec![
            Expression {
                t: NodeType::Const(0),
                children: vec![],
            },
            Expression {
                t: NodeType::Var("x"),
                children: vec![],
            },
        ],
    };
    let expression2 = expression.clone();
    let expression3 = Expression {
        t: NodeType::Add,
        children: vec![
            Expression {
                t: NodeType::Const(0),
                children: vec![],
            },
            Expression {
                t: NodeType::Const(0),
                children: vec![],
            },
        ],
    };

    let index = graph.add_expression(expression);
    let index2 = graph.add_expression(expression2);
    let index3 = graph.add_expression(expression3);

    println!("{:?}", index);
    println!("{:?}", index2);
    println!("{:?}", index3);

    let node = graph.children.get(index).unwrap();
    let node2 = graph.children.get(index2).unwrap();
    let node3 = graph.children.get(index3).unwrap();

    println!("{:?}", node);
    println!("{:?}", node2);
    println!("{:?}", node3);

    println!("{:?}", graph.extract_all(index2, 2));

    let mut matches = vec![];

    let graph_copy = graph.clone();

    for rule in Rule::rules() {
        for (assignment, eclass_index) in graph_copy.search(&rule.lhs, 3) {
            matches.push((rule.rhs.clone(), assignment, eclass_index));
        }
    }

    for (pattern, assignment, eclass_index) in matches {
        let eclass_index2 = graph.add_expression(pattern.apply_assignment(&assignment));
        graph.union(eclass_index, eclass_index2);
    }
    
    println!("{:?}", graph.extract_all(index2, 2));
    
}
