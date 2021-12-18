use std::{collections::BTreeMap, rc::{Rc, Weak}, fmt::{Display, Formatter, Result}};

pub struct Node<T> {
    pub data: Rc<T>,
}

pub struct Vertex<T> {
    source: Weak<Node<T>>,
    target: Weak<Node<T>>,
}

pub struct Graph<T> {
    nodes: BTreeMap<String, Rc<Node<T>>>,
    vertices: Vec<Rc<Vertex<T>>>,
}

impl<T> Node<T> {
    pub fn new(p_data: Rc<T>) -> Node<T> {
        Node {
            data: p_data,
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Node => {}", self.data.as_ref())
    }
}

impl<T> Vertex<T> {
    pub fn new(source_node: Weak<Node<T>>, target_node: Weak<Node<T>>) -> Vertex<T> {
        Vertex {
            source: source_node,
            target: target_node,
        }
    }
}

impl<T: Display> Display for Vertex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vertex: {} => {}", self.source.upgrade().as_ref().unwrap(), self.target.upgrade().as_ref().unwrap())
    }
}

impl<T: Copy + Display + Ord> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            nodes: BTreeMap::new(),
            vertices: Vec::new(),
        }
    }

    pub fn create_node(&mut self, data: T) -> Rc<Node<T>> {
        let node = Rc::new(Node::new(Rc::new(data)));

        self.nodes.insert(data.to_string(), node.clone());

        return node;
    }

    pub fn get_node(& self, data: T) -> &Rc<Node<T>> {
        if self.nodes.contains_key(&data.to_string()) {
            if let Some(xx) = self.nodes.get(&data.to_string()) {
                return xx;
            } else {
                panic!("Node not found");
            }
        } else {
            panic!("Node not found")
        }
    }

    pub fn create_vertex(&mut self, source_node_name: Weak<Node<T>>, target_node_name: Weak<Node<T>>) {
        self.vertices
            .push(Rc::new(Vertex::new(source_node_name, target_node_name)));
    }

    pub fn get_vertex_from(& self, node: Weak<Node<T>>) -> Vec<Weak<Vertex<T>>> {
        let mut result = Vec::new();

        for vertex in &self.vertices {
            if let Some(src_node) = vertex.source.upgrade() {
                if Rc::ptr_eq(&src_node, &node.upgrade().expect("Invalid reference to a node")) {
                    result.push(Rc::downgrade(vertex));

                    continue;
                }
            }

            if let Some(target_node) = vertex.target.upgrade() {
                if Rc::ptr_eq(&target_node, &node.upgrade().expect("Invalid reference to a node")) {
                    result.push(Rc::downgrade(vertex));
                }
            }
        }

        return result;
    }
}
