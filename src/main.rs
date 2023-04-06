use std::ops;

#[derive(Debug)]
enum Operation {
    Add,
    Mul
}

#[derive(Debug)]
struct Value {
    label: String,
    data: i64,
    operation: Option<Operation>,
    parents: Option<Vec<String>>
}

impl Value {
    // Initializing new value 
    fn new(data: i64, label: String) -> Self {
        Self { data, label, operation: Option::None, parents: Option::None }
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
            parents: Some(vec![self.label, other.label])
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
            parents: Some(vec![self.label, other.label])
        }
    }
}

fn main() {
    let first_value = Value::new(5, "Hello".to_owned());
    let second_value = Value::new(10, "World".to_owned());

    println!("This is value A: {:?}", first_value);        
    println!("This is value B: {:?}", second_value);

    let added_values = first_value + second_value;

    println!("This is the sum of those two values: {:?}", added_values.data);
    println!("The operation performed was: {:?}", added_values.operation);
    println!("The parents of this node is: {:?}", added_values.parents);
    println!("Hello, world!");
}
