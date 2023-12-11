
//! process for testing

use crate::{api, overlay::Overlay};
use std::{sync::mpsc, thread};

/// process for testing purposes
#[derive(Debug)]
pub struct Test {}

impl Test {
    /// run test
    pub fn exec() {
        let mut cache = api::ImageCache::new();
        let img = api::ScreenImage::new_sized("test_img", api::Coord::new(10, 10), 200, 100);
    
        cache.add(img.clone());
        cache.save();

        let mut visible = false;
        loop {
            let vis = img.is_visible();
            if visible != vis {
                println!("Image changed visibility -> {}", vis);
                visible = vis;
            }
        }
    
        // let (tx, rx) = mpsc::sync_channel(8);
        // let receiver = thread::spawn(move || {
        //     let mut result = None;
        //     while let Ok(msg) = rx.recv() {
        //         result = msg;
        //         println!("{:?}", result);
        //     }

        //     result
        // });
        // Overlay::run(tx);

        // let rect = receiver
        //     .join()
        //     .unwrap_or(None);

        // println!("{:?}", rect);
    }
}