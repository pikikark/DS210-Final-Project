pub struct MyError {
    message: String,
}

impl MyError {
    pub fn new(message: &str) -> MyError {
        MyError {
            message: message.to_string(),
        }
    }
}

pub fn common_utility_function() -> Result<(), MyError> {
    // Implementation
    Ok(())
}

pub const GLOBAL_CONSTANT: u32 = 42;

// Example of a generic trait for extending functionality
pub trait MyExtensionTrait {
    fn extended_function(&self);
}

// Implementing the trait for a generic type (as an example)
impl<T> MyExtensionTrait for T {
    fn extended_function(&self) {
        // Implementation
    }
}
