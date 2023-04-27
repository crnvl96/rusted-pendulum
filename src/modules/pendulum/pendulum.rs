use crate::helpers::vector::vector::Vector;
use s::color::Color;
use s::Graphics2D;
use speedy2d as s;

pub struct Pendulum {
    origin: Vector,
    position: Vector,
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,
    radius: f32,
    gravity: f32,
}

impl Pendulum {
    pub fn new(x: f32, y: f32, radius: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            radius,
            gravity: 1.5,
        }
    }

    pub fn update(&mut self) {
        self.angular_acceleration = -1.0 * self.gravity * self.angle.sin() / self.radius;
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
        self.position.set(
            self.radius * self.angle.sin(),
            self.radius * self.angle.cos(),
        );
        self.position.add(&self.origin);
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED)
    }
}
