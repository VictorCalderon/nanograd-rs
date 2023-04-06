use std::ops;
// use petgraph::graph::{Graph};


#[derive(Debug, Clone)]
enum Operation {
    Add,
    Mul
}

#[derive(Debug, Clone)]
struct Value {
    label: String,
    data: i64,
    operation: Option<Operation>,
    children: Option<Vec<String>>
}


impl Value {
    // Initializing new value 
    fn new(data: i64, label: String) -> Self {
        Self { data, label, operation: Option::None, children: Option::None }
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
            children: Some(vec![self.label, other.label])
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
            children: Some(vec![self.label, other.label])
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
    let first_value = Value::new(5, "Hello".to_owned());
    let second_value = Value::new(10, "World".to_owned());

    println!("This is value A: {:?}", first_value.clone());        
    println!("This is value B: {:?}", second_value.clone());

    let added_values = first_value.clone() + second_value.clone();

    println!("This is the sum of those two values: {:?}", added_values.data);
    println!("The operation performed was: {:?}", added_values.operation);
    println!("The children of this node is: {:?}", added_values.children);

    let my_graph = ValueGraph::new(vec![first_value.clone(), second_value.clone(), added_values]);
    println!("This is my graph: {:?}", my_graph.values);
}
