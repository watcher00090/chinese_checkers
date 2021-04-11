use ggez::{graphics, Context, ContextBuilder, GameResult, nalgebra as na};
use ggez::event::{self, EventHandler};
use ggez::graphics::{DrawMode, MeshBuilder, DrawParam, Color};
use ggez::conf::{WindowSetup, NumSamples};

#[derive(PartialEq)]
struct Hextile {
    y_hex: i32,
    x_hex: i32,
    z_hex: i32,
    c: graphics::Color,
    p: Option<i32>,
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
            c: Color::new(0.0, 0.0, 0.0, 0.0),
            p: None,
        };
    }

    //topright()
    //bottomleft()
    //bottomright()
    //left()
    //right()

    // equals and contain functions


    fn get_tl(board: &Vec<Hextile>) -> Option<Hextile> {
        return None
    }

    fn get_tr(board: &Vec<Hextile>) -> Option<Hextile> {
        return None
    }

    fn get_rt(board: &Vec<Hextile>) -> Option<Hextile> {
        return None
    }

    fn get_lf(board: &Vec<Hextile>) -> Option<Hextile> {
        return None
    }

    fn get_bl(board: &Vec<Hextile>) -> Option<Hextile> {
        return None
    }       

    fn get_br(board: &Vec<Hextile>) -> Option<Hextile> {
        return None
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
    mut board: &mut Vec<Hextile>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    hex_color: Color,
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
 
// Is 'dest' a tile that can be moved to a single move, and can we move from 'piece' to 'dest' in a single move
fn check_step(piece: &Hextile, dest: &Hextile, board: &Vec<Hextile>) -> bool {
    let mut tmp_var_tl : Option<Hextile> = None;
    tmp_var_tl = Hextile::get_tl(board);
    if tmp_var_tl.is_some() {
        tmp_var_tl = Hextile::get_tl(board);
        if tmp_var_tl.is_some() && tmp_var_tl.unwrap() == *dest {
            return true 
        }
    }
    return false;
}

// Dir::top_left -> get_tl()
// Dir::top_right -> get_tr()
// Dir::left -> get_lf()
// ....... 
fn get_method_handle_for_direction(dir: Direction) -> i32 {
    return 0;
}

fn check_hop(piece: &Hextile, dest: &Hextile, board: &Vec<Hextile>) -> bool {
    return false;
}

fn move_piece(piece: Hextile, dest: Hextile, board: &Vec<Hextile>) {
    if check_step(&piece, &dest, board) || check_hop(&piece, &dest, board) {
        // change color of destination to color of moved piece and vice versa
    } else {
        // give error
    }
}

fn create_board() -> Vec<Hextile> {
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

    let yellow_color_array: Color = Color::new(0.902, 0.886, 0.110, 1.0);
    let red_color_array: Color = Color::new(0.902, 0.110, 0.110, 1.0);
    let blue_color_array: Color = Color::new(0.110, 0.110, 0.902, 1.0);
    let green_color_array: Color = Color::new(0.059, 0.600, 0.239, 1.0);
    let black_color_array: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    let white_color_array: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    let center_color_array: Color = Color::new(0.5, 0.5, 0.5, 0.5);

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
    return board; 
}

fn main() {

    // Create the board
    let mut board: Vec<Hextile> = create_board();
    
    // Set up the Window
    let game_window_setup : WindowSetup = WindowSetup{
        title: "Chinese Checkers".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_setup(game_window_setup)
        .build()
        .expect("Error, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => {
            println!("Error occurred: {}", e); 
        }
    }
}

struct MyGame {
    // Your state here...
    // Everything that we need to render the game board.
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let (window_width, window_height) : (f32,f32) = graphics::drawable_size(ctx); 
        let (circle_x, circle_y) : (f32, f32) = (window_width / 2.0, window_height / 2.0);
        let circle_radius : f32 = window_width.min(window_height) / 6.0;

        let board_circle_mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), na::Point2::new(circle_x, circle_y), circle_radius, 0.00001, graphics::BLACK,)
            .build(ctx)?;

        graphics::draw(ctx, &board_circle_mesh, DrawParam::default())?;

        // Draw code here...
        graphics::present(ctx)?;
        
        return Ok(())
    }
}