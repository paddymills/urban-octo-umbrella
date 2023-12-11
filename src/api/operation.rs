
use core::time;
use std::{error::Error, fmt, thread};

use super::{ScreenImage, ImageCache};
use crate::transact::TransAct;

/// Operation to be commited in SAP gui
pub struct Operation<T> {
    /// `Predicate`s that must success before `action` is executed
    depends: Vec<Predicate>,

    /// function to be executed once `depends` are all successful
    action: Box<dyn Fn(T) -> ()>
}

impl<T> Operation<T> {
    /// create a new operation
    pub fn new(depends: Vec<Predicate>, action: Box<dyn Fn(T) -> ()>) -> Self {
        Self { depends, action }
    }

    /// execute operation
    /// 
    /// Checks is `Predicate`s pass.
    /// Once they all pass, `action` is called
    pub fn exec(&self, arg: T) -> Result<(), Box<dyn Error>> {
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

/// Predicate that an operation depends on
#[derive(Debug)]
pub enum Predicate {
    /// Image that must be visible
    Img(ScreenImage)
}

impl Predicate {
    /// create a Predicate::Img from a TransAct and name
    pub fn img(transact: TransAct, name: &str) -> Result<Self, String> {
        let mut cache = ImageCache::load();
        let img = cache.get_for_transact(transact, name)?;

        Ok(Self::Img(img.clone()))
    }

    /// test if an image predicate is visible
    pub fn test(&self) -> bool {
        match self {
            Predicate::Img(image) => image.is_visible(),
        }
    }
}
