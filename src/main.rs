use std::cell::RefCell;
use std::ops;
use std::rc::Rc;
use petgraph::graph::Graph;
use petgraph::dot::Dot;

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
    label: String,
    data: f64,
    operation: Option<Operation>,
    children: Vec<MutableRc<Value>>,
}

impl Value {
    // Initializing new value
    fn new(data: f64, label: String) -> Self {
        Self {
            data,
            label,
            ..Value::default()
        }
    }

    // // Add child
    // fn add_child(&mut self, child: MutableRc<Value>) {
    //     self.children.push(child);
    // }
    //
    // // Simple depth first algorithm
    // fn depth_first(&self) {
    //     println!("Value {}", self.label);
    //     for child in self.children.iter() {
    //         child.borrow().depth_first();
    //     }
    // }
}

impl Default for Value {
    fn default() -> Value {
        Value {
            label: "".to_owned(),
            data: 0.,
            operation: None,
            children: vec![],
        }
    }
}

impl ops::Add<&Value> for &Value {
    type Output = Value;

    fn add(self, other: &Value) -> Value {
        Value {
            label: format!("{}{}", self.label, other.label),
            data: self.data + other.data,
            operation: Some(Operation::Add),
            children: vec![mutable_rc(self.to_owned()), mutable_rc(other.to_owned())],
        }
    }
}

impl ops::Mul<&Value> for &Value {
    type Output = Value;

    fn mul(self, other: &Value) -> Value {
        Value {
            label: format!("{}{}", self.label, other.label),
            data: self.data * other.data,
            operation: Some(Operation::Mul),
            children: vec![mutable_rc(self.to_owned()), mutable_rc(other.to_owned())],
        }
    }
}

#[derive(Debug, Clone)]
struct ValueGraph {
    nodes: Vec<MutableRc<Value>>,
    edges: Vec<(MutableRc<Value>, MutableRc<Value>)>,
}

impl ValueGraph {
    fn new() -> ValueGraph {
        ValueGraph { nodes: vec![], edges: vec![] }
    }

    fn new_with_nodes(values: Vec<MutableRc<Value>>) -> ValueGraph {
        let mut graph = Self::new();
        for node in values {
            graph._build(node);
        }
        return graph
    }

    fn _build(&mut self, node: MutableRc<Value>) {
        if !self.nodes.contains(&node) { 
           self.nodes.push(Rc::clone(&node));
           for child in &node.borrow().children {
                self.edges.push((Rc::clone(&node), Rc::clone(&child)));
                self._build(Rc::clone(child));
           }
        }
    }

    fn plot(&self) {
        // Build an empty base graph
        let mut g: Graph<&str, &str> = Graph::new();
        // Add nodes
        let a = g.add_node("Node vertex: (a)");
        let b = g.add_node("Node vertex: (b)");
        // Add edges
        g.add_edge(a, b, "edge from a to b");
        println!("Plotted this thing...\n{}", Dot::new(&g));
    }
}   

fn main() {
    let first_value = Value::new(5., "Hello".to_owned());
    let second_value = Value::new(10., "World".to_owned());
    // Operator overload working on references
    let third_value = &first_value + &second_value;
    // Build a graph
    let graph = ValueGraph::new_with_nodes(vec![mutable_rc(first_value), mutable_rc(second_value), mutable_rc(third_value)]);
    // Let's see if it's true
    // dbg!(&graph);
    graph.plot();
}
