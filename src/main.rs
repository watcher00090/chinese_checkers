extern crate piston_window;

use piston_window::*;

use graphics::color::*;

<<<<<<< Updated upstream
struct Hextile{y_hex: i32, x_hex: i32, z_hex: i32, c:graphics::types::Color, p:Option<i32>}
=======
struct Hextile {
    y_hex: i32,
    x_hex: i32,
    z_hex: i32,
    c: graphics::types::Color,
    p: Option<i32>,
    tile_type: TileColor
}

enum TileColor {
    RED,
    YELLOW,
    BLUE,
    GREEN,
    BLACK,
    WHITE, 
    GREY
}

enum Direction {yellow,red,green,blue,white,black}

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
>>>>>>> Stashed changes

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

impl Hextile {
    //top_left()
    fn top_left(&self) -> Hextile {
        return Hextile{y_hex: 0, x_hex: 0, z_hex: 0, c: [0.0,0.0,0.0,0.0], p: None}
    }

    //topright()
    //bottomleft()
    //bottomright()
    //left()
    //right()

<<<<<<< Updated upstream
    
    fn set_color(&self) {
        
=======
    // equals and contain functions

    // fast method for (x_hex, y_hex, z_hex) -> Option(&Hextile)
    
    fn get_tl<'a>(hex: &'a Hextile, board: Vec<&'a Hextile>)  -> Option<&'a Hextile> {
        for tile in board {
            let dx : i32 = 0;
            let dy : i32 = 1;
            let dz : i32 = -1;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    fn get_tr<'a>(hex: &'a Hextile, board: Vec<&'a Hextile>) -> Option<&'a Hextile> {
        for tile in board {
            let dx : i32 = 1;
            let dy : i32 = 0;
            let dz : i32 = -1;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    fn get_rt<'a>(hex: &'a Hextile, board: Vec<&'a Hextile>) -> Option<&'a Hextile> {
        for tile in board {
            let dx : i32 = 1;
            let dy : i32 = -1;
            let dz : i32 = 0;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    fn get_lf<'a>(hex: &'a Hextile, board: Vec<&'a Hextile>) -> Option<&'a Hextile> {
        for tile in board {
            let dx : i32 = -1;
            let dy : i32 = 1;
            let dz : i32 = 0;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    fn get_bl<'a>(hex: &'a Hextile, board: Vec<&'a Hextile>) -> Option<&'a Hextile> {
        for tile in board {
            let dx : i32 = -1;
            let dy : i32 = 0;
            let dz : i32 = 1;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    fn get_br<'a>(hex: &'a Hextile, board: Vec<&'a Hextile>) -> Option<&'a Hextile> {
        for tile in board {
            let dx : i32 = 0;
            let dy : i32 = -1;
            let dz : i32 = 1;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
>>>>>>> Stashed changes
    }

    // Center of the screen = (240, 320)

    // width = 640.0
    // height = 480.0
    // x in [-10, 10], y in [-10,10]
    fn screen_x(&self) -> f64 {
        let x : f64 = self.cartesian_x();
        return 320.0 + (x / 10.0) * 320.0;
    }

    fn screen_y(&self) -> f64 {
        let y : f64 = self.cartesian_y();
        return 240.0 + (-(y / 10.0)) * 240.0;
    }

    // a = cartesian x
    // b = cartesian y
    // a = x + z / 2
    // b = -z * sqrt(3) / 2

    fn cartesian_x(&self) -> f64 {
        let x : f64 = self.x_hex as f64;
        let y : f64 = self.y_hex as f64;
        let z : f64 = self.z_hex as f64;
        return x + z / 2.0;
    }

    fn cartesian_y(&self) -> f64 {
        let x : f64 = self.x_hex as f64;
        let y : f64 = self.y_hex as f64;
        let z : f64 = self.z_hex as f64;
        let inner : f64 = 3.0;
        return -z * (inner).sqrt() / 2.0;
    }

}

// add the valid tiles in the given range to the board
//fn add_appropriate_hextiles_to_board(mut board: &mut Vec<Hextile>, x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) {
<<<<<<< Updated upstream
fn add_appropriate_hextiles_to_board(mut board: &mut Vec<Hextile>, x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32, hex_color: [f32;4]) {
    for x in x_min..(x_max+1) {
        for y in y_min..(y_max+1) {
            for z in z_min..(z_max+1) {
                if x + y + z == 0 {
                    //let tile : Hextile = Hextile{y_hex: y, x_hex: x, z_hex: z, c: [0.0,0.0,0.0,0.0], p: None};
                    let tile : Hextile = Hextile{y_hex: y, x_hex: x, z_hex: z, c: hex_color, p: None};
=======
fn add_appropriate_hextiles_to_board(
    mut board: &mut Vec<Hextile>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    hex_color: [f32; 4],
    tile_color: TileColor
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
                        tile_type: tile_color
                    };
>>>>>>> Stashed changes
                    (*board).push(tile)
                }
            }
        }
    }
}

<<<<<<< Updated upstream
=======


fn get_adjacent(x: i32, y: i32, z: i32) -> Vec<[i32; 3]> {
    let mut neighbors: Vec<[i32; 3]> = Vec::new();
    neighbors.push([x, y+1, z-1]); // top left
    neighbors.push([x+1, y, z-1]); // top right
    neighbors.push([x, y-1, z+1]); // bottom right
    neighbors.push([x-1, y+1, z]); // left
    neighbors.push([x+1, y-1, z]); // right
    neighbors.push([x-1, y, z+1]); // bottom left
    return neighbors
}

fn check_step<'a>(piece: &'a Hextile, dest: &'a Hextile) -> bool {
    return false;
}

fn check_hop<'a>(piece: &'a Hextile, dest: &'a Hextile) -> bool {
    return false;
}

fn can_single_step_move_to_dest<'a>(piece: &'a Hextile, dest: &'a Hextile, board: Vec<&'a Hextile>) -> bool {
    // check if dest is one tile away from piece
    if (get_bl(piece, board) != None) {
        if ()
    }

    }

    // check if dest is unoccupied, piece is occupied, and the tile between is occupied

}

// move the piece pointed to by the piece reference to the location pointed to by the dest reference
fn move_piece<'a>(piece: &'a Hextile, dest: &'a Hextile) {
    if can_single_step_move_to_dest(piece, dest) {
        // change color of destination to color of moved piece and vice versa
        // swap the colors 
        let tmp : TileColor = (*piece).tile_type;
        (*piece).tile_type = (*dest).tile_type;
        (*dest).tile_type = tmp;
    } else {
        // give error
    }
}

>>>>>>> Stashed changes
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

    let mut board : Vec<Hextile> = Vec::new();

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

    let yellow_color_array  : [f32; 4] = [0.902, 0.886, 0.110, 1.0];
    let red_color_array : [f32; 4] = [0.902, 0.110, 0.110, 1.0];
    let blue_color_array  : [f32; 4] = [0.110, 0.110, 0.902, 1.0];
    let green_color_array  : [f32; 4] = [0.059, 0.600, 0.239, 1.0];
    let black_color_array  : [f32; 4] = [0.0,0.0,0.0,1.0];
    let white_color_array : [f32; 4] = [0.5, 0.5, 0.9, 1.0];

    // yellow triangle: x in [-4, -1], y in [-4, -1], z in [5, 8]
<<<<<<< Updated upstream
    let x_min : i32 = -4;
    let x_max : i32 = -1;
    let y_min : i32 = -4;
    let y_max : i32 = -1;
    let z_min : i32 = 5;
    let z_max : i32 = 8;
    add_appropriate_hextiles_to_board(&mut board, x_min, x_max, y_min, y_max, z_min, z_max, yellow_color_array);

    // red triangle: x in [-8, -5], y in [1, 4], z in [1, 4]          
    let x_min : i32 = -8;
    let x_max : i32 = -5;
    let y_min : i32 = 1;
    let y_max : i32 = 4;
    let z_min : i32 = 1;
    let z_max : i32 = 4;
    add_appropriate_hextiles_to_board(&mut board, x_min, x_max, y_min, y_max, z_min, z_max, red_color_array);
                                                                               
    // blue triangle: x in [1, 4], y in [-5, -8], z in [1, 4]
    let x_min : i32 = 1;
    let x_max : i32 = 4;
    let y_min : i32 = -5;
    let y_max : i32 = -8;
    let z_min : i32 = 1;
    let z_max : i32 = 4;
    add_appropriate_hextiles_to_board(&mut board, x_min, x_max, y_min, y_max, z_min, z_max, blue_color_array);

    // black triangle:  x in [-8, -5], y in [5, 8], z in [-4 ,-1]
    let x_min : i32 = -8;
    let x_max : i32 = -5;
    let y_min : i32 = 5;
    let y_max : i32 = 8;
    let z_min : i32 = -4;
    let z_max : i32 = -1;
    add_appropriate_hextiles_to_board(&mut board, x_min, x_max, y_min, y_max, z_min, z_max, black_color_array);

    // green triangle: x in [5, 8], y in [-4, -1], z in [-4, -1]
    let x_min : i32 = 5;
    let x_max : i32 = 8;
    let y_min : i32 = -4;
    let y_max : i32 = -1;
    let z_min : i32 = -4;
    let z_max : i32 = -1;
    add_appropriate_hextiles_to_board(&mut board, x_min, x_max, y_min, y_max, z_min, z_max, green_color_array);

=======
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
        TileColor::YELLOW
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
        TileColor::RED
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
        TileColor::BLUE
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
        TileColor::BLACK
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
        TileColor::GREEN
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
        TileColor::WHITE
    );
>>>>>>> Stashed changes

    // white triangle: x in [1, 4], y in [1, 4], z in [-5, -8]
    let x_min : i32 = 1;
    let x_max : i32 = 4;
    let y_min : i32 = -5;
    let y_max : i32 = -8;
    let z_min : i32 = 1;
    let z_max : i32 = 4;
<<<<<<< Updated upstream
    add_appropriate_hextiles_to_board(&mut board, x_min, x_max, y_min, y_max, z_min, z_max, white_color_array);

    // center squares
    //let x_min : i32 = -4;
    //let x_max : i32 = 4;
    //let y_min : i32 = -4;
    //let y_max : i32 = 4;
    //let z_min : i32 = -4;
    //let z_max : i32 = 4;
    //add_appropriate_hextiles_to_board(&mut board, x_min, x_max, y_min, y_max, z_min, z_max);

    

    let mut prev_row : Vec<&mut Hextile> = Vec::new();

   // let delta_x, delta_y, delta_z
   // let delta_x_prime, delta_y_prime, delta_z_prime

   // let traingle_endpoint


    let mut prev_row : Vec<&mut Square> = Vec::new();
    prev_row.push(&start);

    let Nrows : i32 = 17;
    let j : i32;

    for j in 0..Nrows {
        let row_num : i32 = j+1;

        let mut cur_row : Vec<&mut Square> = Vec::new();
        for k in 0..row_length(row_num) {
            let tmp_square : Square = Square{x:1000.0, y:1000.0, c:[0.0, 0.0, 0.0, 0.0], p:None, tl:None, tr:None, lf:None, rt:None, bl:None, br:None};
            cur_row.push(&mut tmp_square);
        }

        for k in 0..row_length(row_num-1) {
            let mut tmp : &mut Square = *(prev_row.get_mut(k as usize).unwrap());
            // connect tmp to tile at bottom left
            if !(row_num == 14 && k == 0)  { // special case for edge elements in top row of bottom triangle
                if row_length(row_num-1) == 4 { 
                    tmp.bl = Some(*(cur_row.get_mut(5+k as usize).unwrap()));
                    let mut top_el : &mut Square = *cur_row.get_mut(5+k as usize).unwrap();
                    top_el.tr = Some(tmp);
                } else { 
                    tmp.bl = Some(*(cur_row.get_mut(k as usize).unwrap())); 
                    let mut top_el : &mut Square = *cur_row.get_mut(k as usize).unwrap();
                    top_el.tr = Some(tmp);
                }
            }
            // connecting tmp to tile at bottom right
            if !(row_num == 14 && k == 3) { // special case for edge elements in top row of bottom triangle
                if row_length(row_num-1) == 4 { 
                    tmp.br = Some(*(cur_row.get_mut((5+k+1) as usize).unwrap()));
                    let mut top_el : &mut Square = *cur_row.get_mut((5+k+1) as usize).unwrap();
                    top_el.tl = Some(tmp);
                } else {
                    tmp.br = Some(*(cur_row.get_mut((k+1) as usize).unwrap()));
                    let mut top_el : &mut Square = *cur_row.get_mut((k+1) as usize).unwrap();
                    top_el.tl = Some(tmp);
                }
            }
            if k == 0 {
                tmp.lf = None;
            } else {
                tmp.lf = Some(*(prev_row.get_mut((k-1) as usize).unwrap()));
            }
            if k == row_length(row_num-1)-1 {
                tmp.rt = None;
            } else {
                tmp.rt = Some(*(prev_row.get_mut((k+1) as usize).unwrap()));
            }
        }

        // another case to handle the yellow at the bottom


    }

    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480]).resizable(false).exit_on_esc(true).build().unwrap();
=======
    add_appropriate_hextiles_to_board(
        &mut board, 
        x_min, 
        x_max, 
        y_min, 
        y_max, 
        z_min, 
        z_max,
        center_color_array,
        TileColor::GREY
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
>>>>>>> Stashed changes
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
        });

        for tile in board.iter() {
            window.draw_2d(&e, |c, g, _device| {
                // rectangle([1.0, 0.0, 0.0, 1.0], // red
                //           [0.0, 0.0, 100.0, 100.0],
                //           c.transform, g);
                // polygon([0.0,0.0,0.0,1.0], points.as_slice(), c.transform, g);

                    //format!("x_hex = {x_hex}, y_hex = {y_hex}, z = {z_hex}", x_hex = tile.x_hex, y_hex = tile.y_hex, z_hex = tile.z_hex);
                    //std::thread::sleep(std::time::Duration::from_millis(200));
                    circle_arc(tile.c, 5.0, 0.0, 6.3, [tile.screen_x() - 5.0, tile.screen_y() - 5.0, 10.0, 10.0], c.transform, g);    

                //circle_arc([0.5,0.5,0.5,1.0], 5.0, 0.0, 6.3, [(320.0 - 5.0), (240.0 - 5.0), 10.0, 10.0], c.transform, g);
            });
            //std::thread::sleep(std::time::Duration::from_millis(20));
        }

    }
}


