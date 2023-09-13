
use core::time;
use std::{error::Error, thread};

use super::ScreenImage;

/// Operation to be commited in SAP gui
pub struct Operation<T> {
    depends: Vec<Predicate>,
    action: Box<dyn FnOnce(T) -> ()>
}

impl<T> Operation<T> {
    pub fn new(depends: Vec<Predicate>, action: Box<dyn FnOnce(T) -> ()>) -> Self {
        Self { depends, action }
    }

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
