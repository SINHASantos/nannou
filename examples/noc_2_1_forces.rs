// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 2-1: Forces
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
    mover: Mover,
}

struct Mover {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    mass: f32,
}

impl Mover {
    fn new(rect: Rect<f32>) -> Self {
        let position = Point2::new(rect.left() + 30.0,rect.top() - 30.0);
        let velocity = Vector2::new(0.0,0.0);
        let acceleration = Vector2::new(0.0,0.0);
        let mass = 1.0;
        Mover { position, velocity, acceleration, mass }
    }

    fn apply_force(&mut self, force: Vector2<f32>) {
        let f = force / self.mass; 
        self.acceleration += f;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    fn check_edges(&mut self, rect: Rect<f32>) {
        if self.position.x > rect.right() {
            self.position.x = rect.right();
            self.velocity.x *= -1.0;
        } else if self.position.x < rect.left() {
            self.velocity.x *= -1.0;
            self.position.x = rect.left();
        }
        if self.position.y < rect.bottom() {
            self.velocity.y *= -1.0;
            self.position.y = rect.bottom();
        } 
    }

    fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .w_h(48.0, 48.0)
            .rgb(0.3, 0.3, 0.3);
    }
}

fn model(app: &App) -> Model {
    let rect = Rect::from_wh(Vector2::new(640.0,360.0));
    let window = app.new_window().with_dimensions(rect.w() as u32,rect.h() as u32).build().unwrap();
    let mover = Mover::new(rect);
    Model { window, mover }
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
            let wind = Vector2::new(0.01,0.0);
            let gravity = Vector2::new(0.0,-0.1);
            m.mover.apply_force(wind);
            m.mover.apply_force(gravity);
            m.mover.update();
            m.mover.check_edges(app.window.rect());
        }
        _ => (),
    }
    m
}

fn view(app: &App, m: &Model, frame: Frame) -> Frame {
    app.main_window().set_title("noc_2_1_forces");

    // Begin drawing
    let draw = app.draw();
    draw.background().rgb(1.0, 1.0, 1.0);

    m.mover.display(&draw);

    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
