extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod player;
mod vector2;
mod settings;

use glutin_window::GlutinWindow as Window;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Key, ReleaseEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::Button::Keyboard;
use piston_window::PressEvent;
use player::{Action, KeyState, Player, State};
use vector2::Vector2;
use crate::settings::color;

fn main() {
    let (width, height) = settings::window::SIZE;
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("square", [width, height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut gl = GlGraphics::new(opengl);
    let mut player = Player {
        position: Vector2 {
            x: width as f64 / 2.0,
            y: height as f64 / 2.0,
        },
        horizontal: State::Active(Action::NoMove),
        vertical: State::Active(Action::NoMove),
    };

    // Key state
    let mut up_key = KeyState::NotPressed;
    let mut down_key = KeyState::NotPressed;
    let mut left_key = KeyState::NotPressed;
    let mut right_key = KeyState::NotPressed;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(press_args) = e.press_args() {
            match press_args {
                Keyboard(Key::W) => up_key = KeyState::Pressed,
                Keyboard(Key::S) => down_key = KeyState::Pressed,
                Keyboard(Key::A) => left_key = KeyState::Pressed,
                Keyboard(Key::D) => right_key = KeyState::Pressed,
                _ => (),
            }
            player.input(left_key, right_key, up_key, down_key);
        }

        if let Some(release_args) = e.release_args() {
            match release_args {
                Keyboard(Key::W) => up_key = KeyState::NotPressed,
                Keyboard(Key::S) => down_key = KeyState::NotPressed,
                Keyboard(Key::A) => left_key = KeyState::NotPressed,
                Keyboard(Key::D) => right_key = KeyState::NotPressed,
                _ => (),
            }
            player.input(left_key, right_key, up_key, down_key);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(color::GREY, gl);
                player.draw(c, gl);
            });
        }

        if let Some(args) = e.update_args() {
            player.update(args.dt);
        }
    }
}