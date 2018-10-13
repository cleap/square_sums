extern crate piston_window;

use piston_window::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = WIDTH;

struct Node {
    num: u32,
    links: Vec<u32>
}

impl Node {

    fn new(num: u32) -> Node {
        Node {
            num,
            links: Vec::new()
        }
    }

    fn connect(&mut self, other: u32) {
        // If the nodes sum to a square
        if  self.num != other &&
                ((self.num + other) as f64).sqrt().fract() == 0.0 &&
                self.num + other != 4 { // Because Elijah said so
            self.links.push(other);
            println!("Connecting {:02} and {:02}", self.num, other);
        }
    }

    fn generate_graph(number: u32) -> Vec<Node> {

        // Initialize nodes
        let mut nodes: Vec<Node> = Vec::new();
        for n in 1..number+1 {
            nodes.push(Node::new(n));
        }

        // Connect graph
        for node in &mut nodes {
            for other in 1..number+1 {
                node.connect(other);
            }
        }

        nodes
    }
}

// Generates the coordinates for a number in the list
fn coord_from_n(n: u32, maxn: u32, radius: f64) -> [f64; 2] {
    let scale: f64 = 2.0*std::f64::consts::PI / (maxn as f64);
    let xcoord: f64 = (n as f64 * scale).sin() * radius + WIDTH as f64 / 2.0;
    let ycoord: f64 = -(n as f64 * scale).cos() * radius + HEIGHT as f64 / 2.0;
    [xcoord, ycoord]
}

fn main() {

    // Initialize the window
    let mut window: PistonWindow =
        WindowSettings::new("Square Sums", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    // Initialize text stuff
    let ref font = "fonts/FreeMono.ttf";
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    // Other vars
    let mut number: u32 = 15;

    // Initialize nodes
    let mut nodes: Vec<Node> = Node::generate_graph(number);

    // Event handler
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        // Key events
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space || key == Key::Right || key == Key::Up {
                number += 1;
                nodes = Node::generate_graph(number);
            }
            else if key == Key::Backspace || key == Key::Left || key == Key::Down {
                if number > 0 {
                    number -= 1;
                    nodes = Node::generate_graph(number);
                }
            }
        }

        // Render
        if let Some(_r) = e.render_args() {

            let radius: f64 = 8.0 * number as f64;

            window.draw_2d(&e, |context, graphics| {

                clear([1.0, 1.0, 1.0, 1.0], graphics);

                for node in &nodes {
                    let [x1, y1] = coord_from_n(node.num-1, number, radius);
                    ellipse(
                        [1.0, 0.0, 0.0, 1.0],
                        ellipse::circle(x1, y1, 5.0),
                        context.transform,
                        graphics
                    );
                    for other in &node.links {
                        let [x2, y2] = coord_from_n(*other-1, number, radius);
                        line(
                            [1.0, 0.0, 0.0, 1.0],
                            1.0,
                            [x1, y1, x2, y2],
                            context.transform,
                            graphics
                         );
                    }
                    let [x1, y1] = coord_from_n(node.num-1, number, radius + 25.0);
                    text::Text::new_color([0.0, 0.0, 0.0, 1.0], 16).draw(
                        &format!("{:02}", node.num),
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(x1 - 12.5, y1 + 6.0),
                        graphics
                    );
                }
            });
        }
    }
}
