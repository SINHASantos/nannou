// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Exercise 2-10 Attract Repel
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


struct Mover {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    mass: f32,
}

// A type for a draggable attractive body in our world
struct Attractor {
    mass: f32, // Mass, tied to size
    radius: f32, // Radius of the attractor
    position: Point2<f32>, // position
    dragging: bool, // Is the object being dragged?
    roll_over: bool, // Is the mouse over the ellipse?
    drag: Vector2<f32>, // holds the offset for when the object is clicked on
}

impl Attractor {
    fn new(rect: Rect<f32>) -> Self {
        let position = rect.xy();
        let mass = 10.0;
        let radius = mass * 3.0;
        let drag= Vector2::new(0.0,0.0);
        let dragging = false;
        let roll_over = false;
        Attractor { position, mass, radius, drag, dragging, roll_over }
    }

    fn attract(&self, m: &Mover) -> Vector2<f32> {
        let mut force = self.position - m.position;             // Calculate direction of force
        let mut d = force.magnitude();                              // Distance between objects
        d = d.max(5.0).min(25.0);                               // Limiting the distance to eliminate "extreme" results for very cose or very far object
        force = force.normalize();                              // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let g = 1.0;
        let strength = (g * self.mass * m.mass) / (d * d); // Calculate gravitational force magnitude
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
            gray = 0.0;
        }
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .w_h(self.radius * 2.0, self.radius * 2.0)
            .rgb(gray,gray, gray);
    }

    // The methods below are for mouse interaction
    fn clicked(&mut self, mx: f32, my: f32) {
        let d = self.position.distance(Point2::new(mx, my));
        if d < self.mass {
            self.dragging = true;
            self.drag.x = self.position.x - mx;
            self.drag.y = self.position.y - my;
        }
    }

    fn rollover(&mut self, mx: f32, my: f32) {
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
            self.position.x = mx + self.drag.x;
            self.position.y = my + self.drag.y;
        }
    }
}

impl Mover {
    fn new(m: f32, x: f32, y: f32) -> Self {
        let mass = m;
        let position = Point2::new(x, y);
        let velocity = Vector2::new(0.0, 0.0);
        let acceleration = Vector2::new(0.0, 0.0);
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
            .w_h(self.mass * 2.0, self.mass * 2.0)
            .rgba(0.6, 0.6, 0.6, 0.5);
    }
    
    fn repel(&self, m: &Mover) -> Vector2<f32> {
        let mut force = self.position - m.position;                      // Calculate direction of force
        let mut distance = force.magnitude();                            // Distance between objects
        distance = distance.max(1.0).min(10000.0);                          // Limiting the distance to eliminate "extreme" results for very cose or very far object
        force = force.normalize();                                       // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let g = 1.0;
        let strength = (g * self.mass * m.mass) / (distance * distance); // Calculate gravitational force magnitude
        force * (-1.0* strength)                                                 // Get force vector --> magnitude * direction
    }

    fn check_edges(&mut self, rect: Rect<f32>) {
        if self.position.x < rect.left() {
            self.position.x = rect.left();
            self.velocity.x *= -1.0;
        }
        else if self.position.x > rect.right() {
            self.position.x = rect.right();
            self.velocity.x *= -1.0;
        }
        if self.position.y > rect.top() {
            self.position.y = rect.top();
            self.velocity.y *= -1.0;
        }
        else if self.position.y < rect.bottom() {
            self.position.y = rect.bottom();
            self.velocity.y *= -1.0;
        }
    }

}

struct Model {
    window: WindowId,
    movers: Vec<Mover>,
    attractor: Attractor,
}

fn model(app: &App) -> Model {
    let rect = Rect::from_wh(Vector2::new(640.0, 360.0));
    let window = app.new_window()
        .with_dimensions(rect.w() as u32, rect.h() as u32)
        .build()
        .unwrap();

    let movers = (0..20)
        .map(|_| {
            Mover::new(
                map_range(random(), 0.0, 1.0, 4.0, 12.0),
                map_range(random(), 0.0, 1.0, rect.left(), rect.right()),
                map_range(random(), 0.0, 1.0, rect.top(), rect.bottom()),
            )
        })
        .collect();
    
    let attractor = Attractor::new(rect);

    Model {
        window,
        movers,
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
            m.attractor.drag(app.mouse.x, app.mouse.y);
            m.attractor.rollover(app.mouse.x, app.mouse.y);

            for i in 0..m.movers.len() {
                for j in 0..m.movers.len() {
                    if i != j {
                        let force = m.movers[j].repel(&m.movers[i]);
                        m.movers[i].apply_force(force);
                    }
                }
                let force = m.attractor.attract(&m.movers[i]);
                m.movers[i].apply_force(force);
                m.movers[i].update();
            }
        }
        _ => (),
    }
    m
}

fn view(app: &App, m: &Model, frame: Frame) -> Frame {
    app.main_window()
        .set_title("noc_2_20_exercise_attract_repel");

    // Begin drawing
    let draw = app.draw();
    draw.background().rgb(1.0, 1.0, 1.0);

    m.attractor.display(&draw);
    
    // Draw movers
    for mover in &m.movers {
        mover.display(&draw);
    }

    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
