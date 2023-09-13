
use core::time;
use std::{error::Error, fmt, thread};

use super::ScreenImage;

/// Operation to be commited in SAP gui
pub struct Operation<T> {
    /// `Predicate`s that must success before `action` is executed
    depends: Vec<Predicate>,

    /// function to be executed once `depends` are all successful
    action: Box<dyn FnOnce(T) -> ()>
}

impl<T> Operation<T> {
    /// create a new operation
    pub fn new(depends: Vec<Predicate>, action: Box<dyn FnOnce(T) -> ()>) -> Self {
        Self { depends, action }
    }

    /// execute operation
    /// 
    /// Checks is `Predicate`s pass.
    /// Once they all pass, `action` is called
    pub fn exec(self, arg: T) -> Result<(), Box<dyn Error>> {
        // wait for all predicates to pass
        while let false = self.depends.iter().all(Predicate::test) {
            thread::sleep(time::Duration::from_millis(250));
        }

        // call the action
        (self.action)(arg);

        Ok(())
    }
}

impl<T> fmt::Debug for Operation<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operation")
            .field("depends", &self.depends)
            .field("action", &format!("FnOnce({})", std::any::type_name::<T>()))
            .finish()
    }
}


#[derive(Debug)]
pub enum Predicate {
    Img(ScreenImage)
}

impl Predicate {
    pub fn test(&self) -> bool {
        match self {
            Predicate::Img(image) => image.is_visible(),
        }
    }
}
