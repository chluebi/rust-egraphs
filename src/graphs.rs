use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EGraph<'a> {
    pub children: Vec<EClass<'a>>,
}

impl<'a> EGraph<'a> {
    pub fn init() -> EGraph<'a> {
        return EGraph { children: vec![] };
    }

    pub fn add_expression(&mut self, expression: Expression<'a>) -> usize {
        for class_index in 0..self.children.len() {
            if self.class_syntactically_equal_to_expression(&expression, class_index) {
                return class_index;
            }
        }

        let child_indices: Vec<usize> = expression
            .children
            .into_iter()
            .map(|x| self.add_expression(x))
            .collect();

        self.children.push(EClass {
            representative: self.children.len(),
            children: vec![Node {
                t: expression.t,
                children: child_indices,
            }],
        });

        return self.children.len() - 1;
    }

    pub fn class_syntactically_equal_to_expression(
        &self,
        expression: &Expression<'a>,
        class_index: usize,
    ) -> bool {
        let class = self.children.get(class_index).unwrap();

        for node in class.children.iter() {
            if expression.t == node.t && expression.children.len() == node.children.len() {
                if expression
                    .children
                    .clone()
                    .into_iter()
                    .zip(node.children.clone())
                    .all(|(expression, class_index)| {
                        self.class_syntactically_equal_to_expression(&expression, class_index)
                    })
                {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn union(&mut self, class_index1: usize, class_index2: usize) {
        if class_index1 == class_index2
            || class_index1 >= self.children.len()
            || class_index2 >= self.children.len()
        {
            return;
        }

        let (idx1, idx2) = if class_index1 < class_index2 {
            (class_index1, class_index2)
        } else {
            (class_index2, class_index1)
        };

        let (before, rest) = self.children.split_at_mut(idx2);

        let class1 = before.get_mut(idx1).unwrap();
        let class2 = rest.get_mut(0).unwrap();
        class1.children.extend(class2.children.clone());
        class2.representative = class_index1;
        class2.children = vec![];
    }

    pub fn extract_all(&self, class_index: usize, max_recursion: usize) -> Vec<Expression> {
        return self.extract_all_helper(class_index, max_recursion, 0);
    }

    pub fn extract_all_helper(
        &self,
        class_index: usize,
        max_recursion: usize,
        current_recursion: usize,
    ) -> Vec<Expression> {
        if current_recursion > max_recursion {
            return vec![];
        }
        let mut class_index = class_index;
        let mut class = self.children.get(class_index).unwrap();
        while class.representative != class_index {
            class_index = class.representative;
            class = self.children.get(class.representative).unwrap();
        }

        let mut expressions = vec![];

        for node in class.children.iter() {
            let expression = Expression {
                t: node.t.clone(),
                children: vec![],
            };

            let mut child_expression_lists = vec![];

            for child_index in node.children.iter() {
                let child_expressions =
                    self.extract_all_helper(*child_index, max_recursion, current_recursion + 1);
                child_expression_lists.push(child_expressions);
            }

            let child_expressions_product = cartesian_product(&child_expression_lists);

            for child_expressions in child_expressions_product {
                let mut new_expression = expression.clone();
                new_expression.children = child_expressions;
                expressions.push(new_expression);
            }
        }

        return expressions;
    }

    pub fn search<'b>(&'a self, pattern: &'b Expression<'a>, max_recursion: usize) -> Vec<(Assignment<'a>, usize)> {
        let mut needles = vec![];

        for class_index in 0..self.children.len() {
            for expression in self.extract_all(class_index, max_recursion) {
                if let Some(assignment) = pattern.structural_match(&expression) {
                    needles.push((assignment, class_index));
                }
            }
        }

        needles
    }
}

fn cartesian_product<T: Clone>(lists: &[Vec<T>]) -> Vec<Vec<T>> {
    if lists.is_empty() {
        return vec![vec![]];
    }

    let first_list = &lists[0];
    let rest_lists = &lists[1..];

    let rest_product = cartesian_product(rest_lists);
    let mut result = Vec::new();

    for item in first_list {
        for product in &rest_product {
            let mut new_product = vec![item.clone()];
            new_product.extend_from_slice(product);
            result.push(new_product);
        }
    }

    result
}

#[derive(Debug, Clone)]
pub struct EClass<'a> {
    representative: usize,
    pub children: Vec<Node<'a>>,
}

impl EClass<'_> {}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub t: NodeType<'a>,
    pub children: Vec<usize>,
}

#[derive(Debug, Clone)]
pub enum NodeType<'a> {
    MetaVar(&'a str),
    Const(usize),
    Var(&'a str),
    Add,
}

impl<'a> PartialEq for NodeType<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeType::MetaVar(a), NodeType::MetaVar(b)) => a == b,
            (NodeType::Const(a), NodeType::Const(b)) => a == b,
            (NodeType::Var(a), NodeType::Var(b)) => a == b,
            (NodeType::Add, NodeType::Add) => true,
            _ => false,
        }
    }
}

impl<'a> Eq for NodeType<'a> {}

#[derive(Debug, Clone)]
pub struct Expression<'a> {
    pub t: NodeType<'a>,
    pub children: Vec<Expression<'a>>,
}

impl<'a> PartialEq for Expression<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.children == other.children
    }
}

impl<'a> Eq for Expression<'a> {}

type Assignment<'a> = HashMap<String, Expression<'a>>;

impl<'a> Expression<'a> {
    pub fn structural_match(&self, expression: &Expression<'a>) -> Option<Assignment<'a>> {
        match &self.t {
            NodeType::MetaVar(x) => {
                let mut map: Assignment = HashMap::new();
                map.insert(x.to_string(), expression.clone());
                return Some(map);
            }
            _ => {
                if self.t == expression.t && self.children.len() == expression.children.len() {
                    let mut assignment: Assignment = HashMap::new();
                    for (pattern_child, expression_child) in
                        self.children.iter().zip(expression.children.iter())
                    {
                        match pattern_child.structural_match(expression_child) {
                            Some(child_assignment) => {
                                for (key, value) in child_assignment {
                                    if let Some(existing_value) = assignment.get(&key) {
                                        if existing_value != &value {
                                            return None; // Inconsistent assignment
                                        }
                                    }
                                    assignment.insert(key, value);
                                }
                            }
                            _ => return None, // Child match failed
                        }
                    }
                    return Some(assignment);
                }
            }
        }
        None
    }

    pub fn apply_assignment(&self, assignment: &Assignment<'a>) -> Expression<'a> {
        match &self.t {
            NodeType::MetaVar(x) => match assignment.get(&x.to_string()) {
                Some(expr) => expr.clone(),
                _ => self.clone(),
            },
            _ => {
                let new_children: Vec<Expression<'a>> = self
                    .children
                    .iter()
                    .map(|child| child.apply_assignment(assignment))
                    .collect();
                Expression {
                    t: self.t.clone(),
                    children: new_children,
                }
            }
        }
    }
}
