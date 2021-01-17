extern crate piston_window;

use piston_window::*;

struct Square<'a> {x:f64, y:f64, c:graphics::types::Color, p:Option<i32>, tl:Option<&'a Square<'a>>, tr:Option<&'a Square<'a>>, lf:Option<&'a Square<'a>>, rt:Option<&'a Square<'a>>, bl:Option<&'a Square<'a>>, br:Option<&'a Square<'a>>}

// rownum goes from 1 to 17
fn row_length(rownum:i32) -> i32 {
    if (rownum < 5) {
        return rownum
    } else if (rownum > 13) {
        return 18-rownum
    } else {
        return 9 + (9-rownum).abs()
    }
}

fn main() {

    let H: f64 = 100.0; // hexagon side length
    let C_x = 640.0 / 2.0;
    let C_y = 480.0 / 2.0;
    let angles = [0.0, 60.0, 120.0, 180.0, 240.0, 300.0];

    let angles_rad : Vec<f64> = angles.into_iter().map(|angle| {
        return angle * std::f64::consts::PI / 180.0;
    }).collect::<Vec<f64>>();

    let points : Vec<[f64; 2]> = angles_rad.into_iter().map(|angle| {
        return [C_x + H * angle.cos(), C_y + H * angle.sin()];
    }).collect::<Vec<[f64; 2]>>();

    // initialize board
    let start : Square = Square{x:1000.0, y:1000.0, c:[0.0, 0.0, 0.0, 0.0], p:None, tl:None, tr:None, lf:None, rt:None, bl:None, br:None};

    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480]).resizable(false).exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //           [0.0, 0.0, 100.0, 100.0],
            //           c.transform, g);
            polygon([0.0,0.0,0.0,1.0], points.as_slice(), c.transform, g);
        });
    }
}


