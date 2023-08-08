use std::{env::args, time::Instant};

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

    let mut pointer4 = SharedPointer::new(Vec::with_capacity(10));
    pointer4.push(1);

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
    print_test_mark("Start rc test");
    shared_pointer_test(true);
    print_test_mark("End rc test");
}

fn performance_test() {
    let mut unique_iterations = 0_u64;
    let mut start = Instant::now();
    while start.elapsed().as_secs() < 1 {
        unique_pointer_test(false);
        unique_iterations += 1;
    }
    println!("Unique pointer iterations: {unique_iterations}");

    let mut shared_iterations = 0_u64;
    start = Instant::now();
    while start.elapsed().as_secs() < 1 {
        unique_pointer_test(false);
        shared_iterations += 1;
    }
    println!("Shared pointer iterations: {shared_iterations}");
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
