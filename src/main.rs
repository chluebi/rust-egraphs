mod graphs;

use graphs::*;

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

    println!("{:?}", graph.extract_all(index2, 10));
}
