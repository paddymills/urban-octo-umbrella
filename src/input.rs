
use std::{
    sync::{mpsc, atomic::{AtomicBool, Ordering}},
    time::Duration
};
use winput::{
    Action,
    Button,
    Mouse,
    message_loop::{Event, MessageLoopError},
};

use crate::api::Coord;

pub fn event_loop(tx: mpsc::Sender<EventLoopMessage>, should_exit: &AtomicBool) -> Result<(), MessageLoopError> {
    let receiver = winput::message_loop::start()?;

    let timeout = Duration::from_millis(500);

    loop {
        if let Some(event) = receiver.next_event_timeout(timeout) {
            match event {
                // Event::Keyboard { vk, action, .. } => (),
                Event::MouseButton {
                    action: Action::Release,
                    button: Button::Left
                } => {
                    if let Ok((x, y)) = winput::Mouse::position() {
                        let _ = tx.send(EventLoopMessage::MouseButtonClicked(x, y));
                    }
                }
                _ => ()
            }
        }

        if should_exit.fetch_or(false, Ordering::Relaxed) == true {
            break;
        }
    }

    Ok(())
}

/// performs a click at a location and then returns to the original position
pub fn click_pos_and_return(pos: Coord<i32>) {
    let original_pos = Mouse::position();

    match Mouse::set_position(pos.x, pos.y) {
        Ok(()) => {
            winput::send(winput::Vk::MouseLeft);
        },
        Err(_) => {
            eprintln!("Failed to select start of order text box. Please select it");
            // TODO: await click at location
        }
    }

    if let Ok((x, y)) = original_pos {
        let _ = Mouse::set_position(x, y);
    }
}

pub enum EventLoopMessage {
    MouseButtonClicked(i32, i32),
    
}
