use std::ops::{Deref, DerefMut};

pub struct StackProxy<T: Clone> {
    stack: Vec<T>,
}

impl<T: Clone> StackProxy<T> {
    pub fn new(initial: T) -> Self {
        Self {
            stack: vec![initial],
        }
    }
    pub fn push(&mut self) {
        let top = self.stack.last().unwrap().clone();
        self.stack.push(top);
    }
    pub fn pop(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }else {
            panic!("Cannot pop the last state from the stack");
        }
    }
}

impl<T: Clone> Deref for StackProxy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.stack.last().unwrap()
    }
}

impl<T: Clone> DerefMut for StackProxy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.stack.last_mut().unwrap()
    }
}

impl<T: Clone> Default for StackProxy<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}