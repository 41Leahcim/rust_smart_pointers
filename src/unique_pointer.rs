use core::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
};

extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;

pub struct UniquePointer<T>(ptr::NonNull<T>);

impl<T> UniquePointer<T> {
    fn allocate_memory() -> ptr::NonNull<T> {
        // Allocate memory
        let pointer = unsafe { libc::malloc(mem::size_of::<T>()) };

        // Panic if it failed
        if pointer.is_null() {
            panic!("No memory");
        }

        // Store the pointer in a non-null pointer and return it
        ptr::NonNull::<T>::new(pointer.cast()).unwrap()
    }

    pub fn new(value: T) -> Self {
        // Allocate memory
        let pointer = Self::allocate_memory();

        // Store the value at the address pointed to by the pointer
        unsafe { pointer.as_ptr().write(value) };

        // Store the pointer in a UniquePointer and return it
        Self(pointer)
    }
}

impl<T: Default> Default for UniquePointer<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone> Clone for UniquePointer<T> {
    fn clone(&self) -> Self {
        // Clone the value stored in the UniquePointer and use it to create a new one
        Self::new(self.deref().to_owned())
    }
}

#[cfg(not(feature = "alloc"))]
impl<T: Copy> Clone for UniquePointer<T> {
    fn clone(&self) -> Self {
        // Clone the value stored in the UniquePointer and use it to create a new one
        Self::new(*self.deref())
    }
}

impl<T> AsRef<T> for UniquePointer<T> {
    fn as_ref(&self) -> &T {
        // Cast the pointer to a reference and return it
        unsafe { self.0.as_ref() }
    }
}

impl<T> AsMut<T> for UniquePointer<T> {
    fn as_mut(&mut self) -> &mut T {
        // Cast the pointer to a mutable reference and return it
        unsafe { self.0.as_mut() }
    }
}

impl<T> Deref for UniquePointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for UniquePointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for UniquePointer<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Write the UniquePointer as if the value pointed to is stored in it
        f.write_fmt(format_args!("UniquePointer({:?})", self.as_ref()))
    }
}

impl<T> Drop for UniquePointer<T> {
    fn drop(&mut self) {
        // Get the pointer
        let pointer = self.0.as_ptr();
        unsafe {
            // Call the destructor of the pointed to value
            pointer.drop_in_place();

            // Free the memory
            libc::free(pointer.cast());
        };
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;
    use heapless::String;

    use super::UniquePointer;

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

        // Store it in a UniquePointer
        let pointer = UniquePointer::new(value);

        // Clone the pointer and check whether the values are the same
        assert!((*pointer - *pointer.clone()).abs() < 0.000_01);
    }

    #[test]
    fn debug() {
        // Generate a random value
        let value = rand::random::<u8>();

        // Store it in a UniquePointer
        let pointer = UniquePointer::new(value);

        // Write the debug format to a stack String
        let mut debug_output = String::<64>::new();
        write!(debug_output, "{pointer:?}").unwrap();

        // Write the value we expect to a stack string
        let mut expected_output = String::<32>::new();
        write!(expected_output, "UniquePointer({value})").unwrap();

        // Check whether the pointer is printed as expected
        assert_eq!(debug_output, expected_output);
    }
}
