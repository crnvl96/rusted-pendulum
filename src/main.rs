mod crates;
mod helpers;
mod modules;

use s::Window;
use speedy2d as s;

use crates::draw::draw::LocalWindowHandler;
use modules::pendulum::pendulum::Pendulum;

fn main() {
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

    let win = LocalWindowHandler {
        pendulum: Pendulum::new(400.0, 0.0, 200.0),
    };

    window.run_loop(win)
}
