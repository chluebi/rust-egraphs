mod graphs;
mod rules;

use graphs::*;
use rules::*;

fn main() {
    let mut graph = EGraph::init();
    let expression = Expression::divide(Expression::multiply(Expression::variable("x"), Expression::constant(2)), Expression::constant(2));

    let index = graph.add_expression(expression);

    println!("{:?}", index);
    let node = graph.children.get(index).unwrap();

    println!("{:?}", node);
    println!("{:?}", graph.extract_all(index, 2));

    let mut graph_copy = graph.clone();
    
    for _ in 0..3 {
        let mut matches = vec![];
        graph_copy = graph.clone();

        for rule in Rule::rules() {
            let search_results = graph_copy.search(&rule.lhs, 4);
            for (assignment, eclass_index) in search_results {
                matches.push((rule.rhs.clone(), assignment, eclass_index));
            }
        }

        for (pattern, assignment, eclass_index) in matches {
            let eclass_index2 = graph.add_expression(pattern.apply_assignment(&assignment));
            graph.union(eclass_index2, eclass_index);
        }
    }

    println!("{:?}", graph.extract_all(index, 2));

    for expression in graph.extract_all(index, 2) {
        println!("{}", expression);
    }
    
}
