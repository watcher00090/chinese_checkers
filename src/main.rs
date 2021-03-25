extern crate piston_window;

use piston_window::*;

use graphics::color::*;

#[derive(PartialEq)]
struct Hextile {
    y_hex: i32,
    x_hex: i32,
    z_hex: i32,
    c: graphics::types::Color,
    p: Option<i32>,
    tile_type: TileColor
}

#[derive(PartialEq, Copy, Clone)]
enum TileColor {
    RED,
    YELLOW,
    BLUE,
    GREEN,
    BLACK,
    WHITE, 
    GREY // A tile is empty iff it is grey
}

#[derive(PartialEq)]
enum Direction {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Left,
    Right
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
    /*
    fn top_left(&self) -> Hextile {
        return Hextile {
            y_hex: 0,
            x_hex: 0,
            z_hex: 0,
            c: [0.0, 0.0, 0.0, 0.0],
            p: None,
        };
    }
    */

    //topright()
    //bottomleft()
    //bottomright()
    //left()
    //right()

    // equals and contain functions

    // fast method for (x_hex, y_hex, z_hex) -> Option(&Hextile)

    fn get_tile<'a>(x_hex: i32, y_hex: i32, z_hex: i32) -> Option<&'a Hextile> {
        return None
    }

    // Returns the hextile at the top left of the given tile, if it exists 
    fn get_tl<'a>(hex: &'a Hextile, board: &[&'a Hextile])  -> Option<&'a Hextile> {
        for tile in board.iter() {
            let dx : i32 = 0;
            let dy : i32 = 1;
            let dz : i32 = -1;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    // Returns the hextile at the top right of the given tile, if it exists 
    fn get_tr<'a>(hex: &'a Hextile, board: &[&'a Hextile]) -> Option<&'a Hextile> {
        for tile in board.iter() {
            let dx : i32 = 1;
            let dy : i32 = 0;
            let dz : i32 = -1;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    // Returns the hextile to the right of the given tile, if it exists 
    fn get_rt<'a>(hex: &'a Hextile, board: &[&'a Hextile]) -> Option<&'a Hextile> {
        for tile in board.iter() {
            let dx : i32 = 1;
            let dy : i32 = -1;
            let dz : i32 = 0;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    // Returns the hextile to the left of the given tile, if it exists 
    fn get_lf<'a>(hex: &'a Hextile, board: &[&'a Hextile]) -> Option<&'a Hextile> {
        for tile in board.iter() {
            let dx : i32 = -1;
            let dy : i32 = 1;
            let dz : i32 = 0;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    // Returns the hextile at the bottom left of the given tile, if it exists
    fn get_bl<'a>(hex: &'a Hextile, board: &[&'a Hextile]) -> Option<&'a Hextile> {
        for tile in board.iter() {
            let dx : i32 = -1;
            let dy : i32 = 0;
            let dz : i32 = 1;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

    // Returns the hextile at the bottom right of the given tile, if it exists
    fn get_br<'a>(hex: &'a Hextile, board: &[&'a Hextile]) -> Option<&'a Hextile> {
        for tile in board.iter() {
            let dx : i32 = 0;
            let dy : i32 = -1;
            let dz : i32 = 1;
            if (*hex).x_hex + dx == (*tile).x_hex && (*hex).y_hex + dy == (*tile).y_hex && (*hex).z_hex + dz == (*tile).z_hex {
                return Some(&tile);
            }
        }
        return None;
    }

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
        return -z * (inner).sqrt() / 1.5;
    }
}

// add the valid tiles in the given range to the board
//fn add_appropriate_hextiles_to_board(mut board: &mut Vec<Hextile>, x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) {
fn add_appropriate_hextiles_to_board(
    board: &mut Vec<Hextile>,
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
                    (*board).push(tile)
                }
            }
        }
    }
}



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

// Returns the tile two squares away, in the given direction, if it exists. Otherwise returns None.
fn hextile_two_away_in_a_given_direction<'a>(tile: &'a Hextile, board: &[&'a Hextile], dir: Direction) -> Option<&'a Hextile> {
    if dir == Direction::BottomLeft {
        let tmp_tile : Option<&'a Hextile> = Hextile::get_bl(tile, board);
        if tmp_tile.is_some() {
            let tmp_tile_2 : Option<&'a Hextile> = Hextile::get_bl(tmp_tile.unwrap(), board);
            if tmp_tile_2.is_some() {
                return tmp_tile_2;
            }
        }
    }
    if dir == Direction::BottomRight {
        let tmp_tile : Option<&'a Hextile> = Hextile::get_br(tile, board);
        if tmp_tile.is_some() {
            let tmp_tile_2 : Option<&'a Hextile> = Hextile::get_br(tmp_tile.unwrap(), board);
            if tmp_tile_2.is_some() {
                return tmp_tile_2;
            }
        }
    }
    if dir == Direction::Left {
        let tmp_tile : Option<&'a Hextile> = Hextile::get_lf(tile,board);
        if tmp_tile.is_some() {
            let tmp_tile_2 : Option<&'a Hextile> = Hextile::get_lf(tmp_tile.unwrap(), board);
            if tmp_tile_2.is_some() {
                return tmp_tile_2;
            }
        }
    }
    if dir == Direction::Right {
        let tmp_tile : Option<&'a Hextile> = Hextile::get_rt(tile,board);
        if tmp_tile.is_some() {
            let tmp_tile_2 : Option<&'a Hextile> = Hextile::get_rt(tmp_tile.unwrap(), board);
            if tmp_tile_2.is_some() {
                return tmp_tile_2;
            }
        }
    }
    if dir == Direction::TopLeft {
        let tmp_tile : Option<&'a Hextile> = Hextile::get_tl(tile,board);
        if tmp_tile.is_some() {
            let tmp_tile_2 : Option<&'a Hextile> = Hextile::get_tl(tmp_tile.unwrap(), board);
            if tmp_tile_2.is_some() {
                return tmp_tile_2;
            }
        }
    }
    if dir == Direction::TopRight {
        let tmp_tile : Option<&'a Hextile> = Hextile::get_tr(tile,board);
        if tmp_tile.is_some() {
            let tmp_tile_2 : Option<&'a Hextile> = Hextile::get_tr(tmp_tile.unwrap(), board);
            if tmp_tile_2.is_some() {
                return tmp_tile_2;
            }
        }
    }
    return None
}   

/*
fn equals<'a>(tile1: &'a Hextile, tile2: &'a Hextile) -> bool {
    return *tile1 == *tile2
}
*/

fn can_single_step_move_to_dest<'a>(piece: &'a Hextile, dest: &'a Hextile, board: Vec<&'a Hextile>) -> bool {
    // check if dest is one tile away from piece
    if hextile_two_away_in_a_given_direction(piece, &board, Direction::TopLeft).unwrap() == dest {
        /*
        if dest.tile_type != TileType::GREY {
            return true;
        }
        */
    }

    // check if dest is unoccupied, piece is occupied, and the tile between is occupied
    return false;
}

fn get_user_input(mut s_target: String, mut s_destination: String, current_player: i32) {
    println!("Player {}'s turn.", current_player);
    println!("Please enter a target square to move from:");
    std::io::stdin().read_line(&mut s_target).expect("Error, invalid input");
    if let Some('\n')=s_target.chars().next_back() {
        s_target.pop();
    }     
    if let Some('\r')=s_target.chars().next_back() {
        s_target.pop();
    }
    println!("You typed: {}", s_target);

    println!("Please enter a destination square to move to:");
    std::io::stdin().read_line(&mut s_destination).expect("Error, invalid input");
    if let Some('\n')=s_target.chars().next_back() {
        s_destination.pop();
    }     
    if let Some('\r')=s_target.chars().next_back() {
        s_destination.pop();
    }
    println!("You typed: {}", s_destination);
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

// example: 4,tr,3 (format for square)
//
//
//
//
//
fn is_on_board(mut square: String) -> bool {
    remove_whitespace(&mut square);
    let v : Vec<&str> = square.split(|c| c == 'c').collect();
    
    // get the radius
    let radius_parse_result = v[0].parse::<i32>();
    let mut radius : i32 = 0;
    if (!radius_parse_result.is_err()) {
        radius = radius_parse_result.unwrap();
    }

    // get the diagonal
    //let diagonal_parse_result = v[0].parse();
    let mut diagonal = v[1];
    //if (!diagonal_parse_result.is_err()) {
    //    diagonal = diagonal_parse_result.unwrap();
    //}

    // get the number of clockwise moves
    let num_cw_moves_parse_result = v[2].parse::<i32>();
    let mut num_cw_moves : i32 = 0;
    if (!num_cw_moves_parse_result.is_err()) {
        num_cw_moves = num_cw_moves_parse_result.unwrap();
    }
    return false;
}

fn get_hextile<'a>(mut square: String, board: Vec<&Hextile>) -> Option<&'a Hextile> {

    str diagonal
    int radius
    int num_steps_cw

    if (radius == 0) {
        return get_root();
    }

    // Diagonal = top-left
    // x coordinate is 0
    // y coordinate increases and z coordinate decreases heading top left
    if (diag == top-left) {
        // get the coordinates of the tile on the diagonal prior to moving radially
        let x_hex : i32 = 0;
        let y_hex : i32 = radius;
        let z_hex : i32 = -radius;

        // move radially from those coordinates to compute the coordinates of the tile
        // every step subtracts 1 from the y and adds 1 to the x
        for i in 0..num_steps_cw {
            x_hex += 1;
            y_hex -= 1;
        }

        // loop through the board to get the Hextile
    }

    return None
}

// move the piece pointed to by the piece reference to the location pointed to by the dest reference
fn move_piece<'a>(piece: &'a Hextile, dest: &'a Hextile, board: Vec<&'a Hextile>) {
    if can_single_step_move_to_dest(piece, dest, board) {
        // change color of destination to color of moved piece and vice versa
        // swap the colors 
     //   let tmp : TileColor = (*piece).tile_type;
     //   (*piece).tile_type = (*dest).tile_type;
     //   (*dest).tile_type = tmp;
    } else {
        // give error
    }
}

// Two players
// represent a tile as [radius],[diag],[num_moves_clockwise_at_radius_to_reach_from_diag_point]

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

    let mut current_player : i32 = 1; // Player-1 = 1, Player-2 = 2

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

    while let Some(e) = window.next() {
        println!("while-loop iteration...");

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

        // take a line of user input
        let mut s_target=String::new();
        let mut s_destination=String::new();

        get_user_input(s_target, s_destination, current_player);

        while !is_on_board(s_target) || !is_on_board(s_destination) {
            print!("Error, either the target or destination isn't on the board. Please trying inputting the target and destination again...");
            get_user_input(s_target, s_destination, current_player);
        }

    //    move_piece(get_hextile(s_target).unwrap(), get_hextile(s_destination), board);

        current_player = current_player + 1;
        if current_player == 3 {
            current_player = 1;
        }
        
    }


}
