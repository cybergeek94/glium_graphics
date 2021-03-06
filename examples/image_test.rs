extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate image;
extern crate piston;
extern crate glutin_window;

use std::rc::Rc;
use std::cell::RefCell;
use glium::Surface;
use glium_graphics::{
    Flip, Glium2d, GliumGraphics, GliumWindow, Texture, TextureSettings
};
use piston::event_loop::*;
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use glutin_window::{ GlutinWindow, OpenGL };

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (300, 300);
    let ref window: Rc<RefCell<GlutinWindow>> = Rc::new(RefCell::new(
        WindowSettings::new("glium_graphics: image_test", [w, h])
        .exit_on_esc(true).build().unwrap()
    ));
    let ref mut glium_window = GliumWindow::new(window).unwrap();
    let rust_logo = Texture::from_path(glium_window, "assets/rust.png",
        Flip::None, &TextureSettings::new()).unwrap();

    let mut g2d = Glium2d::new(opengl, glium_window);
    let transform = graphics::math::abs_transform(w as f64, h as f64);

    let mut events = window.borrow().events().swap_buffers(false);
    // Temporary fix for https://github.com/rust-lang/rust/issues/30832.
    while let Some(e) = { let mut b = window.borrow_mut(); events.next(&mut *b) } {
        use graphics::*;

        if let Some(_) = e.render_args() {
            let mut target = glium_window.draw();
            {
                let mut g = GliumGraphics::new(&mut g2d, &mut target);

                clear(color::WHITE, &mut g);
                rectangle([1.0, 0.0, 0.0, 1.0],
                          [0.0, 0.0, 100.0, 100.0],
                          transform,
                          &mut g);
                rectangle([0.0, 1.0, 0.0, 0.3],
                          [50.0, 50.0, 100.0, 100.0],
                          transform,
                          &mut g);
                image(&rust_logo, transform.trans(100.0, 100.0), &mut g);
            }
            target.finish().unwrap();
        }

    }
}
