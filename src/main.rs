use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::cell::RefCell;
use std::ops;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn tanh(x: f64) -> f64 {
    (std::f64::consts::E.powf(x) - std::f64::consts::E.powf(-x))
        / (std::f64::consts::E.powf(x) + std::f64::consts::E.powf(-x))
}

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
    gradient: f64,
    operation: Option<Operation>,
    previous: Vec<MutableRc<Value>>,
}

impl Value {
    // Initializing new value
    fn new(data: f64, label: String) -> Self {
        Self {
            id: VALUE_COUNTER.fetch_add(1, Ordering::SeqCst),
            data,
            gradient: 0.,
            label,
            operation: None,
            previous: vec![],
        }
    }

    // Change label
    fn set_label(&mut self, label: &str) {
        self.label = label.to_owned();
    }

    // Apply function to value
    fn apply<F>(&mut self, action: F)
    where
        F: Fn(f64) -> f64,
    {
        self.data = action(self.data);
    }
}

impl ops::Add<&Value> for &Value {
    type Output = Value;

    fn add(self, other: &Value) -> Value {
        Value {
            id: VALUE_COUNTER.fetch_add(1, Ordering::SeqCst),
            label: format!("[{}+{}]", self.label, other.label),
            gradient: 0.,
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
            label: format!("[{}*{}]", self.label, other.label),
            data: self.data * other.data,
            gradient: 0.,
            operation: Some(Operation::Mul),
            previous: vec![mutable_rc(self.to_owned()), mutable_rc(other.to_owned())],
        }
    }
}

fn format_node(node: &MutableRc<Value>) -> String {
    format!(
        "{}({:.2}) | {:.2}",
        node.borrow().label.clone(),
        node.borrow().data,
        node.borrow().gradient
    )
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
    let x_1 = Value::new(2., "x1".to_owned());
    let x_2 = Value::new(0., "x2".to_owned());
    let w_1 = Value::new(-3., "w1".to_owned());
    let w_2 = Value::new(1., "w1".to_owned());

    let b = Value::new(6.7, "b".to_owned());
    // Multiply xn * wn to create neuron
    let xw_1 = &x_1 * &w_1;
    let xw_2 = &x_2 * &w_2;

    // Run neurons
    let mut neuron = &(&xw_1 + &xw_2) + &b;
    neuron.set_label("neuron");

    // Run activation function on neuron
    neuron.apply(|value| tanh(value));

    let (nodes, edges) = trace(mutable_rc(neuron));
    plot_graph(&nodes, &edges);
}
