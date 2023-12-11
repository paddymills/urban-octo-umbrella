
//! CO02: Delete an operation from an order

use std::error::Error;

use winput::Vk;

use crate::{
    api::{Operation, Coord},
    input
};

/// operations pertaining to the MATLCONS operation
#[derive(Debug)]
pub struct Matlcons {
    orders: Vec<u32>
}

impl Matlcons {
    /// delete the `MATLCONS` operation
    pub fn delete(&self) -> Result<(), Box<dyn Error>> {
        let ops = vec![
            Operation::new(
                vec![],
                Box::new(|order: u32| {
                    // select order location
                    let loc = Coord::new(0, 0);
    
                    input::click_pos_and_return(loc);
                    winput::send_str(&order.to_string());
                    winput::send(Vk::F5);
                })
            )
        ];

        let _ = self.orders.iter()
            .map(|order| {
                ops.iter()
                    // TODO: propegate error instad of unwrap
                    .map(|op| op.exec(*order).unwrap())
            });
    
        Ok(())
    }
}