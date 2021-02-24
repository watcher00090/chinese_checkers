extern crate piston_window;

use piston_window::*;

use graphics::color::*;

struct Hextile {
    y_hex: i32,
    x_hex: i32,
    z_hex: i32,
    c: graphics::types::Color,
    p: Option<i32>,
}

static XMIN_BOARD: f64 = -10.0;
static XMAX_BOARD: f64 = 10.0;
static YMIN_BOARD: f64 = -10.0;
static YMAX_BOARD: f64 = 10.0;
static R_X_BOARD: f64 = 10.0; // horizontal distance from the center of the board to the far right
static R_Y_BOARD: f64 = 10.0; // vertical distance from the center of the board to the top

fn screen_x(x: f64) -> f64 {
    return 320.0 + (x / R_X_BOARD) * 320.0;
}

fn screen_y(y: f64) -> f64 {
    return 240.0 + (-(y / R_Y_BOARD)) * 240.0;
}

// rownum goes from 1 to 17
fn row_length(rownum: i32) -> i32 {
    if (rownum < 5) {
        return rownum;
    } else if (rownum > 13) {
        return 18 - rownum;
    } else {
        return 9 + (9 - rownum).abs();
    }
}

impl Hextile {
    //top_left()
    fn top_left(&self) -> Hextile {
        return Hextile {
            y_hex: 0,
            x_hex: 0,
            z_hex: 0,
            c: [0.0, 0.0, 0.0, 0.0],
            p: None,
        };
    }

    //topright()
    //bottomleft()
    //bottomright()
    //left()
    //right()

    fn set_color(&self) {}

    // Center of the screen = (320, 240)

    // width = 640.0
    // height = 480.0
    // x in [-10, 10], y in [-10,10]
    fn screen_x(&self) -> f64 {
        let x: f64 = self.cartesian_x();
        return 320.0 + (x / R_X_BOARD) * 320.0;
    }

    fn screen_y(&self) -> f64 {
        let y: f64 = self.cartesian_y();
        return 240.0 + (-(y / R_Y_BOARD)) * 240.0;
    }

    // a = cartesian x
    // b = cartesian y
    // a = x + z / 2
    // b = -z * sqrt(3) / 2

    fn cartesian_x(&self) -> f64 {
        let x: f64 = self.x_hex as f64;
        let y: f64 = self.y_hex as f64;
        let z: f64 = self.z_hex as f64;
        return x + z / 2.0;
    }

    fn cartesian_y(&self) -> f64 {
        let x: f64 = self.x_hex as f64;
        let y: f64 = self.y_hex as f64;
        let z: f64 = self.z_hex as f64;
        let inner: f64 = 3.0;
        return -z * (inner).sqrt() / 2.0;
    }
}

// add the valid tiles in the given range to the board
//fn add_appropriate_hextiles_to_board(mut board: &mut Vec<Hextile>, x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) {
fn add_appropriate_hextiles_to_board(
    mut board: &mut Vec<Hextile>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    hex_color: [f32; 4],
) {
    for x in x_min..(x_max + 1) {
        for y in y_min..(y_max + 1) {
            for z in z_min..(z_max + 1) {
                if x + y + z == 0 {
                    //let tile : Hextile = Hextile{y_hex: y, x_hex: x, z_hex: z, c: [0.0,0.0,0.0,0.0], p: None};
                    let tile: Hextile = Hextile {
                        y_hex: y,
                        x_hex: x,
                        z_hex: z,
                        c: hex_color,
                        p: None,
                    };
                    (*board).push(tile)
                }
            }
        }
    }
}

fn main() {
    let H: f64 = 100.0; // hexagon side length
    let C_x = 640.0 / 2.0;
    let C_y = 480.0 / 2.0;
    let angles = [0.0, 60.0, 120.0, 180.0, 240.0, 300.0];

    let angles_rad: Vec<f64> = angles
        .into_iter()
        .map(|angle| {
            return angle * std::f64::consts::PI / 180.0;
        })
        .collect::<Vec<f64>>();

    let points: Vec<[f64; 2]> = angles_rad
        .into_iter()
        .map(|angle| {
            return [C_x + H * angle.cos(), C_y + H * angle.sin()];
        })
        .collect::<Vec<[f64; 2]>>();

    let mut board: Vec<Hextile> = Vec::new();

    // furthest points of the board
    // let top : Hextile = Hextile{y_hex : 4, x_hex : 4, z_hex : -8, c : [0.0,0.0,0.0,0.0], p : None};
    // let top_left : Hextile = Hextile{y_hex : 8, x_hex : -4, z_hex : -4, c : [0.0,0.0,0.0,0.0], p : None};
    // let top_right : Hextile = Hextile{y_hex : -4, x_hex : 8, z_hex : -4, c : [0.0,0.0,0.0,0.0], p : None};
    // let bottom : Hextile = Hextile{y_hex : -4, x_hex : -4, z_hex : 8, c : [0.0,0.0,0.0,0.0], p : None};
    // let bottom_left : Hextile = Hextile{y_hex : 4, x_hex : -8, z_hex : 4, c : [0.0,0.0,0.0,0.0], p : None};
    // let bottom_right : Hextile = Hextile{y_hex : -8, x_hex : 4, z_hex : 4, c : [0.0,0.0,0.0,0.0], p : None};

    // points at the edges of the hexagon
    // let hex_top_left : Hextile = Hextile{y_hex : 4, x_hex : 0, z_hex : -4, c : [0.0,0.0,0.0,0.0], p : None};
    // let hex_top_right : Hextile = Hextile{y_hex : 0, x_hex : 4, z_hex : -4, c : [0.0,0.0,0.0,0.0], p : None};
    // let hex_left : Hextile = Hextile{y_hex : 4, x_hex : -4, z_hex : 0, c : [0.0,0.0,0.0,0.0], p : None};
    // let hex_right : Hextile = Hextile{y_hex : -4, x_hex : 4, z_hex : 0, c : [0.0,0.0,0.0,0.0], p : None};
    // let hex_bottom_left : Hextile = Hextile{y_hex : 0, x_hex : -4, z_hex : 4, c : [0.0,0.0,0.0,0.0], p : None};
    // let hex_bottom_right : Hextile = Hextile{y_hex : -4, x_hex : 0, z_hex : 4, c : [0.0,0.0,0.0,0.0], p : None};

    let yellow_color_array: [f32; 4] = [0.902, 0.886, 0.110, 1.0];
    let red_color_array: [f32; 4] = [0.902, 0.110, 0.110, 1.0];
    let blue_color_array: [f32; 4] = [0.110, 0.110, 0.902, 1.0];
    let green_color_array: [f32; 4] = [0.059, 0.600, 0.239, 1.0];
    let black_color_array: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    let white_color_array: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    let center_color_array: [f32; 4] = [0.5, 0.5, 0.5, 0.5];

    // yellow triangle: x in [-4, -1], y in [-4, -1], z in [5, 8]
    let x_min: i32 = -4;
    let x_max: i32 = -1;
    let y_min: i32 = -4;
    let y_max: i32 = -1;
    let z_min: i32 = 5;
    let z_max: i32 = 8;
    add_appropriate_hextiles_to_board(
        &mut board,
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
        yellow_color_array,
    );

    // red triangle: x in [-8, -5], y in [1, 4], z in [1, 4]
    let x_min: i32 = -8;
    let x_max: i32 = -5;
    let y_min: i32 = 1;
    let y_max: i32 = 4;
    let z_min: i32 = 1;
    let z_max: i32 = 4;
    add_appropriate_hextiles_to_board(
        &mut board,
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
        red_color_array,
    );

    // blue triangle: x in [1, 4], y in [-5, -8], z in [1, 4]
    let x_min: i32 = 1;
    let x_max: i32 = 4;
    let y_min: i32 = -8;
    let y_max: i32 = -5;
    let z_min: i32 = 1;
    let z_max: i32 = 4;
    add_appropriate_hextiles_to_board(
        &mut board,
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
        blue_color_array,
    );

    // black triangle:  x in [-8, -5], y in [5, 8], z in [-4 ,-1]
    let x_min: i32 = -4;
    let x_max: i32 = -1;
    let y_min: i32 = 5;
    let y_max: i32 = 8;
    let z_min: i32 = -4;
    let z_max: i32 = -1;
    add_appropriate_hextiles_to_board(
        &mut board,
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
        black_color_array,
    );

    // green triangle: x in [5, 8], y in [-4, -1], z in [-4, -1]
    let x_min: i32 = 5;
    let x_max: i32 = 8;
    let y_min: i32 = -4;
    let y_max: i32 = -1;
    let z_min: i32 = -4;
    let z_max: i32 = -1;
    add_appropriate_hextiles_to_board(
        &mut board,
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
        green_color_array,
    );

    // white triangle: x in [1, 4], y in [1, 4], z in [-5, -8]
    let x_min: i32 = 1;
    let x_max: i32 = 4;
    let y_min: i32 = 1;
    let y_max: i32 = 4;
    let z_min: i32 = -8;
    let z_max: i32 = -5;
    add_appropriate_hextiles_to_board(
        &mut board,
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
        white_color_array,
    );

    // center squares
    let x_min : i32 = -4;
    let x_max : i32 = 4;
    let y_min : i32 = -4;
    let y_max : i32 = 4;
    let z_min : i32 = -4;
    let z_max : i32 = 4;
    add_appropriate_hextiles_to_board(
        &mut board, 
        x_min, 
        x_max, 
        y_min, 
        y_max, 
        z_min, 
        z_max,
        center_color_array
    );

    let mut prev_row: Vec<&mut Hextile> = Vec::new();

    // let delta_x, delta_y, delta_z
    // let delta_x_prime, delta_y_prime, delta_z_prime

    // let traingle_endpoint
    let screen_radius: f64 = screen_x(R_X_BOARD) - screen_x(0.0);
    println!(
        "screen_radius = {screen_radius}",
        screen_radius = screen_radius
    );

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        /*
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //           [0.0, 0.0, 100.0, 100.0],
            //           c.transform, g);
            // polygon([0.0,0.0,0.0,1.0], points.as_slice(), c.transform, g);

            for tile in board.iter() {
                format!("x_hex = {x_hex}, y_hex = {y_hex}, z = {z_hex}", x_hex = tile.x_hex, y_hex = tile.y_hex, z_hex = tile.z_hex);
                std::thread::sleep(std::time::Duration::from_millis(2000));
                circle_arc([0.5,0.5,0.5,1.0], 5.0, 0.0, 6.3, [tile.screen_x() - 5.0, tile.screen_y() - 5.0, 10.0, 10.0], c.transform, g);
            }

            //circle_arc([0.5,0.5,0.5,1.0], 5.0, 0.0, 6.3, [(320.0 - 5.0), (240.0 - 5.0), 10.0, 10.0], c.transform, g);
        });

        */
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);

            let origin_x: f64 = 0.0;
            let origin_y: f64 = 0.0;
            let R: f64 = R_X_BOARD;
            let screen_radius: f64 = screen_y(R_Y_BOARD) - screen_y(0.0);
            //  println!("screen_radius = %d", screen_radius)
            ellipse(
                [0.569, 0.404, 0.173, 1.0],
                [
                    screen_x(origin_x) - screen_radius,
                    screen_y(origin_y) - screen_radius,
                    2.0 * screen_radius,
                    2.0 * screen_radius,
                ],
                c.transform,
                g,
            );
        }); //145 103 44

        for tile in board.iter() {
            window.draw_2d(&e, |c, g, _device| {
                // rectangle([1.0, 0.0, 0.0, 1.0], // red
                //           [0.0, 0.0, 100.0, 100.0],
                //           c.transform, g);
                // polygon([0.0,0.0,0.0,1.0], points.as_slice(), c.transform, g);

                //format!("x_hex = {x_hex}, y_hex = {y_hex}, z = {z_hex}", x_hex = tile.x_hex, y_hex = tile.y_hex, z_hex = tile.z_hex);
                //std::thread::sleep(std::time::Duration::from_millis(200));
                circle_arc(
                    tile.c,
                    5.0,
                    0.0,
                    6.3,
                    [tile.screen_x() - 5.0, tile.screen_y() - 5.0, 10.0, 10.0],
                    c.transform,
                    g,
                );

                //circle_arc([0.5,0.5,0.5,1.0], 5.0, 0.0, 6.3, [(320.0 - 5.0), (240.0 - 5.0), 10.0, 10.0], c.transform, g);
            });
            //std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
}
