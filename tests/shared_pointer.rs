use std::cell::RefCell;

use smart_pointers::SharedPointer;

#[test]
fn pointer_creation() {
    // Generate a random value
    let value = rand::random::<i32>();

    // store it in a SharedPointer
    let pointer = SharedPointer::new(value);

    // check whether the value was stored correctly
    assert_eq!(*pointer, value);
}

#[test]
fn pointer_manipulation() {
    // Generate a random value
    let mut value = rand::random::<i64>();

    // Store it in a SharedPointer
    let pointer = SharedPointer::new(RefCell::new(value));

    // Take a reference to the value in the RefCell
    let mut reference = pointer.borrow_mut();

    // Check whether it was stored correctly
    assert_eq!(*reference, value);

    // Generate a new value and store it in the pointer
    value = rand::random();
    *reference = value;

    // Check whether the value was changed correctly
    assert_eq!(*reference, value);
}

#[test]
fn default() {
    // Get the default value of a type
    let value = u16::default();

    // Create a default SharedPointer to the same type
    let pointer = SharedPointer::<u16>::default();

    // Check whether the values are the same
    assert_eq!(*pointer, value);
}

#[test]
fn cloning() {
    // Generate a random value
    let value = rand::random::<f64>();

    // Store it in a SharedPointer
    let pointer = SharedPointer::new(value);

    // Clone the Shared Pointer
    let cloned_pointer = pointer.clone();

    // Check whether the values are the same
    assert_eq!(*pointer, *cloned_pointer);
}

#[test]
fn debug() {
    // Generate a random value
    let value = rand::random::<u8>();

    // Store it in a SharedPointer
    let pointer = SharedPointer::new(value);

    // Check whether the pointer is printed as expected
    assert_eq!(format!("{:?}", pointer), format!("SharedPointer({value})"));
}
