use std::collections::HashMap;
use std::fmt;

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

    pub fn get_representative_class(&self, class_index: usize) -> &EClass {
        let mut class_index = class_index;
        let mut class = self.children.get(class_index).unwrap();
        while class.representative != class_index {
            class_index = class.representative;
            class = self.children.get(class.representative).unwrap();
        }
        return class;
    }

    pub fn extract_node(&self, node: &Node, max_recursion: usize) -> Vec<Expression> {
        return self.extract_node_helper(node, max_recursion, 0);
    }

    pub fn extract_node_helper(
        &self, 
        node: &Node,
        max_recursion: usize,
        current_recursion: usize
    ) -> Vec<Expression> {
        let mut expressions = vec![];

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
            expressions.push(new_expression.into_owned());
        }

        return expressions;
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
        let class = self.get_representative_class(class_index);

        let mut expressions = vec![];

        for node in class.children.iter() {
            expressions.extend(self.extract_node_helper(node, max_recursion, current_recursion))
        }

        return expressions;
    }

    pub fn search(&self, pattern: &Expression, max_recursion: usize) -> Vec<(Assignment<'static>, usize)> {
        let mut needles = vec![];
    
        for class_index in 0..self.children.len() {
            for assignment in self.search_in_class(class_index, pattern, max_recursion) {
                needles.push((assignment, class_index));
            }
        }
    
        needles
    }

    pub fn search_in_class(&self, class_index: usize, pattern: &Expression, max_recursion: usize) -> Vec<Assignment<'static>> {
        let mut needles = vec![];
        let class = self.get_representative_class(class_index);

        for node in class.children.iter() {

            match pattern.t {
                NodeType::MetaVar(x) => {
                    for expression in self.extract_node(node, max_recursion) {
                        let mut map: Assignment = HashMap::new();
                        map.insert(x.to_string(), expression.into_owned());
                        needles.push(map);
                    }
                }
                _ => {
                    if pattern.t == node.t && pattern.children.len() == node.children.len() {

                        let mut child_assignments_list = vec![];

                        for (pattern_child, node_child_index) in
                            pattern.children.iter().zip(node.children.iter())
                        {
                            let child_assignments = self.search_in_class(*node_child_index, pattern_child, max_recursion);
                            
                            child_assignments_list.push(child_assignments);
                        }

                        let child_assignments_product = cartesian_product(&child_assignments_list);

                        for child_assignments in child_assignments_product {

                            let mut assignment: Assignment = HashMap::new();

                            let merged_assignment: Option<Assignment> = (|| {
                                for child_assignment in child_assignments {
                                    for (key, value) in child_assignment {
                                        if let Some(existing_value) = assignment.get(&key) {
                                            if existing_value != &value {
                                                return None;
                                            }
                                        }
                                        assignment.insert(key, value);
                                    }
                                }

                                return Some(assignment);
                            })();

                            if let Some(merged_assignment) = merged_assignment {
                                needles.push(merged_assignment);
                            }
                        }
                    }
                }
            }
        }

        return needles;
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
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

impl<'a> PartialEq for NodeType<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeType::MetaVar(a), NodeType::MetaVar(b)) => a == b,
            (NodeType::Const(a), NodeType::Const(b)) => a == b,
            (NodeType::Var(a), NodeType::Var(b)) => a == b,
            (NodeType::Neg, NodeType::Neg) => true,
            (NodeType::Add, NodeType::Add) => true,
            (NodeType::Sub, NodeType::Sub) => true,
            (NodeType::Mul, NodeType::Mul) => true,
            (NodeType::Div, NodeType::Div) => true,
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

    pub fn into_owned(self) -> Expression<'static> {
        match self.t {
            NodeType::MetaVar(s) => Expression { t: NodeType::MetaVar(Box::leak(s.to_string().into_boxed_str())), children: self.children.into_iter().map(|c| c.into_owned()).collect() },
            NodeType::Const(c) => Expression { t: NodeType::Const(c), children: self.children.into_iter().map(|c| c.into_owned()).collect() },
            NodeType::Var(s) => Expression { t: NodeType::Var(Box::leak(s.to_string().into_boxed_str())), children: self.children.into_iter().map(|c| c.into_owned()).collect() },
            NodeType::Neg => Expression { t: NodeType::Neg, children: self.children.into_iter().map(|c| c.into_owned()).collect() },
            NodeType::Add => Expression { t: NodeType::Add, children: self.children.into_iter().map(|c| c.into_owned()).collect() },
            NodeType::Sub => Expression { t: NodeType::Sub, children: self.children.into_iter().map(|c| c.into_owned()).collect() },
            NodeType::Mul => Expression { t: NodeType::Mul, children: self.children.into_iter().map(|c| c.into_owned()).collect() },
            NodeType::Div => Expression { t: NodeType::Div, children: self.children.into_iter().map(|c| c.into_owned()).collect() },
        }
    }

    /// Creates a new constant expression.
    pub fn constant(value: usize) -> Self {
        Expression {
            t: NodeType::Const(value),
            children: Vec::new(),
        }
    }

    /// Creates a new variable expression.
    pub fn variable(name: &'a str) -> Self {
        Expression {
            t: NodeType::Var(name),
            children: Vec::new(),
        }
    }

    /// Creates a new meta-variable expression.
    pub fn meta_variable(name: &'a str) -> Self {
        Expression {
            t: NodeType::MetaVar(name),
            children: Vec::new(),
        }
    }

    /// Creates a new negation expression.
    pub fn negate(child: Self) -> Self {
        Expression {
            t: NodeType::Neg,
            children: vec![child],
        }
    }

    /// Creates a new addition expression.
    pub fn add(left: Self, right: Self) -> Self {
        Expression {
            t: NodeType::Add,
            children: vec![left, right],
        }
    }

    /// Creates a new subtraction expression.
    pub fn subtract(left: Self, right: Self) -> Self {
        Expression {
            t: NodeType::Sub,
            children: vec![left, right],
        }
    }

    /// Creates a new multiplication expression.
    pub fn multiply(left: Self, right: Self) -> Self {
        Expression {
            t: NodeType::Mul,
            children: vec![left, right],
        }
    }

    /// Creates a new division expression.
    pub fn divide(left: Self, right: Self) -> Self {
        Expression {
            t: NodeType::Div,
            children: vec![left, right],
        }
    }
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.t {
            NodeType::MetaVar(s) => write!(f, "?{}", s),
            NodeType::Const(c) => write!(f, "{}", c),
            NodeType::Var(v) => write!(f, "{}", v),
            NodeType::Neg => {
                if self.children.len() == 1 {
                    write!(f, "-({})", self.children[0])
                } else {
                    write!(f, "-(?)") // Should not happen in a well-formed expression
                }
            }
            NodeType::Add => {
                if self.children.len() == 2 {
                    write!(f, "({} + {})", self.children[0], self.children[1])
                } else {
                    write!(f, "(? + ?)") // Should not happen in a well-formed expression
                }
            }
            NodeType::Sub => {
                if self.children.len() == 2 {
                    write!(f, "({} - {})", self.children[0], self.children[1])
                } else {
                    write!(f, "(? - ?)") // Should not happen in a well-formed expression
                }
            }
            NodeType::Mul => {
                if self.children.len() == 2 {
                    write!(f, "({} * {})", self.children[0], self.children[1])
                } else {
                    write!(f, "(? * ?)") // Should not happen in a well-formed expression
                }
            }
            NodeType::Div => {
                if self.children.len() == 2 {
                    write!(f, "({} / {})", self.children[0], self.children[1])
                } else {
                    write!(f, "(? / ?)") // Should not happen in a well-formed expression
                }
            }
        }
    }
}