use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::{thread, time};
use vek::vec::Vec2;

#[derive(Clone, PartialEq)]
enum Color {
    Black,
    White,
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
}

impl Color {
    pub fn get_hex(&self) -> u32 {
        use Color::*;

        match self {
            Black => 0x00000000,
            White => 0x00ffffff,
            Red => 0x00ff0000,
            Yellow => 0x00ffff00,
            Green => 0x0000ff00,
            Cyan => 0x0000ffff,
            Blue => 0x000000ff,
            Purple => 0x00ff00ff,
        }
    }

    pub fn get_rgb(&self) -> (u8, u8, u8) {
        use Color::*;

        match self {
            Black => (0, 0, 0),
            White => (255, 255, 255),
            Red => (255, 0, 0),
            Yellow => (255, 255, 0),
            Green => (0, 255, 0),
            Cyan => (0, 255, 255),
            Blue => (0, 0, 255),
            Purple => (255, 0, 255),
        }
    }

    pub fn rotate_color(&mut self, advance: i32) {
        use Color::*;

        if advance > 0 {
            for _i in 0..advance {
                *self = match self {
                    Black => White,
                    White => Red,
                    Red => Yellow,
                    Yellow => Green,
                    Green => Cyan,
                    Cyan => Blue,
                    Blue => Purple,
                    Purple => Black,
                }
            }
        } else if advance < 0 {
            for _i in advance..0 {
                *self = match self {
                    Black => Purple,
                    White => Black,
                    Red => White,
                    Yellow => Red,
                    Green => Yellow,
                    Cyan => Green,
                    Blue => Cyan,
                    Purple => Blue,
                }
            }
        }
    }
}

struct PixelBuffer {
    size: Vec2<usize>,
    pixels: Vec<u32>,
}

impl PixelBuffer {
    pub fn new(size: Vec2<usize>) -> PixelBuffer {
        PixelBuffer {
            size,
            pixels: vec![0; size.x * size.y],
        }
    }

    pub fn rectangle(&mut self, start: Vec2<usize>, size: Vec2<usize>, color: u32) {
        let end = Vec2::new(start.x + size.x, start.y + size.y);

        for y in start.y..end.y {
            for x in start.x..end.x {
                self.pixels[(y * self.size.x) + x] = color;
            }
        }
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.pixels
    }
}

fn main() {
    let target_framerate = 60;
    let target_frametime = time::Duration::from_secs_f64(1.0 / target_framerate as f64);

    let pix_rendering_size: Vec2<usize> = Vec2::new(5, 5);
    let image_size: Vec2<usize> = Vec2::new(100, 100);
    let color_indicator_size: usize = 20;
    let window_size = Vec2::new(
        pix_rendering_size.x * image_size.x + color_indicator_size,
        pix_rendering_size.y * image_size.y,
    );
    
    let mut pixs = vec![Color::Black; image_size.x * image_size.x];
    let mut brush_color = Color::White;

    let mut pending_window_update = true;
    let mut buffer: PixelBuffer = PixelBuffer::new(window_size);

    let mut window = match Window::new(
        "pix_paint",
        window_size.x,
        window_size.y,
        WindowOptions {
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            panic!("Unable to create window: {}", err);
        }
    };

    // The main rendering and input loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let render_start = time::Instant::now();

        if window.get_mouse_down(MouseButton::Left) {
            window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
                let pos = Vec2::new(
                    x as usize / pix_rendering_size.x,
                    y as usize / pix_rendering_size.y,
                ); // the integer position among the pixels
    
                pixs[pos.y * image_size.x + pos.x] = brush_color.clone();

                buffer.rectangle(
                    Vec2::new(
                        pos.x * pix_rendering_size.x,
                        pos.y * pix_rendering_size.y,
                    ),
                    pix_rendering_size,
                    brush_color.get_hex(),
                );

                pending_window_update = true;
            
            });
        }

        window.get_scroll_wheel().map(|scroll| {
            let scroll = scroll.0 as i32 + scroll.1 as i32; // adding to each other to support horizontal scrolling because why not?

            brush_color.rotate_color(scroll / 2); // divided by two because -- on my mouse at least -- one scroll notch is 2.

            buffer.rectangle(
                Vec2::new(window_size.x - color_indicator_size, 0),
                Vec2::new(color_indicator_size, window_size.y),
                brush_color.get_hex(),
            );

            pending_window_update = true;
        });

        let elapsed_render_time = time::Instant::now() - render_start;

        let next_update_in = target_frametime - elapsed_render_time;

        if pending_window_update {
            if next_update_in > time::Duration::from_secs(0) {
                thread::sleep(next_update_in);

                window.update_with_buffer(buffer.get_buffer()).unwrap();
            } else {
                // We unwrap here as we want this code to exit if it fails
                window.update_with_buffer(buffer.get_buffer()).unwrap();
            }

            pending_window_update = false;
        } else {
            window.update();
            if next_update_in > time::Duration::from_secs(0) {
                thread::sleep(next_update_in);
                println!("{:?}", next_update_in);
            }
        }

        if window.is_key_down(Key::LeftCtrl) && window.is_key_down(Key::S)  {
            let mut img = bmp::Image::new(image_size.x as u32, image_size.y as u32);

            for y in 0..image_size.y {
                for x in 0..image_size.x {
                    let color = pixs[y * image_size.x + x].get_rgb();

                    img.set_pixel(
                        x as u32,
                        y as u32,
                        bmp::Pixel::new(color.0, color.1, color.2),
                    );
                }
            }

            img.save("save.bmp").unwrap();
        }
    }
}