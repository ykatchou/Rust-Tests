use std::rc::Rc;
mod graph_lib;
use graph_lib::*;

fn main() {
    println!("Hello, world!");

    let mut graph: Graph<&str> = Graph::new();

    let node1: Rc<Node<&str>> = graph.create_node("node1");
    let node2: Rc<Node<&str>> = graph.create_node("node2");
    let node3: Rc<Node<&str>> = graph.create_node("node3");
    let node4: Rc<Node<&str>> = graph.create_node("node4");

    graph.create_vertex(Rc::downgrade(&node1), Rc::downgrade(&node2));
    graph.create_vertex(Rc::downgrade(&node2), Rc::downgrade(&node3));
    graph.create_vertex(Rc::downgrade(&node3), Rc::downgrade(&node4));
    graph.create_vertex(Rc::downgrade(&node4), Rc::downgrade(&node1));

    let node_1 = graph.get_node("node1");
    let node_2 = graph.get_node("node2");

    println!("{}", node_1.as_ref());
    println!("{}", node_2.as_ref());

    let vertices = graph.get_vertex_from(Rc::downgrade(node_1));

    for vertex in vertices {
        println!("{}", vertex.upgrade().unwrap().as_ref());
    }
}
