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

// Plot helper function
fn plot_graph(nodes: Vec<MutableRc<Value>>) {
    // Iterate over nodes
    let mut all_nodes = vec![];
    let mut all_edges = vec![];

    for node in nodes.iter() {
        // Iterate over these previous nodes, format them, and add to edges
        let formatted_nodes: Vec<(usize, String)> = node
            .borrow()
            .previous
            .iter()
            .map(|p| (p.borrow().id, format_node(p)))
            .collect();
        // Build edges
        let edges: Vec<(usize, usize)> = formatted_nodes
            .iter()
            .map(|(id, _node)| (id.clone(), node.borrow().id))
            .collect();
        // Push both to their respective containers
        all_nodes.extend(formatted_nodes);
        all_nodes.extend(vec![(node.borrow().id, format_node(node))]);
        all_edges.extend(edges);
    }
    // Create empty graph
    let mut graph = Graph::<String, String, Directed, usize>::from_edges(&all_edges);
    // Add node's value
    for (id, node) in all_nodes.iter() {
        let mutable_node = graph.node_weight_mut(NodeIndex::new(id.clone()));
        if let Some(x) = mutable_node {
            x.clear();
            x.push_str(node);
        }
    }
    // Plot graph
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}

fn main() {
    let first_value = Value::new(5., "A".to_owned());
    let second_value = Value::new(10., "B".to_owned());
    let third_value = &first_value + &second_value;
    let fourth_value = &third_value + &first_value;
    // Plot this graph
    plot_graph(vec![mutable_rc(fourth_value)]);
}
