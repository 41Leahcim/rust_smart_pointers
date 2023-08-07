use smart_pointers::{SharedPointer, UniquePointer};

fn unique_pointer_test() {
    let pointer = UniquePointer::new(1);
    let pointer2 = UniquePointer::new(2);
    let pointer3 = pointer;

    let mut pointer4 = UniquePointer::new(Vec::with_capacity(10));
    pointer4.push(1);

    println!("{}", *pointer2 + *pointer3);
}

fn shared_pointer_test() {
    let pointer = SharedPointer::new(1);
    let pointer2 = SharedPointer::new(2);
    let pointer3 = pointer.clone();

    let mut pointer4 = SharedPointer::new(Vec::with_capacity(10));
    pointer4.push(1);

    println!("{}", *pointer2 + *pointer3);
}

fn print_test_mark(test_name: &str) {
    println!("====== {test_name} ======");
}

fn main() {
    print_test_mark("Start box test");
    unique_pointer_test();
    print_test_mark("End box test");
    print_test_mark("Start rc test");
    shared_pointer_test();
    print_test_mark("End rc test");
}
