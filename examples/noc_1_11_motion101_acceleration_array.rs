// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 1-11: Motion 101 Acceleration Array
//
// Demonstration of the basics of motion with vector.
// A "Mover" object stores position, velocity, and acceleration as vectors
// The motion is controlled by affecting the acceleration
// (in this case towards the mouse)
extern crate nannou;

use nannou::prelude::*;
use nannou::app::Draw;
use nannou::geom::rect::Rect;
use nannou::rand::random;
use nannou::math::map_range;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    window: WindowId,
    movers: Vec<Mover>,
}

struct Mover {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    top_speed: f32,
}

impl Mover {
    fn new(rect: Rect<f32>) -> Self {
        let rand_x = map_range(random(), 0.0, 1.0, rect.left(), rect.right());
        let rand_y = map_range(random(), 0.0, 1.0, rect.top(), rect.bottom());
        let position = Point2::new(rand_x, rand_y);
        let velocity = Vector2::new(0.0, 0.0);
        let acceleration = Vector2::new(0.0, 0.0);
        let top_speed = 5.0;
        Mover {
            position,
            velocity,
            acceleration,
            top_speed,
        }
    }

    fn update(&mut self, mouse: Point2<f32>) {
        // Computer a vector that points from position to mouse
        self.acceleration = mouse - self.position;
        // Set magnitude of acceleration
        self.acceleration = self.acceleration.normalize() * 0.2;
        // Velocity chages according to acceleration
        self.velocity += self.acceleration;
        // Limit the velocity by top_speed
        self.velocity = vec2(
            self.velocity.x.min(self.top_speed),
            self.velocity.y.min(self.top_speed),
        );
        // Position changes velocity
        self.position += self.velocity;
    }

    fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .w_h(48.0, 48.0)
            .rgba(0.5, 0.5, 0.5,0.7);
    }
}

fn model(app: &App) -> Model {
    let rect = Rect::from_wh(Vector2::new(640.0, 360.0));
    let window = app.new_window()
        .with_dimensions(rect.w() as u32, rect.h() as u32)
        .build()
        .unwrap();

    let movers = (0..20).map(|_| Mover::new(rect)).collect();
    Model { window, movers }
}

fn event(app: &App, mut m: Model, event: Event) -> Model {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => {
            match event {
                // KEY EVENTS
                KeyPressed(_key) => {}

                // MOUSE EVENTS
                MouseReleased(_button) => {}

                _other => (),
            }
        }
        // update gets called just before view every frame
        Event::Update(_dt) => {
            for mover in &mut m.movers {
                mover.update(Point2::new(app.mouse.x, app.mouse.y));
            }
        }
        _ => (),
    }
    m
}

fn view(app: &App, m: &Model, frame: Frame) -> Frame {
    app.main_window()
        .set_title("noc_1_11_motion101_acceleration_array");

    // Begin drawing
    let draw = app.draw();
    draw.background().rgb(1.0, 1.0, 1.0);

    for mover in &m.movers {
        mover.display(&draw);
    }

    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
