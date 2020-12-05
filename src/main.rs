extern crate piston_window;

use piston_window::*;

fn main() {
<<<<<<< HEAD

    let H: f64 = 5.0; // hexagon side length
    let C_x = 640.0 / 2.0;
    let C_y = 480.0 / 2.0;
    let angles = [0.0, 60.0, 120.0, 180.0, 240.0, 300.0];
    let angles_rad : [f64; 6] = angles.into_iter().map(|angle| {
        return angle * 180.0 / std::f64::consts::PI;
    }).collect::<Vec<f64>>().as_slice();
    let points : [f64; 60] = angles_rad.into_iter().map(|angle| {
        return [C_x + H * angle.cos(), C_y + H * angle.sin()];
    }).collect();

    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480]).resizable(false).exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
            polygon([0.0,0.0,0.0,1.0], points, graphics::math::identity(), g);
=======
    let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
>>>>>>> 4a50f15ca89f2b0c7a6d06be514124fd62518dec
        });
    }
}


