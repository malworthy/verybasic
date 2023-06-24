use crate::vm::ValueType;

// use font_kit::family_name::FamilyName;
// use font_kit::properties::Properties;
// use font_kit::source::SystemSource;
use minifb::{MouseMode, Scale, ScaleMode, Window, WindowOptions};
use raqote::{
    DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source, StrokeStyle, Transform,
};
const WIDTH: i32 = 400;
const HEIGHT: i32 = 400;

pub struct Graphics {
    draw_target: Option<DrawTarget>,
    width: i32,
    height: i32,
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            draw_target: None, // DrawTarget::new(WIDTH as i32, HEIGHT as i32),
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn init(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.draw_target = Some(DrawTarget::new(self.width, self.height));
        self.clear()
    }

    pub fn clear(&mut self) {
        if let Some(ref mut draw_target) = self.draw_target {
            draw_target.clear(SolidSource::from_unpremultiplied_argb(
                0xff, 0xff, 0xff, 0xff,
            ));
        }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, rgb: (u8, u8, u8)) {
        if let Some(ref mut draw_target) = self.draw_target {
            let mut pb = PathBuilder::new();
            pb.rect(x, y, width, height);

            let path = pb.finish();
            draw_target.fill(
                &path,
                &Source::Solid(SolidSource::from_unpremultiplied_argb(
                    0xff, rgb.0, rgb.1, rgb.2,
                )),
                &DrawOptions::new(),
            );
        }
    }

    pub fn show_window(&mut self) {
        if let Some(ref mut draw_target) = self.draw_target {
            let mut window = Window::new(
                "Very Basic",
                self.width as usize,
                self.height as usize,
                WindowOptions {
                    ..WindowOptions::default()
                },
            )
            .unwrap();
            let size = window.get_size();
            loop {
                window
                    .update_with_buffer(draw_target.get_data(), size.0, size.1)
                    .unwrap();

                if !window.is_open() {
                    break;
                }
            }
        };
    }
}
