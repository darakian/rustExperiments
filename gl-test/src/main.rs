extern crate glium;

fn main() {
    use glium::{Display, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    // 2. Parameters for building the Window.
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let context = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(window, context, &events_loop).unwrap();
    // let display = glium::glutin::WindowBuilder::new()
    //     .with_dimensions(1024, 768)
    //     .with_title(format!("Hello world"))
    //     .build_glium()
    //     .unwrap();

    let mut x = 1.0;
    let mut y = 2.0;
    let mut z = 3.0;
    let mut target = display.draw();

    loop {

        target.clear_color(x, y, z, 1.0);
        //target.finish().unwrap();
        x = (x +1.0);
        y = (y +2.0);
        z = (z +3.0);


        // for ev in display.poll_events() {
        //     match ev {
        //         glium::glutin::Event::Closed => return,
        //         _ => ()
        //     }
        // }
    }
}
