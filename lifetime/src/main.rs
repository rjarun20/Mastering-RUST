//Reference counted variables
use std::rc::Rc;
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let mut data = String::from("Hello, world!");

    // Borrowing data
    let borrowed_data = borrow_data(&data);
    println!("Borrowed data: {}", borrowed_data);

    // Modifying data
    modify_data(&mut data);
    println!("Modified data: {}", data);

    let data = String::from("Hello, world!");

    // Passing data to the function
    print_data(data);

    // Error: data has been moved
    // println!("Data: {}", data);

    let boss = Person {
        name: String::from("Elon Musk"),
    };

    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };

    println!("{} is the CEO of {}", tesla.ceo.name, tesla.name);

    
    let ceo_name = &boss.name;
    println!("CEO: {}", ceo_name);

    
    let ceo_name = &tesla.ceo.name;
    println!("CEO: {}", ceo_name);

    let name = "Elon Musk";
    let person = new_person::new(name);
    println!("Name: {}", person.name);
    

    let data = Rc::new(String::from("Hello, world!"));

    // Cloning the reference
    let cloned_data = Rc::clone(&data);
    println!("Data: {}", cloned_data);

    // Dropping the reference
    drop(data);

    // Error: value used here after move
    // println!("Data: {}", data);

    rc_demo();

    // arc_demo();

    mutex_demo();

    
}

fn borrow_data(data: &String) -> &str {
    // Returning a borrowed reference
    &data[..]
}

fn modify_data(data: &mut String) {
    // Modifying the borrowed data
    data.push_str(" Have a nice day!");
}

fn print_data(data: String) {
    // Printing the data
    println!("Data: {}", data);
}

struct Person {
    name: String,
}

struct Company<'a> {
    name: String,
    ceo: &'a Person,
}

struct new_person <'a> {
    name: &'a str,
}

impl <'a> new_person <'a> {
    fn new(name: &'a str) -> new_person<'a> {
        new_person {
            name: name,
        }
    }
}


struct rc_person {
    name: Rc<String>,
}

impl rc_person {
    fn new(name: Rc<String>) -> rc_person {
        rc_person {
            name: name,
        }
    }

    fn rc_greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn rc_demo() {
    let name = Rc::new(String::from("Elon Musk"));
    println!("Name {} has Reference count: {}", name, Rc::strong_count(&name));
    {
        let person = rc_person::new(name.clone());
        person.rc_greet();
        println!("Name: {}", name);
        println!("Name {} has Reference count: {}", name, Rc::strong_count(&name));
    }
    println!("Name {} has Reference count: {}", name, Rc::strong_count(&name));


}

//ARC - Atomic reference counted variables
struct arc_person {
    name: Arc<String>,
    state: Arc<Mutex< String>>,
}

impl arc_person {
    fn new(name: Arc<String>, state : Arc<Mutex<String>>) -> arc_person {
        arc_person {
            name: name,
            state: state
        }
    }

    fn arc_greet(&self) {
        println!("Hello, {}!", self.name);
        let mut state = self.state.lock().unwrap();
        state.clear();
        *state = String::from("Happy");
        println!("hi, my name is {} and i am State: {}",self.name, *state);

        // self.state.clear();

        // self.state.push_str("Happy");
    }
}

// fn arc_demo() {
//     let name = Arc::new(String::from("Elon Musk"));
//     let person = arc_person::new(name.clone());

//     let handle = thread::spawn(move || {
//         person.arc_greet();
//     });

//     handle.join().unwrap();
//     println!("Name: {}", name);


// }

//mutex demo


fn mutex_demo() {
    let name = Arc::new(String::from("Elon Musk"));
    let state = Arc::new(Mutex::new(String::from("Happy")));
    let person = arc_person::new(name.clone(), state.clone());

    let handle = thread::spawn(move || {
        person.arc_greet();
    });

    println!("Name = {} , State = {}", name, state.lock().unwrap().as_str());

    handle.join().unwrap();
    println!("Name: {}", name);
    println!("State: {}", state.lock().unwrap());

    
}