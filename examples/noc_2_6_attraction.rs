// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 2-6: Attraction
extern crate nannou;

use nannou::prelude::*;
use nannou::app::Draw;
use nannou::geom::rect::Rect;
use nannou::rand::random;
use nannou::math::prelude::*;
use nannou::math::map_range;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    window: WindowId,
    mover: Mover,
    attractor: Attractor,
}

struct Mover {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    mass: f32,
}

// A type for a draggable attractive body in our world
struct Attractor {
    mass: f32, // Maxx, tied to size
    G: f32, // Gravitational Constant
    position: Point2<f32>, // position
    dragging: bool, // Is the object being dragged?
    roll_over: bool, // Is the mouse over the ellipse?
    drag_offset: Vector2<f32>, // holds the offset for when the object is clicked on
}

impl Attractor {
    fn new(rect: Rect<f32>) -> Self {
        let position = rect.xy();
        let mass = 20.0;
        let G = 1.0;
        let drag_offset = Vector2::new(0.0,0.0);
        let dragging = false;
        let roll_over = false;
        Attractor { position, mass, G, drag_offset, dragging, roll_over }
    }

    fn attract(&self, m: &Mover) -> Vector2<f32> {
        let mut force = self.position - m.position;             // Calculate direction of force
        let mut d = force.magnitude();                              // Distance between objects
        d = d.max(5.0).min(25.0);                               // Limiting the distance to eliminate "extreme" results for very cose or very far object
        force = force.normalize();                              // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let strength = (self.G * self.mass * m.mass) / (d * d); // Calculate gravitational force magnitude
        force * strength                                        // Get force vector --> magnitude * direction
    }

    // Method to display
    fn display(&self, draw: &Draw) {
        let mut gray = 0.5;
        if self.dragging {
            gray = 0.2;
        } else if self.roll_over {
            gray = 0.4;
        } else {
            gray = 0.75;
        }
        draw.rect()
            .x_y(self.position.x, self.position.y)
            .w_h(self.mass * 2.0, self.mass * 2.0)
            .rgba(gray,gray, gray, 0.8);
    }

    // The methods below are for mouse interaction
    fn clicked(&mut self, mx: f32, my: f32) {
        let d = self.position.distance(Point2::new(mx, my));
        if d < self.mass {
            self.dragging = true;
            self.drag_offset.x = self.position.x - mx;
            self.drag_offset.y = self.position.y - my;
        }
    }

    fn hover(&mut self, mx: f32, my: f32) {
        let d = self.position.distance(Point2::new(mx, my));
        if d < self.mass {
            self.roll_over = true;
        } else {
            self.roll_over = false;
        }
    }

    fn stop_dragging(&mut self) {
        self.dragging = false;
    }
    
    fn drag(&mut self, mx: f32, my: f32) {
        if self.dragging {
            self.position.x = mx + self.drag_offset.x;
            self.position.y = my + self.drag_offset.y;
        }
    }
}

impl Mover {
    fn new() -> Self {
        let position = Point2::new(80.0, 130.0);
        let velocity = Vector2::new(1.0, 0.0);
        let acceleration = Vector2::new(0.0, 0.0);
        let mass = 1.0;
        Mover {
            position,
            velocity,
            acceleration,
            mass,
        }
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

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .w_h(16.0, 16.0)
            .rgb(0.3, 0.3, 0.3);
    }

    fn check_edges(&mut self, rect: Rect<f32>) {
        if self.position.x > rect.right() {
            self.position.x = rect.left();
        } else if self.position.x < rect.left() {
            self.position.x = rect.right();
        }
        if self.position.y < rect.bottom() {
            self.velocity.y *= -1.0; 
            self.position.y = rect.bottom();
        }
    }
}

fn model(app: &App) -> Model {
    let rect = Rect::from_wh(Vector2::new(640.0, 360.0));
    let window = app.new_window()
        .with_dimensions(rect.w() as u32, rect.h() as u32)
        .build()
        .unwrap();

    let mover = Mover::new();
    let attractor = Attractor::new(rect);

    Model {
        window,
        mover,
        attractor,
    }
}

fn event(app: &App, mut m: Model, event: Event) -> Model {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => {
            match event {
                // MOUSE EVENTS
                MousePressed(_button) => {
                    m.attractor.clicked(app.mouse.x, app.mouse.y);
                }

                MouseReleased(_buttom) => {
                    m.attractor.stop_dragging(); 
                }
                _other => (),
            }
        }
        // update gets called just before view every frame
        Event::Update(_dt) => {
            let force = m.attractor.attract(&m.mover);
            m.mover.apply_force(force);
            m.mover.update();
            m.attractor.drag(app.mouse.x, app.mouse.y);
            m.attractor.hover(app.mouse.x, app.mouse.y);
        }
        _ => (),
    }
    m
}

fn view(app: &App, m: &Model, frame: Frame) -> Frame {
    app.main_window()
        .set_title("noc_2_6_attraction");

    // Begin drawing
    let draw = app.draw();
    draw.background().rgb(1.0, 1.0, 1.0);

    m.attractor.display(&draw);
    m.mover.display(&draw);
    
    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
