use std::ops;
use std::rc::Rc;
use std::cell::RefCell;
// use petgraph::graph::{Graph};


type MutableRc<T> = Rc<RefCell<T>>;
fn mutable_rc<T>(data: T) -> MutableRc<T> {
    Rc::new(RefCell::new(data))
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Mul
}

#[derive(Debug, Clone)]
struct Value {
    label: String,
    data: f64,
    operation: Option<Operation>,
    children: Vec<MutableRc<Value>>
}


impl Value {
    // Initializing new value 
    fn new(data: f64, label: String) -> Self {
        Self { data, label, ..Value::default() }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value { label: "".to_owned(), data: 0., operation: None, children: vec![] }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        // New value label
        let new_label = format!("{}{}", self.label, other.label);
        let new_value = self.data + other.data;
        Value { 
            label: new_label, 
            data: new_value, 
            operation: Some(Operation::Add),
            ..Value::default()
        }
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        // New value label
        let new_label = format!("{}{}", self.label, other.label);
        let new_value = self.data * other.data;
        Value { 
            label: new_label, 
            data: new_value, 
            operation: Some(Operation::Mul), 
            ..Value::default()
        }
    }
}

#[derive(Debug)]
struct ValueGraph {
    values: Vec<Value>,
}

impl ValueGraph {
   // Create a new ValueGraph using nodes as input
   fn new(values: Vec<Value>) -> Self {
       Self { values }
   }
}


fn main() {
    let first_value = Value::new(5., "Hello".to_owned());
    let second_value = Value::new(10., "World".to_owned());

    println!("This is value A: {:?}", first_value.clone());        
    println!("This is value B: {:?}", second_value.clone());

    let added_values = first_value.clone() + second_value.clone();

    println!("This is the sum of those two values: {:?}", added_values.data);
    println!("The operation performed was: {:?}", added_values.operation);
    println!("The children of this node is: {:?}", added_values.children);

    let my_graph = ValueGraph::new(vec![first_value.clone(), second_value.clone(), added_values]);
    println!("This is my graph: {:?}", my_graph.values);
}
