
//! screen overlay for capturing mouse input

use std::sync::mpsc;

use speedy2d::{
    color::Color,
    dimen::{Vec2, UVec2},
    font::{Font, TextLayout, TextOptions},
    Graphics2D,
    shape::Rectangle,
    window::{WindowHandler, WindowHelper, WindowCreationOptions, MouseButton, VirtualKeyCode},
    Window,
};

/// Overlay for capturing input
#[derive(Debug)]
pub struct Overlay {}

impl Overlay {
    /// runs overlay loop
    pub fn run(tx: mpsc::SyncSender<Option<Rectangle>>) {
        let overlay = OverlayWindowHandler::new(tx);

        Window::<()>::new_with_options("mouse_input_overlay",
            WindowCreationOptions::new_fullscreen_borderless()
                .with_always_on_top(true)
                .with_transparent(true)
        )
            .expect("failed to create overlay window")
            .run_loop(overlay);
    }
}

#[derive(Debug)]
enum Graphic {
    Point(Vec2),
    Rectangle(Rectangle)
}

#[derive(Debug)]
struct OverlayWindowHandler {
    mouse_position: Option<Vec2>,
    window_size: Option<UVec2>,
    
    graphic: Option<Graphic>,
    results: mpsc::SyncSender<Option<Rectangle>>
}

impl OverlayWindowHandler {
    fn new(results: mpsc::SyncSender<Option<Rectangle>>) -> Self {
        Self {
            results,
            
            mouse_position: None,
            window_size: None,
            graphic: None,
        }
    }
    fn draw_help(&mut self, graphics: &mut Graphics2D) {
        let font_size = 32.;

        let bytes = include_bytes!("../assets/NotoSans-Regular.ttf");
        let font = Font::new(bytes).unwrap();
        let accept_block = font.layout_text("right-click to accept", font_size, TextOptions::new());
        let reject_block = font.layout_text("press Escape to quit",  font_size, TextOptions::new());

        let coord = match self.window_size {
            Some(uvec) => uvec.into_f32(),
            None => Vec2 { x: 400., y: 400. }
        };

        // quit block text
        let x = (coord.x - reject_block.width() ) / 2.;
        let y = (coord.y - reject_block.height()) / 2.;
        graphics.draw_text((x, y), Color::BLUE, &reject_block);

        // accept block text
        if let Some(Graphic::Rectangle(_)) = &self.graphic {
            let x = (coord.x - accept_block.width() ) / 2.;
            let y = (coord.y - accept_block.height()+font_size) / 2.;
            graphics.draw_text((x, y), Color::BLUE, &accept_block);
        }
    }
}

impl WindowHandler for OverlayWindowHandler {
    fn on_start(
            &mut self,
            _helper: &mut WindowHelper<()>,
            info: speedy2d::window::WindowStartupInfo
        ) {
        self.window_size = Some(info.viewport_size_pixels().clone());
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgba(0.26, 0.30, 0.37, 0.25));

        self.draw_help(graphics);

        if let Some(graphic) = &self.graphic {
            let (rect, color) = match graphic {
                Graphic::Point(pt) => (
                    Rectangle::new(pt.clone(), self.mouse_position.unwrap()),
                    Color::from_rgba(0., 1., 0., 0.25)
                ),
                Graphic::Rectangle(rect) => (rect.clone(), Color::from_rgba(1., 0., 0., 0.25)),
            };

            graphics.draw_line(rect.top_left(), rect.top_right(), 1., Color::BLACK);
            graphics.draw_line(rect.top_right(), rect.bottom_right(), 1., Color::BLACK);
            graphics.draw_line(rect.bottom_right(), rect.bottom_left(), 1., Color::BLACK);
            graphics.draw_line(rect.bottom_left(), rect.top_left(), 1., Color::BLACK);

            graphics.draw_rectangle(rect, color);
        }

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    fn on_key_down(
            &mut self,
            helper: &mut WindowHelper<()>,
            virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
            _scancode: speedy2d::window::KeyScancode
        ) {
        if let Some(VirtualKeyCode::Escape) = virtual_key_code {
            let _ = self.results.send(None);    // clear results
            helper.terminate_loop();
        }
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        match button {
            MouseButton::Left => {
                if let Some(pos) = self.mouse_position {
                    self.graphic = match self.graphic {
                        Some(Graphic::Point(p)) => Some(Graphic::Rectangle(Rectangle::new(p, pos))),
                        // either None or Some(Graphic::Rectangle(_)) -> start drawing
                        _ => Some(Graphic::Point(pos)),
                    };

                    if let Some(Graphic::Rectangle(rect)) = &self.graphic {
                        let _ = self.results.send(Some(rect.clone()));
                    }
                }
            },
            MouseButton::Right => {
                if let Some(Graphic::Rectangle(_)) = &self.graphic {
                    helper.terminate_loop()
                }
            },
            _ => ()
        }
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vec2) {
        self.mouse_position = Some(position);
    }
    fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.window_size = Some(size_pixels);
    }
}

