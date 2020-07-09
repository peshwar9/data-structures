// Implementation of Stack data structure in Rust.
// APIs for the stack: push(), pop(), len() and peek()

//Define data structure for stack

#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<T>,
}

// Implement methods on Stack struct
impl<T> Stack<T> {
    // Creates  a new Stack
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // Pops last element out of stack
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Pushes new item to top of stack
    pub fn push(&mut self, data: T) {
        self.elements.push(data)
    }
    //Checks if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    // Returns a reference to the last item (on top) in the stack without actually removing it from stack
    pub fn peek(&self) -> Option<&T> {
        self.elements.get(self.elements.len() - 1)
    }
}
