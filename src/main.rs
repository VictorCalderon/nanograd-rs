use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::cell::RefCell;
use std::ops;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static VALUE_COUNTER: AtomicUsize = AtomicUsize::new(0);

type MutableRc<T> = Rc<RefCell<T>>;
fn mutable_rc<T>(data: T) -> MutableRc<T> {
    Rc::new(RefCell::new(data))
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug, Clone, PartialEq)]
struct Value {
    id: usize,
    label: String,
    data: f64,
    operation: Option<Operation>,
    previous: Vec<MutableRc<Value>>,
}

impl Value {
    // Initializing new value
    fn new(data: f64, label: String) -> Self {
        Self {
            id: VALUE_COUNTER.fetch_add(1, Ordering::SeqCst),
            data,
            label,
            operation: None,
            previous: vec![],
        }
    }
}

impl ops::Add<&Value> for &Value {
    type Output = Value;

    fn add(self, other: &Value) -> Value {
        Value {
            id: VALUE_COUNTER.fetch_add(1, Ordering::SeqCst),
            label: format!("[{}+{}]", self.label, other.label),
            data: self.data + other.data,
            operation: Some(Operation::Add),
            previous: vec![mutable_rc(self.to_owned()), mutable_rc(other.to_owned())],
        }
    }
}

impl ops::Mul<&Value> for &Value {
    type Output = Value;

    fn mul(self, other: &Value) -> Value {
        Value {
            id: VALUE_COUNTER.fetch_add(1, Ordering::SeqCst),
            label: format!("[{}x{}]", self.label, other.label),
            data: self.data * other.data,
            operation: Some(Operation::Mul),
            previous: vec![mutable_rc(self.to_owned()), mutable_rc(other.to_owned())],
        }
    }
}

fn format_node(node: &MutableRc<Value>) -> String {
    format!("{}({})", node.borrow().label.clone(), node.borrow().data)
}

type Node = MutableRc<Value>;
type Edge = (Node, Node);

fn build(node: MutableRc<Value>, nodes: &mut Vec<Node>, edges: &mut Vec<Edge>) {
    if nodes.contains(&node) {
        return;
    }

    nodes.push(Rc::clone(&node));
    for child in node.borrow().previous.iter() {
        edges.push((Rc::clone(&child), Rc::clone(&node)));
        build(Rc::clone(child), nodes, edges);
    }
}

fn trace(root: MutableRc<Value>) -> (Vec<Node>, Vec<Edge>) {
    let mut nodes = vec![];
    let mut edges = vec![];
    build(root, &mut nodes, &mut edges);
    (nodes, edges)
}

fn plot_graph(nodes: &Vec<Node>, edges: &Vec<Edge>) {
    // Parse edges as a vector<usize, usize>
    let parsed_edges: Vec<(usize, usize)> = edges
        .iter()
        .map(|e| (e.0.borrow().id, e.1.borrow().id))
        .collect();
    // Build a Graph
    let mut graph = Graph::<String, String, Directed, usize>::from_edges(&parsed_edges);
    // Iterate over nodes
    for node in nodes.iter() {
        let grab_node = graph.node_weight_mut(NodeIndex::new(node.borrow().id));
        if let Some(n) = grab_node {
            n.truncate(0);
            n.push_str(&format_node(node));
        }
    }
    // Extend this graph with edges
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}

fn main() {
    let first_value = Value::new(5., "A".to_owned());
    let second_value = Value::new(10., "B".to_owned());
    let third_value = &first_value + &second_value;
    let fourth_value = Value::new(0.5, "C".to_owned());
    let fifth_value = &fourth_value * &third_value;
    let sixth_value = Value::new(10., "D".to_owned());
    let seventh_value = &fifth_value + &sixth_value;
    let (nodes, edges) = trace(mutable_rc(seventh_value));
    plot_graph(&nodes, &edges);
}
