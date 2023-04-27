use crate::modules::pendulum::pendulum::Pendulum;
use s::color::Color;
use s::window::{WindowHandler, WindowHelper};
use s::Graphics2D;
use speedy2d as s;

pub struct LocalWindowHandler {
    pub pendulum: Pendulum,
}

impl WindowHandler for LocalWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.pendulum.update();
        self.pendulum.draw(graphics);
        helper.request_redraw();
    }
}
