enum Employee{
    Manager { name: String, subordinates: Vec<Box<Employee>> },
    Worker { name: String, manager: String },
}

fn main() {
    // Create workers
    let worker1 = Employee::Worker { name: "Bob".to_string(), manager: "Alice".to_string() };
    let worker2 = Employee::Worker { name: "Charlie".to_string(), manager: "Alice".to_string() };

    // Create manager with workers as subordinates
    let manager = Employee::Manager {
        name: "Alice".to_string(),
        subordinates: vec![Box::new(worker1), Box::new(worker2)],
    };

    match manager {
        Employee::Manager { name, subordinates } =>
            println!("{} is a manager with {} suboridates.", name, subordinates.len()),
        Employee::Worker { name, manager } =>
            println!("{} is a worker under the management of {}.", name, manager),
    }
}
