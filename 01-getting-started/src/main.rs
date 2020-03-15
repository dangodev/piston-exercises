extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use hsl::HSL;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    hue: f64,       // Color for the rect
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let hue = self.hue;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        // Get color
        let rect_color = self.hue_to_rgb(hue);
        let bg_color = self.hue_to_rgb((hue + 180.0) % 360.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bg_color, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(rect_color, square, transform, gl);
        });
    }

    fn hue_to_rgb(&self, hue: f64) -> [f32; 4] {
        let next_color = HSL {
            h: hue,
            s: 1.0,
            l: 0.5,
        };
        let rgb = next_color.to_rgb();
        let r = rgb.0 as f32 / 255.0;
        let g = rgb.1 as f32 / 255.0;
        let b = rgb.2 as f32 / 255.0;
        return [r, g, b, 1.0];
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        // Increase hue by 30 points per second
        self.hue += (30.0 * args.dt) % 360.0;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        hue: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
