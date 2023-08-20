#![feature(type_name_of_val)]

use std::any::type_name_of_val;

use smart_pointers::UniquePointer;

#[test]
fn pointer_creation() {
    // Generate a random value
    let value = rand::random::<i32>();

    // store it in a UniquePointer
    let pointer = UniquePointer::new(value);

    // check whether the value was stored correctly
    assert_eq!(*pointer, value);
}

#[test]
fn pointer_manipulation() {
    // Generate a random value
    let mut value = rand::random::<i64>();

    // Store it in a UniquePointer
    let mut pointer = UniquePointer::new(value);

    // Check whether it was stored correctly
    assert_eq!(*pointer, value);

    // Generate a new value and store it in the pointer
    value = rand::random();
    *pointer = value;

    // Check whether the value was changed correctly
    assert_eq!(*pointer, value);
}

#[test]
fn default() {
    // Get the default value of a type
    let value = u16::default();

    // Create a default UniquePointer to the same type
    let pointer = UniquePointer::<u16>::default();

    // Check whether the values are the same
    assert_eq!(*pointer, value);
}

#[test]
fn cloning() {
    // Generate a random value
    let value = rand::random::<f64>();

    let value = vec![value];

    // Store it in a UniquePointer
    let pointer = UniquePointer::new(value);

    let cloned_pointer = pointer.clone();

    // Clone the pointer and check whether the values are the same
    assert_eq!(*pointer, *pointer.clone());

    // Without alloc, the clone method will be called on the vector instead of the pointer.
    #[cfg(not(feature = "alloc"))]
    assert_ne!(
        type_name_of_val(&pointer),
        type_name_of_val(&cloned_pointer)
    );

    #[cfg(feature = "alloc")]
    assert_eq!(
        type_name_of_val(&pointer),
        type_name_of_val(&cloned_pointer)
    );
}

#[test]
fn debug() {
    // Generate a random value
    let value = rand::random::<u8>();

    // Store it in a UniquePointer
    let pointer = UniquePointer::new(value);

    // Check whether the pointer is printed as expected
    assert_eq!(format!("{:?}", pointer), format!("UniquePointer({value})"));
}
