mod graphs;
mod rules;

use graphs::*;
use rules::*;

fn main() {
    let mut graph = EGraph::init();
    let expression = Expression::divide(Expression::multiply(Expression::variable("x"), Expression::constant(-6)), Expression::multiply(Expression::variable("x"), Expression::constant(-3)));

    let index = graph.add_expression(expression);

    println!("{:?}", index);
    let node = graph.children.get(index).unwrap();

    println!("{:?}", node);
    println!("{:?}", graph.extract_all(index, 10));

    let mut graph_copy = graph.clone();
    
    for i in 0..4 {
        println!("iteration {}", i);

        let mut matches = vec![];
        graph_copy = graph.clone();

        for rule in Rule::rules() {
            let search_results = graph_copy.search(&rule.lhs, 3);
            for (assignment, eclass_index) in search_results {
                matches.push((rule.rhs.clone(), assignment, eclass_index));
            }
        }

        for (assignment, eclass_index) in graph_copy.search(&Expression::meta_variable("a"), 3) {
            let expression = Expression::meta_variable("a").apply_assignment(&assignment);
            if let Some(const_value) = expression.const_eval() {
                let eclass_index2 = graph.add_expression(Expression::constant(const_value));
                graph.union(eclass_index2, eclass_index);

                if const_value < 0 {
                    let eclass_index3 = graph.add_expression(Expression::negate(Expression::constant(-const_value)));
                    graph.union(eclass_index3, eclass_index);
                }
            }
        }

        for (pattern, assignment, eclass_index) in matches {
            let eclass_index2 = graph.add_expression(pattern.apply_assignment(&assignment));
            graph.union(eclass_index2, eclass_index);
        }
    }

    for expression in graph.extract_all(index, 2) {
        println!("{}", expression);
    }

    println!("----------");
    /* 
    let pattern = Expression::divide(Expression::meta_variable("a"), Expression::meta_variable("b"));
    for (assignment, class_index) in graph.search(&pattern, 1) {
        let expression = pattern.apply_assignment(&assignment);
        println!("{} {}", expression, class_index);
    }
    */
    
}
