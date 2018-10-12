extern crate piston_window;

use piston_window::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = WIDTH;

// Generates the coordinates for a number in the list
fn coord_from_n(n: u32, maxn: u32, radius: f64) -> [f64; 2] {
    let scale: f64 = 2.0*std::f64::consts::PI / (maxn as f64);
    let xcoord: f64 = (n as f64 * scale).sin() * radius + WIDTH as f64 / 2.0;
    let ycoord: f64 = -(n as f64 * scale).cos() * radius + HEIGHT as f64 / 2.0;
    [xcoord, ycoord]
}

// Returns true if a link exists between the two points, false otherwise
// (except for 1 and 3)
fn is_link(n1: u32, n2: u32) -> bool {
    ((n1 + n2) as f64).sqrt().fract() == 0.0 && (n1 + n2) > 4 // Because Elijah said so
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

    // Event handler
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        // Key events
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space || key == Key::Right || key == Key::Up {
                number += 1;
            }
            else if key == Key::Backspace || key == Key::Left || key == Key::Down {
                if number > 0 {
                    number -= 1;
                }
            }
        }

        // Render
        if let Some(_r) = e.render_args() {

            let radius: f64 = 8.0 * number as f64;

            window.draw_2d(&e, |context, graphics| {

                clear([1.0, 1.0, 1.0, 1.0], graphics);

                // Draw Nodes
                for n in 0..number {
                    let [xcoord, ycoord] = coord_from_n(n, number, radius);
                    ellipse(
                        [1.0, 0.0, 0.0, 1.0],
                        ellipse::circle(xcoord, ycoord, 5.0),
                        context.transform,
                        graphics
                    );
                    let [xcoord, ycoord] = coord_from_n(n, number, radius + 25.0);
                    text::Text::new_color([0.0, 0.0, 0.0, 1.0], 16).draw(
                        &format!("{:02}", n+1),
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(xcoord - 12.5, ycoord + 6.0),
                        graphics
                    );
                }

                // Draw Links
                for n1 in 0..number {
                    for n2 in n1+1..number {
                        if is_link(n1+1, n2+1) {
                            let [x1, y1] = coord_from_n(n1, number, radius);
                            let [x2, y2] = coord_from_n(n2, number, radius);
                            line([1.0, 0.0, 0.0, 1.0],
                                 1.0,
                                 [x1, y1, x2, y2],
                                 context.transform,
                                 graphics
                             );
                        }
                    }
                }
            });
        }
    }
}
