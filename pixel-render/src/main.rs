extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Pixels!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut x_draw_pos = 0.0;
    let mut y_draw_pos = 0.0;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [x_draw_pos, y_draw_pos, 100.0, 100.0],
                      c.transform, g);
        });
        x_draw_pos=(x_draw_pos+15.0)%640.0;
        y_draw_pos=(y_draw_pos+5.0)%480.0;
    }
}
