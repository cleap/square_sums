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

    fn draw(&mut self, tot_num: u32, radius: f64, context: &Context,
            graphics: &mut G2d) {
        let [x, y] = coord_from_n(self.num-1, tot_num, radius);
        ellipse(
            [1.0, 0.0, 0.0, 1.0],
            ellipse::circle(x, y, 5.0),
            context.transform,
            graphics
        );
    }

    fn draw_text(&mut self, tot_num: u32, radius: f64, context: &Context,
            graphics: &mut G2d, glyphs: &mut Glyphs) {

        let [x, y] = coord_from_n(self.num-1, tot_num, radius);
        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 16).draw(
            &format!("{:02}", self.num),
            glyphs,
            &context.draw_state,
            context.transform.trans(x - 12.5, y + 6.0),
            graphics
        );
    }

    fn draw_link(n1: u32, n2: u32, tot_num: u32, radius: f64,
            context: &Context, graphics: &mut G2d) {

        let [x1, y1] = coord_from_n(n1 - 1, tot_num, radius);
        let [x2, y2] = coord_from_n(n2 - 1, tot_num, radius);
        line(
            [1.0, 0.0, 0.0, 1.0],
            1.0,
            [x1, y1, x2, y2],
            context.transform,
            graphics
         );
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
    let mut glyphs = Glyphs::new(font, factory,
        TextureSettings::new()).unwrap();

    // Other vars
    let mut number: u32 = 15;
    let mut nodes: Vec<Node> = Node::generate_graph(number);

    // Event handler
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        // Key events
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Right | Key::Up => {
                        number += 1;
                        nodes = Node::generate_graph(number);
                    },
                Key::Left | Key::Down => {
                        number -= 1;
                        nodes = Node::generate_graph(number);
                    }
                _ => {}
            }
        }

        // Render
        if let Some(_r) = e.render_args() {

            let radius: f64 = 8.0 * number as f64;

            window.draw_2d(&e, |context, mut graphics| {

                clear([1.0, 1.0, 1.0, 1.0], graphics);

                for node in &mut nodes {
                    node.draw(number, radius, &context, &mut graphics);
                    node.draw_text(number, radius + 25.0, &context, &mut graphics, &mut glyphs);
                    for other in &node.links {
                        Node::draw_link(node.num, *other, number, radius, &context, &mut graphics)
                    }
                }
            });
        }
    }
}
