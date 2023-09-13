
use std::error::Error;

use winput::Vk;

use crate::{
    api::{Operation, Coord},
    input
};

pub fn delete_matlcons(order: u32) -> Result<(), Box<dyn Error>> {
    let _ops = vec![
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

    Ok(())
}