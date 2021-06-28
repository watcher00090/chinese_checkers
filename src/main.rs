use druid::widget::{Scroll, ListIter ,List, FlexParams, MainAxisAlignment, CrossAxisAlignment, ControllerHost, Click, SizedBox, Align, Padding, Button, Flex, Container, Label, IdentityWrapper};
use druid::AppLauncher;
use druid::lens::{self, LensExt};
use druid::{UnitPoint, WidgetPod, WindowId, MenuDesc, MenuItem, Screen, LocalizedString, ContextMenu, Affine, Point, Rect, FontDescriptor, TextLayout, Color, Handled, DelegateCtx, AppDelegate, Command, Selector, Target, Widget, Data, Lens, WindowDesc, EventCtx, Event, Env, LayoutCtx, BoxConstraints, LifeCycle, LifeCycleCtx, Size, PaintCtx, UpdateCtx, WidgetId, WidgetExt, MouseButton};
use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use druid::widget::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};
use druid::kurbo::{Circle, Shape, BezPath};
use druid::piet::{FontFamily, FontWeight, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid_shell::{Menu, HotKey, KbKey, KeyEvent, RawMods, SysMods};
use druid::im;
use druid::im::{vector, Vector};
use std::convert::TryInto;
use std::any::Any;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref whose_turn_FONT : FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(FontWeight::BOLD).with_size(48.0);
}

lazy_static! {
    // Global mutable variable storing the WidgetId of the root widget. 
    static ref root_widget_id_guard : Mutex::<WidgetId> = Mutex::<WidgetId>::new(WidgetId::next());  // global variable always storing the widget id of the root widget
    static ref start_game_selector : Selector<usize> = Selector::new("START_GAME");
    static ref piece_size_bounds : Size = Size::new(20.0, 20.0);
    static ref square_edge_bounds : Size = Size::new(26.5, 26.5);
    // static ref SQUARE_COLOR : Color = Color::rgb8(96,54,15);
    //static ref INTERMEDIATE_CIRCLE_COLOR : Color = Color::rgb8(189, 143, 64);
    static ref SQUARE_COLOR : Color = Color::rgb8( 200, 144, 103 );    
    static ref INTERMEDIATE_CIRCLE_COLOR : Color = Color::rgb8( 200, 144, 103 );   
}

static BOARD_RECT_VERTICAL_OFFSET_IN_CANVAS : f64 = 20f64;

// the number of squares on the board
const N_SQUARES : usize = 121;
const MAX_NUM_PIECES : usize = 60;

static SQRT_3: f64 = 1.732050808;
static ABSTRACT_BOARD_WIDTH: f64 = SQRT_3 * 10.0; 
static ABSTRACT_BOARD_HEIGHT: f64 = SQRT_3 * 10.0;

static BOARD_WIDTH : f64 = 500.0;
static BOARD_HEIGHT : f64 = 500.0;

static ABSTRACT_INNER_CIRCLE_OFFSET : f64 = SQRT_3 / 2.0;

static CANVAS_WIDTH : f64 = 600.0;
static CANVAS_HEIGHT: f64 = BOARD_WIDTH + (2f64)*BOARD_RECT_VERTICAL_OFFSET_IN_CANVAS;
//static ABSTRACT_BOARD_WIDTH: f64 = 25.0;  // horizontal length from size to size of the board, with the origin right in the middle
//static ABSTRACT_BOARD_WIDTH: f64 = 25.0;  // horizontal length from size to size of the board, with the origin right in the middle
//static ABSTRACT_BOARD_HEIGHT: f64 = 15.0; // vertical length from size to size of the board, with the origin right in the middle


// static START_NEW_GAME_2_PLAYERS_ID : u32 = 1000;
// static START_NEW_GAME_3_PLAYERS_ID : u32 = 1001;
// static START_NEW_GAME_4_PLAYERS_ID : u32 = 1002;
// static START_NEW_GAME_5_PLAYERS_ID : u32 = 1003;
// static START_NEW_GAME_6_PLAYERS_ID : u32 = 1004;

static BOARD_CIRCLE_COLOR_r : u8 = 238;
static BOARD_CIRCLE_COLOR_g : u8 = 206;
static BOARD_CIRCLE_COLOR_b : u8 = 166;

// static BOARD_CIRCLE_COLOR_r : u8 = 212;
// static BOARD_CIRCLE_COLOR_g : u8 = 179;
// static BOARD_CIRCLE_COLOR_b : u8 = 137;

// static BOARD_CIRCLE_COLOR_r : u8 = 255;
// static BOARD_CIRLCE_COLOR_g : u8 = 248;
// static BOARD_CIRCLE_COLOR_b : u8 = 220;

lazy_static! {
    static ref YELLOW_COLOR:   Color = Color::rgba(0.902, 0.886, 0.110, 1.0);
    static ref RED_COLOR:      Color = Color::rgba(0.902, 0.110, 0.110, 1.0);
    static ref BLUE_COLOR:     Color = Color::rgba(0.110, 0.110, 0.902, 1.0);
    static ref GREEN_COLOR:    Color = Color::rgba(0.059, 0.600, 0.239, 1.0);
    static ref BLACK_COLOR:    Color = Color::rgba(0.0, 0.0, 0.0, 1.0);
    static ref WHITE_COLOR:    Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    static ref GREY_COLOR:     Color = Color::rgba(0.5, 0.5, 0.5, 1.0);    
    static ref ORANGE_COLOR:   Color = Color::rgba(0.94, 0.55, 0.05, 1.0);
    static ref PURPLE_COLOR:   Color = Color::rgba(0.62, 0.05, 0.94, 1.0);
}

static PLAYER_ONE_NUMBER : usize = 0;
static PLAYER_TWO_NUMBER : usize = 1;
static PLAYER_THREE_NUMBER: usize = 2;
static PLAYER_FOUR_NUMBER: usize = 3;
static PLAYER_FIVE_NUMBER : usize = 4;
static PLAYER_SIX_NUMBER : usize = 5;
static NO_PLAYER : usize = usize::MAX;

#[derive(Clone, Copy)]
struct BoardRegionBoundaryHexCoords {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

// yellow triangle: x in [-4, -1], y in [-4, -1], z in [5, 8]
static BOTTOM_TRIANGLE_BOUNDARY_COORDS : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {
    x_min : -4,
    x_max : -1,
    y_min : -4,
    y_max : -1,
    z_min : 5,
    z_max : 8,
};

// red triangle: x in [-8, -5], y in [1, 4], z in [1, 4]
static BOTTOM_LEFT_TRIANGLE_BOUNDARY_COORDS : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: -8,
    x_max: -5,
    y_min: 1,
    y_max: 4,
    z_min: 1,
    z_max: 4,
};

// blue triangle: x in [1, 4], y in [-5, -8], z in [1, 4]
static BOTTOM_RIGHT_TRIANGLE_BOUNDARY_COORDS : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: 1,
    x_max: 4,
    y_min: -8,
    y_max: -5,
    z_min: 1,
    z_max: 4,
};

// black triangle:  x in [-8, -5], y in [5, 8], z in [-4 ,-1]
static TOP_LEFT_TRIANGLE_BOUNDARY_COORDS : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: -4,
    x_max: -1,
    y_min: 5,
    y_max: 8,
    z_min: -4,
    z_max: -1,
};

// green triangle: x in [5, 8], y in [-4, -1], z in [-4, -1]
static TOP_RIGHT_TRIANGLE_BOUNDARY_COORDS : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: 5,
    x_max: 8,
    y_min: -4,
    y_max: -1,
    z_min: -4,
    z_max: -1,
};

    // //white triangle: x in [1, 4], y in [1, 4], z in [-5, -8]
static TOP_TRIANGLE_BOUNDARY_COORDS : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: 1,
    x_max: 4,
    y_min: 1,
    y_max: 4,
    z_min: -8,
    z_max: -5,
};

    // center squares
static CENTER_REGION_BOUNDARY_COORDS : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    // // center squares
    x_min : -4,
    x_max : 4,
    y_min : -4,
    y_max : 4,
    z_min : -4,
    z_max : 4,
};

#[derive(PartialEq, Clone, Data, Copy, Debug)]
enum AppPage {
    START,
    NEW_GAME,
    JOIN_REMOTE_GAME,
    LOCAL_GAME,
    REMOTE_GAME,
    CREATE_REMOTE_GAME,
    SETTINGS,
}

#[derive(Clone, Copy, Data, PartialEq)]
enum StartingRegion {
    TOP,
    TOP_LEFT,
    TOP_RIGHT,
    BOTTOM_LEFT,
    BOTTOM_RIGHT,
    BOTTOM,
}

impl StartingRegion {
    // returns the opposite region to the given region
    fn opposite(&self) -> StartingRegion {
        match self {
            StartingRegion::TOP => {
                StartingRegion::BOTTOM
            }, 
            StartingRegion::BOTTOM => {
                StartingRegion::TOP
            },
            StartingRegion::TOP_LEFT => {
                StartingRegion::BOTTOM_RIGHT
            },
            StartingRegion::TOP_RIGHT => {
                StartingRegion::BOTTOM_LEFT
            }, 
            StartingRegion::BOTTOM_LEFT => {
                StartingRegion::TOP_RIGHT
            }, 
            StartingRegion::BOTTOM_RIGHT => {
                StartingRegion::TOP_LEFT
            }
            _ => {
                panic!("ERROR: opposite() method of StartingRegion: unrecognized input argument, exiting...");
            }
        }
    }
}


#[derive(PartialEq, Data, Clone, Copy)]
enum PieceColor {
    RED,
    YELLOW,
    GREEN,
    BLUE,
    BLACK,
    WHITE,
    // optional colors below: 
    PURPLE,
    ORANGE,
    GREY
}

impl PieceColor {
    fn to_druid_color(&self) -> &druid::Color {
        match self {
            PieceColor::RED => {
                return &*RED_COLOR;
            }, 
            PieceColor::YELLOW => {
                return &*YELLOW_COLOR;
            },
            PieceColor::BLUE => {
                return &*BLUE_COLOR;
            },
            PieceColor::GREEN => {
                return &*GREEN_COLOR;
            }, 
            PieceColor::BLACK => {
                return &*BLACK_COLOR;
            },
            PieceColor::WHITE => {  
                return &*WHITE_COLOR;
            }, 
            PieceColor::PURPLE => {
                return &*PURPLE_COLOR;
            }, 
            PieceColor::ORANGE => {
                return &*ORANGE_COLOR;
            },
            PieceColor::GREY => {  
                return &*GREY_COLOR;
            },
            _ => {
                panic!("ERROR: unrecognized piece color passed in to to_druid_color(), exiting immediately...");
            }
        }
    }
}

#[derive(PartialEq, Clone, Data, Lens, Copy)]
struct WindowType {
    window_type : AppPage
}

#[derive(PartialEq, Data, Clone, Copy)]
struct Hextile {
    y_hex: i32,
    x_hex: i32,
    z_hex: i32,
    piece_idx: Option<usize>, // the index into the im::Vector<Piece> of the piece that's sitting on this square, if this square is occupied by a piece
}

// use the same pieces over and over again if the user starts a second game
#[derive(PartialEq, Data, Clone, Copy)]
struct Piece {
    player_num: usize,
    hextile_idx: usize, // the index into the im::Vector<Hextile> of the square that this piece is sitting on
    x_hex: i32, // hex coordinates of the square the piece is currently on
    y_hex: i32,
    z_hex: i32
}

// helper methods to convert from hex coordinates to cartesian coordinates
impl Hextile {
    fn cartesian_x(&self) -> f64 {
        let x: f64 = self.x_hex as f64;
        let y: f64 = self.y_hex as f64;
        let z: f64 = self.z_hex as f64;
        return x + z / 2.0;
        //return -y - z / 2.0;
    }

    fn cartesian_y(&self) -> f64 {
        let x: f64 = self.x_hex as f64;
        let y: f64 = self.y_hex as f64;
        let z: f64 = self.z_hex as f64;
        let inner: f64 = 3.0;
        //return -z * (inner).sqrt() / 0.6;
        return -z * inner.sqrt() / 2.0;
    }

    fn get_cartesian_x(x_hex: i32, y_hex: i32, z_hex: i32) -> f64 {
        let x: f64 = x_hex as f64;
        let y: f64 = y_hex as f64;
        let z: f64 = z_hex as f64;
        return x + z / 2.0;
        //return -y - z / 2.0;
    }

    fn get_cartesian_y(x_hex: i32, y_hex: i32, z_hex: i32) -> f64 {
        let x: f64 = x_hex as f64;
        let y: f64 = y_hex as f64;
        let z: f64 = z_hex as f64;
        let inner: f64 = 3.0;
        //return -z * (inner).sqrt() / 0.6;
        return -z * inner.sqrt() / 2.0;
    }

    fn get_tl(&self, data: &AppState) -> Option<usize> {
        for i in 0..data.board.len() {
            let spot = data.board[i];
            if spot.x_hex == self.x_hex && spot.y_hex == self.y_hex + 1 && spot.z_hex == self.z_hex -1 {
                return Some(i);
            }
        }
        return None;
    }

    fn get_tr<'a>(&self, data: &AppState) -> Option<usize> {
        for i in 0..data.board.len() {
            let spot = data.board[i];
            if spot.x_hex == self.x_hex + 1 && spot.y_hex == self.y_hex && spot.z_hex == self.z_hex -1 {
                return Some(i);
            }
        }
        return None;
    }

    fn get_rt<'a>(&self, data: &AppState) -> Option<usize> {
        for i in 0..data.board.len() {
            let spot = data.board[i];
            if spot.x_hex == self.x_hex + 1&& spot.y_hex == self.y_hex - 1 && spot.z_hex == self.z_hex {
                return Some(i);
            }
        }
        return None;
    }

    fn get_lf<'a>(&self, data: &AppState) -> Option<usize> {
        for i in 0..data.board.len() {
            let spot = data.board[i];
            if spot.x_hex == self.x_hex - 1 && spot.y_hex == self.y_hex + 1 && spot.z_hex == self.z_hex {
                return Some(i);
            }
        }
        return None;
    }

    fn get_bl<'a>(&self, data: &AppState) -> Option<usize> {
        for i in 0..data.board.len() {
            let spot = data.board[i];
            if spot.x_hex == self.x_hex - 1 && spot.y_hex == self.y_hex && spot.z_hex == self.z_hex + 1 {
                return Some(i);
            }
        }
        return None;
    }       

    fn get_br<'a>(&self, data: &AppState) -> Option<usize> {
        for i in 0..data.board.len() {
            let spot = data.board[i];
            if spot.x_hex == self.x_hex && spot.y_hex == self.y_hex - 1 && spot.z_hex == self.z_hex + 1 {
                return Some(i);
            }
        }
        return None;
    }

    fn same_hex_coords(&self, tile: Hextile) -> bool {
        return self.x_hex == tile.x_hex && self.y_hex == tile.y_hex && self.z_hex == tile.z_hex;
    }

}

impl Piece {
    fn same_hex_coords(&self, tile: Hextile) -> bool {
        return self.x_hex == tile.x_hex && self.y_hex == tile.y_hex && self.z_hex == tile.z_hex;
    }
}

// Stores which window we're in and the entire state of the game 
#[derive(PartialEq, Clone, Data, Lens)]
struct AppState {
    window_type : AppPage,
    board : im::Vector<Hextile>,
    pieces: im::Vector<Piece>,
    in_game : bool,
    mouse_location_in_canvas : Point,
    player_piece_colors : im::Vector<PieceColor>, // player_piece_colors[i] = piece color of player i,
    whose_turn : Option<usize>,
    last_hopper : Option<Piece>,
    num_players : Option<usize>,
    regions_to_players : im::Vector<StartingRegion>, // regions_to_players[i] = the starting region of player i
    create_remote_game_players_added : Option<Vector<&'static str>>
}

struct MainWidget<T: Data> {
    main_container: Container<T>,
}

struct CanvasWidget {
    piece_is_being_dragged : bool,
    piece_being_dragged : Option<Piece>,
    hextile_over_which_mouse_event_happened : Option<Hextile> // always set to the hextile of the latest mouse event, if it happened within a hextile
}

fn cartesian_x_to_canvas_x(x: f64) -> f64 {
    return (BOARD_WIDTH / 2.0) + (x / (ABSTRACT_BOARD_WIDTH / 2.0)) * (BOARD_WIDTH / 2.0) + (CANVAS_WIDTH - BOARD_WIDTH) / 2.0;
}

fn cartesian_y_to_canvas_y(y: f64) -> f64 {
    return (BOARD_HEIGHT / 2.0) + (-(y / (ABSTRACT_BOARD_HEIGHT / 2.0))) * (BOARD_HEIGHT / 2.0) + BOARD_RECT_VERTICAL_OFFSET_IN_CANVAS;
}

impl CanvasWidget {
    // Returns true iff the Point on the Canvas where a mouse event was triggered inside of a Hextile
    // On the screen each hextile is contained in a 20px x 20px rectangle, so the radius 
    // on the canvas of the circles for the hextiles is 10px
    fn is_within_a_hextile(&mut self, data: &AppState, mouse_event_canvas_coords: Point) -> bool {
        for hextile in data.board.iter() {
            if ((cartesian_x_to_canvas_x(hextile.cartesian_x()) - mouse_event_canvas_coords.x).powi(2) + (cartesian_y_to_canvas_y(hextile.cartesian_y()) - mouse_event_canvas_coords.y).powi(2)).sqrt() < 10.0 {

                self.hextile_over_which_mouse_event_happened = Some(*hextile);

                return true;
            }
        }
        return false;
    }
}
 
// Is 'dest' a tile that can be moved to a single move, and can we move from 'piece' to 'dest' in a single move
fn check_step(start: Hextile, dest: Hextile, data: &AppState) -> bool {
    let tmp_var_tl; // : Option<usize> = None;
    tmp_var_tl = start.get_tl(data);
    if tmp_var_tl.is_some() && data.board[tmp_var_tl.unwrap()].same_hex_coords(dest) {
        return true
    }

    let tmp_var_tr; // : Option<usize> = None;
    tmp_var_tr = start.get_tr(data);
    if tmp_var_tr.is_some() && data.board[tmp_var_tr.unwrap()].same_hex_coords(dest) {
            return true 
    }

    let mut tmp_var_lf : Option<usize> = None;
    tmp_var_lf = start.get_lf(data);
    if tmp_var_lf.is_some() && data.board[tmp_var_lf.unwrap()].same_hex_coords(dest) {
        return true
    }

    let mut tmp_var_rt : Option<usize> = None;
    tmp_var_rt = start.get_rt(data);
    if tmp_var_rt.is_some() && data.board[tmp_var_rt.unwrap()].same_hex_coords(dest) {
            return true 
    }

    let mut tmp_var_bl : Option<usize> = None;
    tmp_var_bl = start.get_bl(data);
    if tmp_var_bl.is_some() && data.board[tmp_var_bl.unwrap()].same_hex_coords(dest) {
        return true 
    }

    let mut tmp_var_br : Option<usize> = None;
    tmp_var_br = start.get_br(data);
    if tmp_var_br.is_some() && data.board[tmp_var_br.unwrap()].same_hex_coords(dest) {
        return true 
    }

    return false;
}

// Dir::top_left -> get_tl()
// Dir::top_right -> get_tr()
// Dir::left -> get_lf()
// ....... 
// fn get_method_handle_for_direction(dir: Direction) -> i32 {
//     return 0;
// }

fn check_hop(start: Hextile, dest: Hextile, data: &AppState) -> bool {
    let mut tmp_var_tl = start.get_tl(data);
    if tmp_var_tl.is_some() && data.board[tmp_var_tl.unwrap()].piece_idx.is_some() {
        tmp_var_tl = data.board[tmp_var_tl.unwrap()].get_tl(data);
        if tmp_var_tl.is_some() && data.board[tmp_var_tl.unwrap()].same_hex_coords(dest) {
            // println!("hop through tl");
            return true;
        }
    }
    let mut tmp_var_tr = start.get_tr(data);
    if tmp_var_tr.is_some() && data.board[tmp_var_tr.unwrap()].piece_idx.is_some() {
        tmp_var_tr = data.board[tmp_var_tr.unwrap()].get_tr(data);
        if tmp_var_tr.is_some() && data.board[tmp_var_tr.unwrap()].same_hex_coords(dest) {
            // println!("hop through tr");
            return true; 
        }
    }
    let mut tmp_var_lf = start.get_lf(data);
    if tmp_var_lf.is_some() && data.board[tmp_var_lf.unwrap()].piece_idx.is_some() {
        tmp_var_lf = data.board[tmp_var_lf.unwrap()].get_lf(data);
        if tmp_var_lf.is_some() && data.board[tmp_var_lf.unwrap()].same_hex_coords(dest) {
            // println!("hop through lf");
            return true;
        }
    }
    let mut tmp_var_br = start.get_br(data);
    if tmp_var_br.is_some() && data.board[tmp_var_br.unwrap()].piece_idx.is_some() {
        tmp_var_br = data.board[tmp_var_br.unwrap()].get_br(data);
        if tmp_var_br.is_some() && data.board[tmp_var_br.unwrap()].same_hex_coords(dest) {
            // println!("hop through br");
            return true;
        }
    }
    let mut tmp_var_bl = start.get_bl(data);
    if tmp_var_bl.is_some() && data.board[tmp_var_bl.unwrap()].piece_idx.is_some() {
        tmp_var_bl = data.board[tmp_var_bl.unwrap()].get_bl(data);
        if tmp_var_bl.is_some() && data.board[tmp_var_bl.unwrap()].same_hex_coords(dest) {
            // println!("hop through bl");
            return true;
        }
    }
    let mut tmp_var_rt = start.get_rt(data);
    if tmp_var_rt.is_some() && data.board[tmp_var_rt.unwrap()].piece_idx.is_some() {
        tmp_var_rt = data.board[tmp_var_rt.unwrap()].get_rt(data);
        if tmp_var_rt.is_some() && data.board[tmp_var_rt.unwrap()].same_hex_coords(dest) {
            // println!("hop through rt");
            return true;
        }
    }
    return false;
}

// fn is_within_region(x: i32, y: i32, z: i32, region: BoardRegionBoundaryHexCoords) -> bool {
//     return x + y + z == 0 &&
//         region.x_min <= x && x <= region.x_max &&
//         region.y_min <= y && y <= region.y_max &&
//         region.z_min <= z && z <= region.z_max
// }

// fn is_within_board(x: i32, y: i32, z: i32) -> bool {
//     return x + y + z == 0 && (is_within_region(x,y,z,BOTTOM_LEFT_TRIANGLE_BOUNDARY_COORDS) ||
//     is_within_region(x, y, z, BOTTOM_RIGHT_TRIANGLE_BOUNDARY_COORDS) ||
//     is_within_region(x, y, z, TOP_LEFT_TRIANGLE_BOUNDARY_COORDS) ||
//     is_within_region(x, y, z, TOP_RIGHT_TRIANGLE_BOUNDARY_COORDS) || 
//     is_within_region(x, y, z, BOTTOM_TRIANGLE_BOUNDARY_COORDS) ||
//     is_within_region(x, y, z, TOP_TRIANGLE_BOUNDARY_COORDS) ||
//     is_within_region(x, y, z, CENTER_REGION_BOUNDARY_COORDS))
// }

// fn get_adjacent(x: i32, y: i32, z: i32) -> Vec<[i32; 3]> {
//     let mut neighbors: Vec<[i32; 3]> = Vec::new();
//     neighbors.push([x, y+1, z-1]); // top left
//     neighbors.push([x+1, y, z-1]); // top right
//     neighbors.push([x, y-1, z+1]); // bottom right
//     neighbors.push([x-1, y+1, z]); // left
//     neighbors.push([x+1, y-1, z]); // right
//     neighbors.push([x-1, y, z+1]); // bottom left

//     let mut i : usize = neighbors.len();
//     while i >= 0 {
//         let pos : [i32; 3] = neighbors[i];
//         if !is_within_board(pos[0], pos[1], pos[2]) {
//             neighbors.remove(i);
//         }
//         i = i - 1;
//     }

//     return neighbors;
// }

impl Widget<AppState> for CanvasWidget {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                println!("in event::MouseDown...");
                if self.is_within_a_hextile(data, mouse_event.pos) {
                    if self.hextile_over_which_mouse_event_happened.unwrap().piece_idx.is_some() {
                        if data.pieces[self.hextile_over_which_mouse_event_happened.unwrap().piece_idx.unwrap()].player_num == data.whose_turn.unwrap() {
                            self.piece_being_dragged = Some(data.pieces[self.hextile_over_which_mouse_event_happened.unwrap().piece_idx.unwrap()]);
                            self.piece_is_being_dragged = true;  
                        } else {
                            self.piece_is_being_dragged = false;
                            self.piece_being_dragged = None;
                        }
                    } else {
                        self.piece_is_being_dragged = false;
                        self.piece_being_dragged = None;                            
                    }
                } else {
                    self.hextile_over_which_mouse_event_happened = None;
                    self.piece_is_being_dragged = false;
                    self.piece_being_dragged = None;
                }
            },
            Event::MouseUp(mouse_event) => {
                if self.piece_is_being_dragged && self.is_within_a_hextile(data, mouse_event.pos) {
                    let starting_square : Hextile;
                    let target_square : Hextile;
                    
                    starting_square = data.board[self.piece_being_dragged.unwrap().hextile_idx]; 
                    target_square = self.hextile_over_which_mouse_event_happened.unwrap();

                    if target_square.piece_idx.is_some() {

                        println!("Error: Square already occupied: please move to an occupied square instead");

                    } else if check_step(starting_square, target_square, data) && data.last_hopper.is_none() {

                        let starting_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(starting_square)).unwrap();
                        let target_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(target_square)).unwrap();
                        let piece_idx : usize = data.pieces.iter().position(|&piece| piece.same_hex_coords(starting_square)).unwrap();

                        let dest_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(target_square)).unwrap();

                        data.board[starting_square_idx].piece_idx = None;
                        data.board[target_square_idx].piece_idx = Some(piece_idx);

                        data.pieces[piece_idx].x_hex = target_square.x_hex;
                        data.pieces[piece_idx].y_hex = target_square.y_hex;
                        data.pieces[piece_idx].z_hex = target_square.z_hex;

                        data.pieces[piece_idx].hextile_idx = dest_square_idx;

                        data.whose_turn = Some((data.whose_turn.unwrap() + 1usize) % data.player_piece_colors.len());

                        data.last_hopper = None;

                    } else if check_hop(starting_square, target_square, data) {
                    
                        println!("making hop move...");

                        let starting_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(starting_square)).unwrap();
                        let target_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(target_square)).unwrap();
                        let piece_idx : usize = data.pieces.iter().position(|&piece| piece.same_hex_coords(starting_square)).unwrap();

                        let dest_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(target_square)).unwrap();

                        if data.last_hopper.is_none() || (data.last_hopper.is_some() && data.last_hopper.unwrap().same_hex_coords(starting_square)) {

                            println!("data.last_hopper is none? {is_none}", is_none = data.last_hopper.is_none());

                            data.board[starting_square_idx].piece_idx = None;
                            data.board[target_square_idx].piece_idx = Some(piece_idx);
    
                            data.pieces[piece_idx].x_hex = target_square.x_hex;
                            data.pieces[piece_idx].y_hex = target_square.y_hex;
                            data.pieces[piece_idx].z_hex = target_square.z_hex;
    
                            data.pieces[piece_idx].hextile_idx = dest_square_idx;
        
                            data.last_hopper = Some(data.pieces[piece_idx]);    
                        }
                    }
                
                } 

                self.piece_is_being_dragged = false;
                self.piece_being_dragged = None;
            }
            Event::MouseMove(mouse_event) => {
                //println!("mouse_x = {mouse_x}, mouse_y = {mouse_y}", mouse_x = mouse_event.window_pos.x, mouse_y = mouse_event.window_pos.y);
                // println!("mouse_x = {mouse_x}, mouse_y = {mouse_y}", mouse_x = mouse_event.pos.x, mouse_y = mouse_event.pos.y);
                data.mouse_location_in_canvas = mouse_event.pos;
                // println!("================================================");
                ctx.request_paint();
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() | bc.is_height_bounded() {
            //println!("Min width = {}", bc.min().width);
            //println!("Min height = {}", bc.min().height);
            //println!("Max width = {}", bc.max().width);
            //println!("Max height = {}", bc.max()data.height);

            let size = Size::new(CANVAS_WIDTH, CANVAS_HEIGHT);
            //bc.constrain(size)
            size
        } else {
            bc.max()
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        // let size = ctx.size();
        // let rect = Rect::from_center_size(Point::new(CANVAS_WIDTH / 2.0, CANVAS_HEIGHT / 2.0), Size::new(BOARD_WIDTH, BOARD_HEIGHT));
        //ctx.fill(rect, &Color::WHITE);

        // let ctx_bounding_rect = ctx.size().to_rect();

        // draw a bounding box around the canvas
        //ctx.stroke(ctx_bounding_rect, &Color::rgba(1.0, 1.0, 1.0, 1.0), 5.0);

        // draw a bounding box around the edges of the board rect
        //ctx.stroke(Rect::from_center_size(Point::new(CANVAS_WIDTH/2.0, CANVAS_HEIGHT/2.0), Size::new(BOARD_WIDTH, BOARD_HEIGHT)), &Color::rgba(1.0, 1.0, 1.0, 1.0), 5.0);

        // draw light brown outer circle of board
        ctx.fill(Circle::new(Point::new(CANVAS_WIDTH / 2.0, CANVAS_HEIGHT / 2.0), BOARD_WIDTH / 2f64), &Color::rgb8(BOARD_CIRCLE_COLOR_r,BOARD_CIRCLE_COLOR_g,BOARD_CIRCLE_COLOR_b));

        // draw an intermediate circle between the outer circle and the pieces
        ctx.stroke(Rect::from_center_size(Point::new(CANVAS_WIDTH/2.0, CANVAS_HEIGHT/2.0)
            , Size::new(cartesian_x_to_canvas_x(ABSTRACT_BOARD_WIDTH / 2.0 - ABSTRACT_INNER_CIRCLE_OFFSET) - cartesian_x_to_canvas_x(-ABSTRACT_BOARD_WIDTH / 2.0 + ABSTRACT_INNER_CIRCLE_OFFSET),
                        cartesian_y_to_canvas_y(-ABSTRACT_BOARD_HEIGHT / 2.0 + ABSTRACT_INNER_CIRCLE_OFFSET) - cartesian_y_to_canvas_y(ABSTRACT_BOARD_HEIGHT / 2.0 - ABSTRACT_INNER_CIRCLE_OFFSET)),
            ).to_ellipse(), &*INTERMEDIATE_CIRCLE_COLOR, 2.0
        );

        // loop through the board, draw each hextile
        // let size_bounds = Size::new(20.0,20.0);
        // let edge_bounds = Size::new(22.0,22.0);

        //ctx.paint_with_z_index(1, move |ctx| {
        
        //println!("Size of board Vec = {}", board.len());

        // let mut x_hex_saved : i32 = 0;
        // let mut y_hex_saved : i32 = 0;
        // let mut z_hex_saved : i32 = 0;
        let mut will_draw_piece_later : bool = false;
        let mut saved_piece_color : Option<&Color> = None;

        for hextile in data.board.iter() {
            //println!("x_hex = {x_hex}, y_hex = {y_hex}, z = {z_hex}", x_hex = hextile.x_hex, y_hex = hextile.y_hex, z_hex = hextile.z_hex);
            //let bounding_rect = Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())),size_bounds);
            //println!("x_screen = {x_screen}, y_screen = {y_screen}", x_screen = screen_x(hextile.cartesian_x()), y_screen = screen_y(hextile.cartesian_y()));

            // draw the square beneath the piece
            ctx.fill(Rect::from_center_size(Point::new(cartesian_x_to_canvas_x(hextile.cartesian_x()), cartesian_y_to_canvas_y(hextile.cartesian_y())), *square_edge_bounds).to_ellipse(), &*SQUARE_COLOR);
        }

        for piece in data.pieces.iter() {
            //ctx.fill(Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())),size_bounds).to_ellipse(), &hextile.c)
            // println!("Painting coordinate: (x, y) = ({cartesian_x}, {cartesian_y})  |  x_hex = {x_hex}, y_hex = {y_hex}, z_hex = {z_hex}", x_hex = hextile.x_hex, y_hex = hextile.y_hex, z_hex = hextile.z_hex, cartesian_x = hextile.cartesian_x(), cartesian_y = hextile.cartesian_y());
            if self.piece_being_dragged.is_some() 
                    && piece.x_hex == self.piece_being_dragged.unwrap().x_hex 
                        && piece.y_hex == self.piece_being_dragged.unwrap().y_hex 
                            && piece.z_hex == self.piece_being_dragged.unwrap().z_hex {
                    
                    // skip over drawing the piece for now, we will draw it later
                    will_draw_piece_later = true;
                    saved_piece_color = Some(data.player_piece_colors[piece.player_num].to_druid_color());

                    // println!("will draw some hextile later!");

            } else {
                // draw the piece in its resting state spot
                // println!("from inside paint(): piece.hextile_idx = {0}, data.board.len() = {1}", piece.hextile_idx, data.board.len());
                ctx.fill(Rect::from_center_size(Point::new(cartesian_x_to_canvas_x(data.board[piece.hextile_idx].cartesian_x()), cartesian_y_to_canvas_y(data.board[piece.hextile_idx].cartesian_y())), *piece_size_bounds).to_ellipse(), data.player_piece_colors[piece.player_num].to_druid_color());
            }
        }

        if will_draw_piece_later {
            // println!("x_hex_saved = {x_hex_saved}, y_hex_saved = {y_hex_saved}, z_hex_saved = {z_hex_saved}", x_hex_saved = x_hex_saved, y_hex_saved = y_hex_saved, z_hex_saved = z_hex_saved);
            // println!("DRAWING THE PIECE!!!");
            ctx.fill(Rect::from_center_size(Point::new(data.mouse_location_in_canvas.x, data.mouse_location_in_canvas.y), *piece_size_bounds).to_ellipse(), saved_piece_color.unwrap());
        }
    }

}

// fn build_page_ui(page: AppPage) -> impl Widget<AppState>
fn build_page_ui(page: AppPage) -> Container<AppState> {
    match page {
        AppPage::CREATE_REMOTE_GAME => {
            // let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
            // let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 

            // let column_layout = Flex::column()
            //     .with_child(
            //         Padding::new(padding_dp,
            //             Label::new("New Remote Game").with_font(font)
            //         )
            //     )
            //     .with_child(
            //         Flex::row()
            //         .with_flex_child(
            //             Flex::column()
            //             .with_child(
            //                 Padding::new(padding_dp,
            //                     Label::new("Add Players")
            //                 )
            //             )
            //             .with_child(
            //                 Padding::new(padding_dp,
            //                     Button::new("Test button").expand_width().expand_height()
            //                 )
            //             ).expand_width()
            //         , 1.0)
            //         .with_flex_spacer(1.0)
            //         .with_flex_child(
            //             Flex::column()
            //             .with_child(
            //                 Padding::new(padding_dp,
            //                     Label::new("Room ID")
            //                 )
            //             )
            //             .with_child(
            //                 Padding::new(padding_dp,
            //                     Button::new("Copy this").expand_width() // TODO replace with textfield
            //                 )
            //             )
            //             .with_child(
            //                 Padding::new(padding_dp,
            //                     Label::new("Registration ticket pastebin")
            //                 )
            //             )
            //             .with_child(
            //                 Padding::new(padding_dp,
            //                     Button::new("Paste here").expand_width()
            //                 )
            //             )
            //         , 1.0)
            //     ).expand_height();

            let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
            let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 

            let mut column_layout = Flex::column()
                .with_child(Button::new("test button"))
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Scroll::new(
                                List::new(|| { 
                                    Flex::row()
                                        .with_child(
                                            Label::new(|(_, item): &(Vector<&str>, &str), _env: &Env| {
                                                format!("{}", item)
                                            })
                                        )
                                        .with_flex_spacer(1.0)
                                        .with_child(
                                            Button::new("-")
                                                .on_click(|_ctx, (list, item): &mut (Vector<&str>, &str), _env| {
                                                    list.retain(|v| v != item) // remove the entry from the list 
                                                })
                                                .fix_size(30.0, 30.0)
                                            //.align_horizontal(UnitPoint::RIGHT)
                                        )
                                        .padding(10.0)
                                        .background(Color::rgb(0.5,0.0,0.5))
                                        .fix_height(50.0)
                                })//.expand_width() //.expand_width().expand_height()
                            )
                            .vertical() // so that the scrolling is vertical, not horizontal
                            .lens(lens::Identity.map(
                                |data: &AppState| {
                                    if data.create_remote_game_players_added.is_some() {                                    
                                        return (data.create_remote_game_players_added.clone().unwrap(), data.create_remote_game_players_added.clone().unwrap());
                                    } else {
                                        return (Vector::new(), Vector::new())
                                    }
                                },
                                |data: &mut AppState, lens_data: (Vector<&str>, Vector<&str>)| {
                                    data.create_remote_game_players_added = Some(lens_data.0)
                                }
                            ))//.expand_width()
                        ,1.0)
                        .with_flex_child(
                            Button::new("add new user").on_click(|ctx, data: &mut AppState, env| {
                            }).expand_width().expand_height()
                        ,1.0)
                        .with_flex_child(
                            Button::new("button 3").expand_width().expand_height()
                        ,1.0)
                , FlexParams::new(1.0, CrossAxisAlignment::Center));

            return Container::new(Align::centered(column_layout))
        },
        AppPage::JOIN_REMOTE_GAME => {
            return Container::new(Align::centered(Flex::column().with_child(Label::new("ATTEMPTED TO JOIN REMOTE GAME"))));
        },
        AppPage::LOCAL_GAME => {
            return Container::new(Align::centered(Flex::column().with_child(Label::new("LOCAL_GAME"))));
        },
        AppPage::REMOTE_GAME => {
            return Container::new(Align::centered(Flex::column().with_child(Label::new("REMOTE_GAME"))));
        },
        AppPage::NEW_GAME => {
            let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
            let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 

            let column_layout = SizedBox::new(Flex::column()
                .with_child(
                    Padding::new(padding_dp,
                        Label::new("New Game").with_font(font)
                    )
                )
                .with_child(
                    Padding::new(padding_dp,
                        Button::new("New Local Game")
                        .on_click(|ctx, data : &mut AppState, env| {
                            data.window_type = AppPage::LOCAL_GAME;
                            println!("New Local Game button pressed....");
                        })
                        .expand_width()
                    )
                )
                .with_child(
                    Padding::new(padding_dp,
                        Button::new("New Remote Game")
                        .on_click(|ctx, data : &mut AppState, env| {
                            data.window_type = AppPage::CREATE_REMOTE_GAME;
                            println!("New Remote Game button pressed....");
                        })
                        .expand_width()
                    )
                )
                .with_child(
                    Padding::new(padding_dp,
                        Button::new("Back")
                        .on_click(|ctx, data : &mut AppState, env| {
                            data.window_type = AppPage::START;
                            println!("Back button pressed from new game page....");
                        })
                        .expand_width()
                    )
                )

            ).width(300.0).expand_height();

            return Container::new(Align::centered(column_layout))
        },
        AppPage::SETTINGS => {
            return Container::new(Align::centered(Flex::column().with_child(Label::new("ATTEMPTED TO ENTER SETTINGS PAGE"))));
        },
        AppPage::START => {
            let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
            let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 
            let column_layout = SizedBox::new(Flex::column()
            .with_child(
                Padding::new(padding_dp, 
                    Label::new("Chinese Checkers").with_font(font)
                )
            )
            .with_child(
                Padding::new(padding_dp, 
                    Button::new("New Game")
                    .on_click(|ctx, data : &mut AppState, env| {
                        data.window_type = AppPage::NEW_GAME;
                        println!("New game button pressed....");
                    })
                    .expand_width()
                )
            )
            .with_child(
                Padding::new(padding_dp, 
                    Button::new("Join Game")
                    .on_click(|ctx, data : &mut AppState, env| {
                        data.window_type = AppPage::JOIN_REMOTE_GAME;
                        println!("Join game button pressed....");
                    })
                    .expand_width()
                )
            )
            .with_child(
                Padding::new(padding_dp, 
                    Button::new("Settings")
                    .expand_width()
                )
            )
            .with_child(
                Padding::new(padding_dp, 
                    Button::new("Quit")
                    .on_click(|ctx, data: &mut AppState, env| {
                        println!("closing the application....");
                        ctx.window().close();
                    })
                    .expand_width()
                )
            )).width(300.0).expand_height();
    
            return Container::new(Align::centered(column_layout));
        }
    }
}

impl MainWidget<AppState> {

    // fn make_start_menu() -> Container<AppState> {
    //     let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
    //     let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 
    //     let column_layout = SizedBox::new(Flex::column()
    //     .with_child(Padding::new(padding_dp, SizedBox::new(Label::new("Chinese Checkers").with_font(font))))
    //     .with_child(Padding::new(padding_dp, SizedBox::new(Button::new("New Game").on_click(|ctx, data : &mut AppState, env| {
    //         data.window_type = AppPage::NEW_GAME;
    //         println!("Single-player button pressed....");
    //     })).expand_width()))
    //     .with_child(Padding::new(padding_dp, SizedBox::new(Button::new("Join Game").on_click(|ctx, data : &mut AppState, env| {
    //         data.window_type = AppPage::JOIN_REMOTE_GAME;
    //         println!("Multi-player button pressed....");
    //     })).expand_width()))
    //     .with_child(Padding::new(padding_dp, SizedBox::new(Button::new("Settings")).expand_width()))
    //     .with_child(Padding::new(padding_dp, SizedBox::new(Button::new("Quit").on_click(|ctx, data: &mut AppState, env| {
    //         println!("closing the application....");
    //         ctx.window().close();
    //     })).expand_width()))).width(300.0);

    //     return Container::new(Align::centered(column_layout));
    // }

    fn new() -> IdentityWrapper<Self> {
        // let padding_dp = (0.0, 10.0); // 4dp of vertical padding, 0dp of horizontal padding 

        // let column_layout = Flex::column()
        //     .with_child(Padding::new(padding_dp, Button::new("Single-Player").on_click(|ctx, data : &mut AppState, env| {
        //         data.window_type = AppPage:: LOCAL_GAME;
        //         println!("Single-player button pressed....");
        //     })))
        //     .with_child(Padding::new(padding_dp, Button::new("Multi-Player").on_click(|ctx, data : &mut AppState, env| {
        //         data.window_type = AppPage::JOIN_MULTIPLAYER_GAME;
        //         println!("Multi-player button pressed....");
        //     })))
        //     .with_child(Padding::new(padding_dp, Button::new("Settings")))
        //     .with_child(Padding::new(padding_dp, Button::new("Feedback")))
        //     .with_child(Padding::new(padding_dp, Button::new("Quit")));
                     
        let main_widget = MainWidget::<AppState> {
            main_container: build_page_ui(AppPage::START)
        };

        let widget_id_holder : MutexGuard<WidgetId> = root_widget_id_guard.lock().unwrap();            
        main_widget.with_id(*widget_id_holder)
        // the mutex will be unlocked here because 'widget_id_holder' is scoped to this block
    } 
}

// struct ApplicationCommandHandler {}

// impl ApplicationCommandHandler {
//     fn new() -> Self {
//         ApplicationCommandHandler {}
//     }
// }

// impl AppDelegate<AppState> for ApplicationCommandHandler {
//     fn event(
//         &mut self,
//         ctx: &mut DelegateCtx<'_>,
//         window_id: WindowId,
//         event: Event,
//         data: &mut AppState,
//         env: &Env
//     ) -> Option<Event> 
//     {
//         return Some(event)
//     }

//     fn command(
//         &mut self,
//         ctx: &mut DelegateCtx,
//         target: Target,
//         cmd: &Command,
//         data: &mut AppState,
//         env: &Env
//     ) -> Handled
//     {
//         // if cmd.is::<AppState>(Selector::new("START_NEW_GAME_WITH_2_PLAYERS")) {
//         //     println!("command to start a new game with 2 players received");
//         //     data.board = Arc::<Vec::<Hextile>>::new(Vec::new());
//         //     data.in_game = true;
//         //     return Handled::Yes;
//         // }
//         // if cmd.is::<AppState>(Selector::new("START_NEW_GAME_WITH_3_PLAYERS")) {
//         //     println!("command to start a new game with 3 players received");
//         //     return Handled::Yes;
//         // }
//         // if cmd.is::<AppState>(Selector::new("START_NEW_GAME_WITH_3_PLAYERS")) {
//         //     println!("command to start a new game with 3 players received");
//         //     return Handled::Yes;
//         // }
//         // if cmd.is::<AppState>(Selector::new("START_NEW_GAME_WITH_4_PLAYERS")) {
//         //     println!("command to start a new game with 4 players received");
//         //     return Handled::Yes;
//         // }
//         // if cmd.is::<AppState>(Selector::new("START_NEW_GAME_WITH_5_PLAYERS")) {
//         //     println!("command to start a new game with 5 players received");
//         //     return Handled::Yes;
//         // }
//         // if cmd.is::<AppState>(Selector::new("START_NEW_GAME_WITH_6_PLAYERS")) {
//         //     println!("command to start a new game with 6 players received");
//         //     return Handled::Yes;
//         // }

//         return Handled::No;
//     }

//     fn window_added(
//         &mut self,
//         id: WindowId,
//         data: &mut AppState,
//         env: &Env,
//         ctx: &mut DelegateCtx
//     ) {}

//     fn window_removed(
//         &mut self,
//         id: WindowId,
//         data: &mut AppState,
//         env: &Env,
//         ctx: &mut DelegateCtx
//     ) {}
// }

impl MainWidget<AppState> {
    // fn create_start_game_popup_window_layout<'a>() -> Label<AppState> {
    //     return Label::<AppState>::new("Enter a number, between 1 and 6");
    // }

    // 1. Create the pieces for the board
    // 2. Link the pieces to the board
    // fn initialize_pieces_for_board(board: Arc<Vec<Hextile>>, pieces: im::Vector<Piece>, num_players: u32) {
    //     if num_players == 6 {

    //     }
    // }   
}

fn get_boundary_coords_struct_for_region(region: StartingRegion) -> BoardRegionBoundaryHexCoords {
    match region {
        StartingRegion::TOP => {
            return TOP_TRIANGLE_BOUNDARY_COORDS;
        }, 
        StartingRegion::TOP_RIGHT => {
            return TOP_RIGHT_TRIANGLE_BOUNDARY_COORDS;
        }, 
        StartingRegion::BOTTOM_RIGHT => {
            return BOTTOM_RIGHT_TRIANGLE_BOUNDARY_COORDS;
        }, 
        StartingRegion::BOTTOM => {
            return BOTTOM_TRIANGLE_BOUNDARY_COORDS;
        },
        StartingRegion::BOTTOM_LEFT => {
            return BOTTOM_LEFT_TRIANGLE_BOUNDARY_COORDS;
        },
        StartingRegion::TOP_LEFT => {
            return TOP_LEFT_TRIANGLE_BOUNDARY_COORDS;
        },
        _ => {
            panic!("Internal Error: get_boundary_coords_struct_for_region(): unrecognized StartingRegion value, exiting immediately....");
        }
    }
}

// returns the index in the board vector of the hextile with coordinates x_hex, y_hex, z_hex, or none if no such hextile with those coordinates exists on the board
fn hextile_idx_at_coordinates(x_hex: i32, y_hex: i32, z_hex: i32, board: &im::Vector<Hextile>) -> Option<usize> {
    let mut hextile : &Hextile;

    for i in 0..board.len() {
        hextile = &board[i];
        if hextile.x_hex == x_hex && hextile.y_hex == y_hex && hextile.z_hex == z_hex {
            return Some(i);
        }
    }
    return None;
}

fn initialize_pieces_for_board(board: &mut im::Vector<Hextile>, pieces: &mut im::Vector<Piece>, num_players: usize, regions_to_players_slice: &[StartingRegion]) {

    println!("From inside initialize_pieces_for_board(): size of board Vec = {x}", x = board.len());

    if num_players == 6 {

        let regions_to_players : [StartingRegion; 6] = regions_to_players_slice.try_into().expect("ERROR: intialize_pieces_for_board(): slice with incorrect length, exiting...");

        for i in 0..6 {
            let starting_region : StartingRegion = regions_to_players[i];

            let player_num = i;

            let boundary_coords = get_boundary_coords_struct_for_region(starting_region);
            
            for x in boundary_coords.x_min..boundary_coords.x_max+1 {
                for y in boundary_coords.y_min..boundary_coords.y_max+1 {
                    for z in boundary_coords.z_min..boundary_coords.z_max+1 {
                        if x + y + z == 0 {
                            // println!("from inside initialize_pieces_for_board(): x_hex={x_hex},y_hex={y_hex},z_hex={z_hex}",x_hex=x,y_hex=y,z_hex=z);

                            let hextile_idx_wrapper : Option<usize> = hextile_idx_at_coordinates(x,y,z,board);

                            if hextile_idx_wrapper.is_none() {
                                println!("from inside initialize_pieces_for_board(), prior to panicking: x_hex={x_hex},y_hex={y_hex},z_hex={z_hex}",x_hex=x,y_hex=y,z_hex=z);
                                panic!("Internal Error: initialize_pieces_for_board(): Unable to find a square on the board with the given hex coordinates. Exiting immediately....");
                            }

                            let hextile_idx = hextile_idx_wrapper.unwrap();
                            
                            let piece : Piece = Piece {
                                player_num: player_num,
                                hextile_idx: hextile_idx,
                                x_hex: board[hextile_idx].x_hex,
                                y_hex: board[hextile_idx].y_hex,
                                z_hex: board[hextile_idx].z_hex,
                            };

                            let piece_idx : usize = pieces.len();

                            pieces.push_back(piece);

                            board[hextile_idx].piece_idx = Some(piece_idx);
                        }
                    }
                }
            }            
        }
    }
}   


impl Widget<AppState> for MainWidget<AppState> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        self.main_container.event(ctx, event, data, _env);

        match event {
            Event::Command(command) => {
                if command.is::<usize>(*start_game_selector) {
                    data.num_players = Some(*command.get_unchecked::<usize>(*start_game_selector));
                    //let num_players : u32 = *command.get_unchecked::<u32>(*start_game_selector);
                    println!("Received a start game command for {} players", data.num_players.unwrap());
                    if data.num_players.unwrap() == 6 {

                        data.board = create_board();

                        data.pieces.clear();

                        let regions_to_players : [StartingRegion; 6] = [
                            // turns proceed clockwise
                            StartingRegion::TOP,
                            StartingRegion::TOP_RIGHT,
                            StartingRegion::BOTTOM_RIGHT,
                            StartingRegion::BOTTOM,
                            StartingRegion::BOTTOM_LEFT,
                            StartingRegion::TOP_LEFT,
                        ];                

                        data.player_piece_colors = vector![
                            PieceColor::RED, 
                            PieceColor::YELLOW,
                            PieceColor::GREEN,
                            PieceColor::BLUE,
                            PieceColor::BLACK,
                            PieceColor::WHITE            
                        ];

                        initialize_pieces_for_board(&mut data.board, &mut data.pieces , data.num_players.unwrap(), &regions_to_players[..]);

                        data.in_game = true;

                        data.regions_to_players = im::vector![regions_to_players[0], regions_to_players[1], regions_to_players[2], regions_to_players[3], regions_to_players[4], regions_to_players[5]];

                        // data.whose_turn = Some(0);
                        data.whose_turn = Some(0);

                        ctx.request_paint();
                    }
                }   
            }
            _ => {} // handle the event as normal
        }
    }

    fn layout(&mut self,  layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, window_type: &AppState, env: &Env) -> Size {
        self.main_container.layout(layout_ctx, bc, window_type, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, window_type: &AppState, env: &Env) {
        self.main_container.lifecycle(ctx, event, window_type, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &AppState, env: &Env) {
        self.main_container.paint(ctx,data,env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx<'_, '_>, old_data: &AppState, data: &AppState, env: &Env) {
        println!("In update() for MainWidget<AppState>....");

        self.main_container.update(ctx, old_data, data, env);

        if data.window_type != old_data.window_type {
            self.main_container = build_page_ui(data.window_type);
            ctx.children_changed();
        }
    }


    // fn update(&mut self, ctx: &mut UpdateCtx<'_, '_>, old_data: &AppState, data: &AppState, env: &Env) {
    //     println!("Update() of MainWidget<AppState> being called..");

    //     self.main_container.update(ctx,old_data,data,env);

    //     if data.window_type != old_data.window_type {
    //         match data.window_type {
    //             AppPage::CREATE_REMOTE_GAME => {
    //                 let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
    //                 let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 
        
    //                 let column_layout = SizedBox::new(Flex::column()
    //                     .with_child(
    //                         Padding::new(padding_dp,
    //                             Label::new("New Remote Game").with_font(font)
    //                         )
    //                     )
    //                     .with_child(
    //                         Flex::row()
    //                         .with_flex_child(
    //                             Flex::column()
    //                             .with_child(
    //                                 Padding::new(padding_dp,
    //                                     Label::new("Add Players")
    //                                 )
    //                             )
    //                             .with_child(
    //                                 Padding::new(padding_dp,
    //                                     Button::new("").expand_width().expand_height()
    //                                 )
    //                             )
    //                         , 0.3333)
    //                         .with_flex_spacer(0.3333)
    //                         .with_flex_child(
    //                             Flex::column()
    //                             .with_child(
    //                                 Padding::new(padding_dp,
    //                                     Label::new("Room ID")
    //                                 )
    //                             )
    //                             .with_child(
    //                                 Padding::new(padding_dp,
    //                                     Button::new("Copy this").expand_width() // TODO replace with textfield
    //                                 )
    //                             )
    //                             .with_child(
    //                                 Padding::new(padding_dp,
    //                                     Label::new("Registration ticket pastebin")
    //                                 )
    //                             )
    //                             .with_child(
    //                                 Padding::new(padding_dp,
    //                                     Button::new("Paste here").expand_width()
    //                                 )
    //                             )
    //                         , 0.3333)
        
    //                     )
    //                 ).width(300.0).expand_height();
        
    //                 self.main_container = Container::new(Align::centered(column_layout))
    //             },
    //             AppPage::JOIN_REMOTE_GAME => {
    //                 self.main_container =  Container::new(Align::centered(Flex::column().with_child(Label::new("ATTEMPTED TO JOIN REMOTE GAME"))));
    //             },
    //             AppPage::LOCAL_GAME => {
    //                 self.main_container = Container::new(Align::centered(Flex::column().with_child(Label::new("LOCAL_GAME"))));
    //             },
    //             AppPage::REMOTE_GAME => {
    //                 self.main_container = Container::new(Align::centered(Flex::column().with_child(Label::new("REMOTE_GAME"))));
    //             },
    //             AppPage::NEW_GAME => {
    //                 let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
    //                 let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 
        
    //                 let column_layout = SizedBox::new(Flex::column()
    //                     .with_child(
    //                         Padding::new(padding_dp,
    //                             Label::new("New Game").with_font(font)
    //                         )
    //                     )
    //                     .with_child(
    //                         Padding::new(padding_dp,
    //                             Button::new("New Local Game")
    //                             .on_click(|ctx, data : &mut AppState, env| {
    //                                 data.window_type = AppPage::LOCAL_GAME;
    //                                 println!("New Local Game button pressed....");
    //                             })
    //                         )
    //                     )
    //                     .with_child(
    //                         Padding::new(padding_dp,
    //                             Button::new("New Remote Game")
    //                             .on_click(|ctx, data : &mut AppState, env| {
    //                                 data.window_type = AppPage::CREATE_REMOTE_GAME;
    //                                 println!("New Remote Game button pressed....");
    //                             })
    //                         )
    //                     )
    //                 ).width(300.0).expand_height();
        
    //                 self.main_container = Container::new(Align::centered(column_layout))
    //             },
    //             AppPage::SETTINGS => {
    //                 self.main_container = Container::new(Align::centered(Flex::column().with_child(Label::new("ATTEMPTED TO ENTER SETTINGS PAGE"))));
    //             },
    //             AppPage::START => {
    //                 let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
    //                 let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 
    //                 let column_layout = SizedBox::new(Flex::column()
    //                 .with_child(
    //                     Padding::new(padding_dp, 
    //                         Label::new("Chinese Checkers").with_font(font)
    //                     )
    //                 )
    //                 .with_child(
    //                     Padding::new(padding_dp, 
    //                         Button::new("New Game")
    //                         .on_click(|ctx, data : &mut AppState, env| {
    //                             data.window_type = AppPage::NEW_GAME;
    //                             println!("New game button pressed....");
    //                         })
    //                         .expand_width()
    //                     )
    //                 )
    //                 .with_child(
    //                     Padding::new(padding_dp, 
    //                         Button::new("Join Game")
    //                         .on_click(|ctx, data : &mut AppState, env| {
    //                             data.window_type = AppPage::JOIN_REMOTE_GAME;
    //                             println!("Join game button pressed....");
    //                         })
    //                         .expand_width()
    //                     )
    //                 )
    //                 .with_child(
    //                     Padding::new(padding_dp, 
    //                         Button::new("Settings")
    //                         .expand_width()
    //                     )
    //                 )
    //                 .with_child(
    //                     Padding::new(padding_dp, 
    //                         Button::new("Quit")
    //                         .on_click(|ctx, data: &mut AppState, env| {
    //                             println!("closing the application....");
    //                             ctx.window().close();
    //                         })
    //                         .expand_width()
    //                     )
    //                 )).width(300.0).expand_height();
            
    //                 self.main_container = Container::new(Align::centered(column_layout));
    //             }
    //         }
    //     }
    // }
}

// Create the main (root) Widget 
fn build_root_widget() -> impl Widget<AppState> {
    MainWidget::<AppState>::new()
}

fn create_board() -> im::Vector<Hextile> {
    let mut board: im::Vector<Hextile> = im::Vector::new();

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

    let yellow_color_array: Color = Color::rgba(0.902, 0.886, 0.110, 1.0);
    let red_color_array: Color = Color::rgba(0.902, 0.110, 0.110, 1.0);
    let blue_color_array: Color = Color::rgba(0.110, 0.110, 0.902, 1.0);
    let green_color_array: Color = Color::rgba(0.059, 0.600, 0.239, 1.0);
    let black_color_array: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);
    let white_color_array: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    let center_color_array: Color = Color::rgba(0.5, 0.5, 0.5, 1.0);

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
    );

    //white triangle: x in [1, 4], y in [1, 4], z in [-5, -8]
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
    );
    println!("Being called from create_board, size of board Vec = {}", board.len());
    return board; 
}

// add the valid tiles in the given range to the board
//fn add_appropriate_hextiles_to_board(mut board: &mut Vec<Hextile>, x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) {
    fn add_appropriate_hextiles_to_board(
        board: &mut im::Vector<Hextile>,
        x_min: i32,
        x_max: i32,
        y_min: i32,
        y_max: i32,
        z_min: i32,
        z_max: i32,
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
                            piece_idx: None,
                        };
                        board.push_back(tile);
                    }
                }
            }
        }
    }

fn main() {
    let main_window = WindowDesc::new(MainWidget::<AppState>::new);

    let initial_state = AppState {whose_turn : None, window_type : AppPage::START, board: im::Vector::new(), 
        in_game: false, mouse_location_in_canvas : Point::new(0.0, 0.0), pieces : vector![], 
        player_piece_colors: im::Vector::new(), last_hopper : None, num_players : None, regions_to_players: im::Vector::new(),
        create_remote_game_players_added: Some(vector!["Tommy", "Karina", "Joseph"])
    };

    //let command_handler = ApplicationCommandHandler::new();

    AppLauncher::with_window(main_window)
        //.delegate(command_handler)
        .launch(initial_state)
        .expect("ERROR: Failed to launch application, exiting immediately....");
}