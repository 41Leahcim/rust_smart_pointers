use std::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
};

#[derive(Debug)]
struct ReferenceCounter<T>(T, usize);

pub struct SharedPointer<T>(ptr::NonNull<ReferenceCounter<T>>);

impl<T> SharedPointer<T> {
    fn allocate_memory() -> ptr::NonNull<ReferenceCounter<T>> {
        // Create a null pointer
        let mut pointer = ptr::null_mut();

        // Create a pointer to the null pointer, so a value can be assigned to it
        // Calculate the alignment for and get the size of the ReferenceCounter to store
        let address = (&mut pointer as *mut *mut ReferenceCounter<T>).cast();
        let alignment = mem::align_of::<ReferenceCounter<T>>().max(mem::size_of::<usize>());
        let amount_to_allocate = mem::size_of::<ReferenceCounter<T>>();

        // Allocate memory
        let error_code = unsafe { libc::posix_memalign(address, alignment, amount_to_allocate) };

        // Check for allocation errors
        match error_code {
            libc::EINVAL => panic!("Alignment incorrect"),
            libc::ENOMEM => panic!("No memory"),
            _ => (),
        }

        // Store the pointer in a non-null pointer and return it
        ptr::NonNull::new(pointer).unwrap()
    }

    pub fn new(value: T) -> Self {
        // Allocate memory
        let pointer = Self::allocate_memory();

        // Create a reference counter storing the value
        let reference_counter = ReferenceCounter(value, 1);

        // Store the reference counter at the address pointed to by the pointer
        unsafe { pointer.as_ptr().write(reference_counter) };

        // Store the pointer in a SharedPointer and return it
        Self(pointer)
    }
}

impl<T: Default> Default for SharedPointer<T> {
    fn default() -> Self {
        // Allocate memory
        let pointer = Self::allocate_memory();

        // Create a reference counter storing the default value of the type to store
        let reference_counter = ReferenceCounter(T::default(), 1);

        // Store the reference counter at the address pointed to by the pointer
        unsafe { pointer.as_ptr().write(reference_counter) };

        // Store the pointer in a SharedPointer and return it
        Self(pointer)
    }
}

impl<T> Clone for SharedPointer<T> {
    fn clone(&self) -> Self {
        // Increment the reference count
        unsafe {
            self.0.as_ptr().as_mut().unwrap().1 += 1;
        }

        // Copy the pointer to a new SharedPointer and return it
        Self(self.0)
    }
}

impl<T> Deref for SharedPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Return a reference to the value stored in the reference counter
        unsafe { &self.0.as_ref().0 }
    }
}

impl<T> DerefMut for SharedPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Return a mutable reference to the value stored in the reference counter
        unsafe { &mut self.0.as_mut().0 }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SharedPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write the SharedPointer as if the ReferenceCounter is stored in it
        f.write_fmt(format_args!("SharedPtr({:?})", unsafe { self.0.as_ref() }))
    }
}

impl<T> Drop for SharedPointer<T> {
    fn drop(&mut self) {
        // Get a mutable reference to the ReferenceCounter
        let reference_counter = unsafe { self.0.as_mut() };

        // Decrement the reference count
        reference_counter.1 -= 1;

        // Free the memory, if the reference count is 0
        if reference_counter.1 == 0 {
            unsafe { libc::free(self.0.as_ptr().cast()) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SharedPointer;

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
        let mut pointer = SharedPointer::new(value);

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

        // Get the reference count
        let reference_count = unsafe { pointer.0.as_ref().1 };

        // Check whether it is 1
        assert_eq!(reference_count, 1);
        {
            // Clone the Shared Pointer
            let cloned_pointer = pointer.clone();

            // Check whether the values and pointers are the same
            assert_eq!(*pointer, *cloned_pointer);
            assert_eq!(pointer.0, cloned_pointer.0);

            // Check whether the reference count is 2
            assert_eq!(reference_count, 2);
        }

        // Check whether the reference count is 1
        assert_eq!(reference_count, 1);
    }

    #[test]
    fn debug() {
        // Generate a random value
        let value = rand::random::<u8>();

        // Store it in a SharedPointer
        let pointer = SharedPointer::new(value);

        // Check whether the pointer is printed as expected
        assert_eq!(format!("{:?}", pointer), "UniquePointer(5)".to_owned());
    }
}
