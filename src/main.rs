use std::{
    env::args,
    sync::{Arc, Mutex},
    time::Instant,
};

use smart_pointers::{SharedPointer, UniquePointer};

fn unique_pointer_test(print: bool) {
    let pointer = UniquePointer::new(1);
    let pointer2 = UniquePointer::new(2);
    let pointer3 = pointer;

    let mut pointer4 = UniquePointer::new(Vec::with_capacity(10));
    pointer4.push(1);

    if print {
        println!("{}", *pointer2 + *pointer3);
    }
}

fn shared_pointer_test(print: bool) {
    let pointer = SharedPointer::new(1);
    let pointer2 = SharedPointer::new(2);
    let pointer3 = pointer.clone();

    let pointer4 = SharedPointer::new(Mutex::new(Vec::with_capacity(10)));
    pointer4.lock().unwrap().push(1);

    if print {
        println!("{}", *pointer2 + *pointer3);
    }
}

fn box_test(print: bool) {
    let pointer = Box::new(1);
    let pointer2 = Box::new(2);
    let pointer3 = pointer;

    let mut pointer4 = Box::new(Vec::with_capacity(10));
    pointer4.push(1);

    if print {
        println!("{}", *pointer2 + *pointer3);
    }
}

fn arc_test(print: bool) {
    let pointer = Arc::new(1);
    let pointer2 = Arc::new(2);
    let pointer3 = pointer.clone();

    let pointer4 = Arc::new(Mutex::new(Vec::<i32>::with_capacity(10)));
    pointer4.lock().unwrap().push(1);

    if print {
        println!("{}", *pointer2 + *pointer3);
    }
}

fn print_test_mark(test_name: &str) {
    println!("====== {test_name} ======");
}

fn memory_leak_test() {
    print_test_mark("Start box test");
    unique_pointer_test(true);
    print_test_mark("End box test");
    print_test_mark("Start Arc test");
    shared_pointer_test(true);
    print_test_mark("End Arc test");
}

fn test_performance(function: impl Fn(bool), name: &str) {
    let mut iterations = 0_u64;
    let start = Instant::now();
    while start.elapsed().as_secs() < 1 {
        function(false);
        iterations += 1;
    }
    println!("{name} iterations: {iterations}");
}

fn performance_test() {
    test_performance(unique_pointer_test, "Unique pointer");
    test_performance(shared_pointer_test, "Shared pointer");
    test_performance(box_test, "Box");
    test_performance(arc_test, "Arc");
}

fn main() {
    match args().nth(1) {
        Some(test) => match test.to_lowercase().as_str() {
            "memory" => memory_leak_test(),
            "performance" => performance_test(),
            _ => println!("Only valid modes are: [memory] [performance]"),
        },
        None => {
            memory_leak_test();
            performance_test();
        }
    }
}
