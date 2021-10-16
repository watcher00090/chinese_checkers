#![feature(mutex_unlock)]

use druid::widget::{Either, MainAxisAlignment, Painter, FillStrat, Svg, SvgData, Controller, RawLabel, TextBox, Scroll ,List, CrossAxisAlignment, SizedBox, Align, Padding, Button, Flex, Container, Label, IdentityWrapper};
use druid::AppLauncher;
use druid::lens::{self, LensExt};
use druid::LocalizedString;

use druid::menu::MenuEventCtx;
use druid::menu::MenuItem;
use druid::menu::Menu;

use std::any::Any;

use druid::Handled;

use druid::WindowId;

use druid::Command;
use druid::Target;
use druid::Code;

use druid::ArcStr;

use crate::lazy_static::__Deref;

use druid::{Point, Rect, FontDescriptor, Color, Selector, Widget, Data, Lens, WindowDesc, EventCtx, DelegateCtx, Event, Env, LayoutCtx, BoxConstraints, LifeCycle, LifeCycleCtx, Size, PaintCtx, UpdateCtx, WidgetId, WidgetExt};
use druid::widget::prelude::*;

use std::sync::{Arc, Mutex, MutexGuard};
use druid::kurbo::{Circle};
use druid::piet::{FontFamily, FontWeight};
use druid::im;
use druid::im::{vector, Vector};
use std::convert::TryInto;

use druid_widget_nursery::DropdownSelect;

use druid::widget::LineBreaking;

use druid::Screen;

// extern crate pem;
//use openssl::rsa::{Rsa, RsaPrivateKeyBuilder};
//use openssl::pkey::PKey;

use core::convert::TryFrom;

//use openssl::pkey::{Private};
//use openssl::bn::BigNum;
use rand::Rng;

use std::iter;

//use openssl::encrypt::{Encrypter, Decrypter};

use tracing::error;

use once_cell::sync::OnceCell;

use druid::text::{Selection, Validation, ValidationError, Formatter};

// Use our modified version of the Checkbox
mod checkbox;
use checkbox::Checkbox;

// Use our modified version of the RadioGroup
mod radio;
use radio::RadioGroup;

mod ColorChangeableLabel;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref whose_turn_FONT : FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(FontWeight::BOLD).with_size(48.0);
}

static mut background_svg_store: Option<Svg> = None;

static DATA_BUF_LEN : usize = 20000;

lazy_static! {
    // Global mutable variable storing the room_id of the remote gameplay room most recently created by this user
    static ref last_room_id : Arc::<Mutex::<String>> = Arc::new(Mutex::<String>::new(String::from("")));
    // Global mutable variable storing the WidgetId of the root widget. 
    static ref root_widget_id_guard : Mutex::<WidgetId> = Mutex::<WidgetId>::new(WidgetId::next());  // global variable always storing the widget id of the root widget
    static ref start_game_selector : Selector<usize> = Selector::new("StartGame");
    static ref piece_size_bounds : Size = Size::new(20.0, 20.0);
    static ref square_edge_bounds : Size = Size::new(26.5, 26.5);
    static ref SQUARE_COLOR : Color = Color::rgb8( 200, 144, 103 );    
    static ref INTERMEDIATE_CIRCLE_COLOR : Color = Color::rgb8( 200, 144, 103 );
    static ref FONT_SIZE_H1 : f64 = 36.0;
    static ref FONT_SIZE_H2 : f64 = 25.0;
    static ref FONT_SIZE_H3 : f64 = 16.0;
    static ref MENU_BUTTON_PADDING : (f64, f64) = (5.0, 10.0);
    static ref TOP_BAR_BUTTON_PADDING : (f64, f64) = (10.0, 10.0);
    static ref ADVANCED_SETTINGS_MENU_ITEMS_PADDING : (f64, f64, f64, f64) = (0.0, 5.0, 0.0, 5.0);
    static ref ADVANCED_SETTINGS_MENU_HEADER_PADDING : (f64, f64, f64, f64) = (0.0, 10.0, 0.0, 5.0);
    static ref ADVANCED_SETTINGS_MENU_SUBHEADER_PADDING : (f64, f64, f64, f64) = (0.0, 10.0, 0.0, 5.0);
    static ref CLOSE_DIALOG_POPUP_OUTER_PADDING : (f64, f64) = (10.0, 2.5);
    static ref DIALOG_POPUP_BUTTONS_CONTAINER_PADDING : (f64, f64) = (5.0, 10.0);
    static ref PUBLIC_KEY  : OnceCell<std::vec::Vec<u8>> = OnceCell::new();
    static ref PRIVATE_KEY : OnceCell<std::vec::Vec<u8>> = OnceCell::new();

    static ref main_window_id : Arc<Mutex<Option<WindowId>>> = Arc::new(<Mutex<Option<WindowId>>>::new(None));

    static ref colored_circle_label_widget_id : Arc::<Mutex::<Option<WidgetId>>> = Arc::new(Mutex::<Option<WidgetId>>::new(None));

    static ref popup_window_id : Arc<Mutex<Option<WindowId>>> = Arc::new(Mutex::<Option<WindowId>>::new(None));
    static ref player_won_window_id : Arc<Mutex<Option<WindowId>>> = Arc::new(Mutex::<Option<WindowId>>::new(None));

    static ref winners_labels_widget_ids : [Arc::<Mutex::<Option::<WidgetId>>>; 6] = [
        Arc::new(Mutex::<Option<WidgetId>>::new(None)),
        Arc::new(Mutex::<Option<WidgetId>>::new(None)),
        Arc::new(Mutex::<Option<WidgetId>>::new(None)),
        Arc::new(Mutex::<Option<WidgetId>>::new(None)),
        Arc::new(Mutex::<Option<WidgetId>>::new(None)),
        Arc::new(Mutex::<Option<WidgetId>>::new(None)),
    ];

    static ref winner_label_id_1st_place_player : Arc::<Mutex::<Option::<WidgetId>>> = Arc::new(Mutex::<Option<WidgetId>>::new(None));
    static ref winner_label_id_2nd_place_player : Arc::<Mutex::<Option::<WidgetId>>> = Arc::new(Mutex::<Option<WidgetId>>::new(None));
    static ref winner_label_id_3rd_place_player : Arc::<Mutex::<Option::<WidgetId>>> = Arc::new(Mutex::<Option<WidgetId>>::new(None));
    static ref winner_label_id_4th_place_player : Arc::<Mutex::<Option::<WidgetId>>> = Arc::new(Mutex::<Option<WidgetId>>::new(None));
    static ref winner_label_id_5th_place_player : Arc::<Mutex::<Option::<WidgetId>>> = Arc::new(Mutex::<Option<WidgetId>>::new(None));
    static ref winner_label_id_6th_place_player : Arc::<Mutex::<Option::<WidgetId>>> = Arc::new(Mutex::<Option<WidgetId>>::new(None));

    static ref DATA_BUF : Arc::<std::vec::Vec<u8>> = Arc::new(vec![0u8; DATA_BUF_LEN]);
    static ref ROOM_ID : OnceCell<String> = OnceCell::<String>::new();

    static ref NULL_DATA : AppState = AppState{
        whose_turn : None,
        window_type : AppPage::Start,
        board: im::Vector::new(), 
        in_game: false,
        display_victory_banner: false, 
        mouse_location_in_canvas : Point::new(0.0, 0.0), 
        pieces : vector![], 
        player_piece_colors: im::Vector::new(), 
        last_hopper : None, 
        num_players : None, 
        regions_to_players: im::Vector::new(),
        create_remote_game_players_added: None, 
        newly_won_player: None,
        players_that_have_won: im::Vector::new(),
        room_id: None,
        join_remote_game_entered_room_id: String::from(""),
        join_remote_game_ticket: None,
        registration_ticket: String::from(" ticket"),
        mouse_click_screen_coordinates: None,
        number_of_players_selected: 0,
        anti_spoiling_rule: AntiSpoilingRule::FilledDest,
        advnset_ranked_winner: false,
        advnset_all_pass_equals_draw: false,
        advnset_three_players_two_triangles: false,
        advnset_two_players_three_triangles: false,
        advnset_forced_move_if_available: false,
        advnset_only_enter_own_dest: false,
        colored_circle_text: Arc::from(CIRCLE_STR),  
        num_consecutive_passes: 0,
        display_draw_banner: false,
        display_game_over_banner: false
    };
}

static CIRCLE_STR : &str = "(\u{2B24})";

static INNER_MENU_CONTAINER_PADDING : (f64, f64) = (10.0, 0.0);
// static INNER_MENU_CONTAINER_PADDING_ADVANCED_SETTINGS_PAGE : (f64, f64) = (25.0, 10.0);
static INNER_MENU_CONTAINER_PADDING_ADVANCED_SETTINGS_PAGE : (f64, f64, f64, f64) = (25.0, 10.0, 25.0, 15.0);


static MIN_WINDOW_WIDTH : f64 = 400f64;
static MIN_WINDOW_HEIGHT: f64 = 400f64;
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

static BOARD_CIRCLE_COLOR_r : u8 = 238;
static BOARD_CIRCLE_COLOR_g : u8 = 206;
static BOARD_CIRCLE_COLOR_b : u8 = 166;

static SWAPPING_ANTI_SPOILING_RULE_TEXT           : &str = "If you are prevented from moving a peg into the destination triangle because some other player's peg is already there, you can swap your peg with that peg. This applies for both a single step move as well as any part of a hop move";
static FILLED_DEST_STRONG_ANTI_SPOILING_RULE_TEXT : &str = "As long as all available squares in the destination triangle are occuiped after the first move, you win";
static FILLED_DEST_WEAK_ANTI_SPOILING_RULE_TEXT   : &str = "As long as all available squares in the destination triangle are occuiped and you have at least one of your pieces in the triangle, you win";

static RANKED_WINNER_CHECKBOX_LABEL_TEXT                              : &str = "Keep playing even after someone has won";
static ALL_PASS_EQUALS_DRAW_CHECKBOX_LABEL_TEXT                       : &str = "If all players pass their turns consecutively, the game is a draw"; 
static THREE_IDENTICAL_CONFIGURATIONS_EQUALS_DRAW_CHECKBOX_LABEL_TEXT : &str = "If the same board state is reached three times, the game is a draw";
static THREE_PLAYERS_TWO_TRIANGLES_CHECKBOX_LABEL_TEXT                : &str = "If starting a three player game, give each player two starting sets of pegs, and victory is only obtained when all a player's starting pegs reach the corresponding respective destination triangles";
static TWO_PLAYERS_THREE_TRIANGLES_CHECKBOX_LABEL_TEXT                : &str = "If starting a two player game, give each player three starting sets of pegs, and victory is only obtained when all a player's starting pegs reach the corresponding respective destination triangles";
static FORCED_MOVE_IF_AVAILABLE_CHECKBOX_LABEL_TEXT                   : &str = "Every turn, players have to make a move if they can, and if they can't they pass";
static ONLY_ENTER_OWN_DEST_CHECKBOX_LABEL_TEXT                        : &str = "You can only enter your own destination triangle";

lazy_static! {
    static ref YELLOW_COLOR:    Color = Color::rgba(0.902, 0.886, 0.110, 1.0);
    static ref RED_COLOR:       Color = Color::rgba(0.902, 0.110, 0.110, 1.0);
    static ref BLUE_COLOR:      Color = Color::rgba(0.110, 0.110, 0.902, 1.0);
    static ref GREEN_COLOR:     Color = Color::rgba(0.059, 0.600, 0.239, 1.0);
    static ref BLACK_COLOR:     Color = Color::rgba(0.0, 0.0, 0.0, 1.0);
    static ref WHITE_COLOR:     Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    static ref GREY_COLOR:      Color = Color::rgba(0.5, 0.5, 0.5, 1.0);    
    static ref ORANGE_COLOR:    Color = Color::rgba(0.94, 0.55, 0.05, 1.0);
    static ref PURPLE_COLOR:    Color = Color::rgba(0.62, 0.05, 0.94, 1.0);
    static ref MENU_GREY:       Color = Color::rgba(0.2, 0.2, 0.2, 1.0);
}

static PLAYER_ONE_NUMBER : usize = 0;
static PLAYER_TWO_NUMBER : usize = 1;
static PLAYER_THREE_NUMBER: usize = 2;
static PLAYER_FOUR_NUMBER: usize = 3;
static PLAYER_FIVE_NUMBER : usize = 4;
static PLAYER_SIX_NUMBER : usize = 5;
static NO_PLAYER : usize = usize::MAX;

static CLOSE_DIALOG_WIDTH : f64 = 300f64;
static CLOSE_DIALOG_HEIGHT: f64 = 150f64;

// Furthest points of the board
// let top : Hextile = Hextile{y_hex : 4, x_hex : 4, z_hex : -8, c : [0.0,0.0,0.0,0.0], p : None};
// let top_left : Hextile = Hextile{y_hex : 8, x_hex : -4, z_hex : -4, c : [0.0,0.0,0.0,0.0], p : None};
// let top_right : Hextile = Hextile{y_hex : -4, x_hex : 8, z_hex : -4, c : [0.0,0.0,0.0,0.0], p : None};
// let bottom : Hextile = Hextile{y_hex : -4, x_hex : -4, z_hex : 8, c : [0.0,0.0,0.0,0.0], p : None};
// let bottom_left : Hextile = Hextile{y_hex : 4, x_hex : -8, z_hex : 4, c : [0.0,0.0,0.0,0.0], p : None};
// let bottom_right : Hextile = Hextile{y_hex : -8, x_hex : 4, z_hex : 4, c : [0.0,0.0,0.0,0.0], p : None};

// Points at the edges of the hexagon
// let hex_top_left : Hextile = Hextile{y_hex : 4, x_hex : 0, z_hex : -4, c : [0.0,0.0,0.0,0.0], p : None};
// let hex_top_right : Hextile = Hextile{y_hex : 0, x_hex : 4, z_hex : -4, c : [0.0,0.0,0.0,0.0], p : None};
// let hex_left : Hextile = Hextile{y_hex : 4, x_hex : -4, z_hex : 0, c : [0.0,0.0,0.0,0.0], p : None};
// let hex_right : Hextile = Hextile{y_hex : -4, x_hex : 4, z_hex : 0, c : [0.0,0.0,0.0,0.0], p : None};
// let hex_bottom_left : Hextile = Hextile{y_hex : 0, x_hex : -4, z_hex : 4, c : [0.0,0.0,0.0,0.0], p : None};
// let hex_bottom_right : Hextile = Hextile{y_hex : -4, x_hex : 0, z_hex : 4, c : [0.0,0.0,0.0,0.0], p : None};

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
static BottomTriangleBoundaryCoords : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {
    x_min : -4,
    x_max : -1,
    y_min : -4,
    y_max : -1,
    z_min : 5,
    z_max : 8,
};

static BottomLeftTriangleBoundaryCoords: BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: -8,
    x_max: -5,
    y_min: 1,
    y_max: 4,
    z_min: 1,
    z_max: 4,
};

static BottomRightTriangleBoundaryCoords : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: 1,
    x_max: 4,
    y_min: -8,
    y_max: -5,
    z_min: 1,
    z_max: 4,
};

static TopLeftTriangleBoundaryCoords : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: -4,
    x_max: -1,
    y_min: 5,
    y_max: 8,
    z_min: -4,
    z_max: -1,
};

static TopRightTriangleBoundaryCoords : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: 5,
    x_max: 8,
    y_min: -4,
    y_max: -1,
    z_min: -4,
    z_max: -1,
};

static TopTriangleBoundaryCoords : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min: 1,
    x_max: 4,
    y_min: 1,
    y_max: 4,
    z_min: -8,
    z_max: -5,
};

// Boundary of the central region
static CenterRegionBoundaryCoords : BoardRegionBoundaryHexCoords = 
BoardRegionBoundaryHexCoords {    
    x_min : -4,
    x_max : 4,
    y_min : -4,
    y_max : 4,
    z_min : -4,
    z_max : 4,
};

#[derive(PartialEq, Clone, Data, Copy, Debug)]
enum AppPage {
    Start,
    NewGame,
    JoinRemoteGame,
    LocalGame,
    RemoteGame,
    CreateLocalGame,
    CreateRemoteGame,
    Settings,
    AdvancedSettings,
}

#[derive(Clone, Copy, Data, PartialEq)]
enum StartingRegion {
    Top,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Bottom,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Data)]
enum PlayerCount {
    TwoPlayerGame,
    ThreePlayerGame,
    FourPlayerGame,
    SixPlayerGame
}

// #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
// enum WidgetType {
//     CheckBox,
//     RadioGroup
// }

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Data)]
enum AntiSpoilingRule {
    Swapping,
    FilledDest,
}

impl BoardRegionBoundaryHexCoords {
    // Returns true iff: the given integers form a hex coord, and the given region contains that coordinate 
    fn contains(&self, x_hex: i32, y_hex: i32, z_hex: i32) -> bool {
        return (x_hex + y_hex + z_hex == 0) && self.x_min <= x_hex && x_hex <= self.x_max && self.y_min <= y_hex && y_hex <= self.y_max && self.z_min <= z_hex && z_hex <= self.z_max; 
    }
}

impl StartingRegion {
    // returns the opposite region to the given region
    fn opposite(&self) -> StartingRegion {
        match self {
            StartingRegion::Top => {
                StartingRegion::Bottom
            }, 
            StartingRegion::Bottom => {
                StartingRegion::Top
            },
            StartingRegion::TopLeft => {
                StartingRegion::BottomRight
            },
            StartingRegion::TopRight => {
                StartingRegion::BottomLeft
            }, 
            StartingRegion::BottomLeft => {
                StartingRegion::TopRight
            }, 
            StartingRegion::BottomRight => {
                StartingRegion::TopLeft
            }
            _ => {
                panic!("ERROR: opposite() method of StartingRegion: unrecognized input argument, exiting...");
            }
        }
    }
}


#[derive(PartialEq, Data, Clone, Copy)]
enum PieceColor {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
    White,
    // optional colors below: 
    Purple,
    Orange,
    Grey
}

impl std::fmt::Debug for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PieceColor::Black => {
                f.write_str("PieceColor::Black")
            },
            PieceColor::Blue => {
                f.write_str("PieceColor::Blue")
            },
            PieceColor::Green => {
                f.write_str("PieceColor::Green")
            },
            PieceColor::Grey => {
                f.write_str("PieceColor::Grey")
            },
            PieceColor::Orange => {
                f.write_str("PieceColor::Orange")
            },
            PieceColor::Purple => {
                f.write_str("PieceColor::Purple")
            },
            PieceColor::Red => {
                f.write_str("PieceColor::Red")
            },
            PieceColor::White => {
                f.write_str("PieceColor::White")
            },
            PieceColor::Yellow => {
                f.write_str("PieceColor::Yellow")
            }
        }
    }
}

impl PieceColor {
    fn to_druid_color(&self) -> &druid::Color {
        match self {
            PieceColor::Red => {
                return &*RED_COLOR;
            }, 
            PieceColor::Yellow => {
                return &*YELLOW_COLOR;
            },
            PieceColor::Blue => {
                return &*BLUE_COLOR;
            },
            PieceColor::Green => {
                return &*GREEN_COLOR;
            }, 
            PieceColor::Black => {
                return &*BLACK_COLOR;
            },
            PieceColor::White => {  
                return &*WHITE_COLOR;
            }, 
            PieceColor::Purple => {
                return &*PURPLE_COLOR;
            }, 
            PieceColor::Orange => {
                return &*ORANGE_COLOR;
            },
            PieceColor::Grey => {  
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
    newly_won_player: Option<usize>,
    display_victory_banner: bool,
    mouse_location_in_canvas : Point,
    player_piece_colors : im::Vector<PieceColor>, // player_piece_colors[i] = piece color of player i,
    whose_turn : Option<usize>,
    last_hopper : Option<Piece>,
    num_players : Option<usize>,
    regions_to_players : im::Vector<StartingRegion>, // regions_to_players[i] = the starting region of player i
    create_remote_game_players_added : Option<Vector<&'static str>>,
    players_that_have_won: im::Vector<usize>,
    room_id: Option<String>,
    join_remote_game_entered_room_id: String,
    join_remote_game_ticket: Option<String>,
    registration_ticket: String,
    mouse_click_screen_coordinates: Option<Point>,
    number_of_players_selected: usize,
    anti_spoiling_rule: AntiSpoilingRule,
    advnset_ranked_winner: bool,
    advnset_all_pass_equals_draw: bool,
    advnset_three_players_two_triangles: bool,
    advnset_two_players_three_triangles: bool,
    advnset_forced_move_if_available: bool,
    advnset_only_enter_own_dest: bool,
    colored_circle_text: ArcStr,
    num_consecutive_passes: i32,
    display_draw_banner: bool,
    display_game_over_banner: bool
}

struct MainWidget<T: Data> {
    main_container: Container<T>,
}

struct CanvasWidget {
    piece_is_being_dragged : bool,
    piece_being_dragged : Option<Piece>,
    hextile_over_which_mouse_event_happened : Option<Hextile>, // always set to the hextile of the latest mouse event, if it happened within a hextile,
    num_moves_made_so_far: usize
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

    // Checks if player 'player_idx' has won. Returns true if yes, false if no.
    fn check_if_won_helper(&self, data: &AppState, player_idx: usize) -> bool {
        let players_to_regions = data.regions_to_players.clone();
        let victory_region = players_to_regions[player_idx].opposite();
        let boundary_coords = boundary_coords_for_region(victory_region);

        if data.anti_spoiling_rule == AntiSpoilingRule::Swapping {
            // Check if all squares in the victory triangle are filled by the pieces of 'player_idx'
            for x_hex in boundary_coords.x_min..boundary_coords.x_max+1 {
                for y_hex in boundary_coords.y_min..boundary_coords.y_max+1 {
                    for z_hex in boundary_coords.z_min..boundary_coords.z_max+1 {
                        if x_hex + y_hex + z_hex == 0 {
                            let tile : Hextile = data.board[hextile_idx_at_coordinates(x_hex, y_hex, z_hex, &data.board).unwrap()];
                            if tile.piece_idx.is_none() {
                                return false;
                            } 
                            if data.pieces[tile.piece_idx.unwrap()].player_num != player_idx {
                                return false;
                            }
                        }
                    }
                }
            }
            return self.num_moves_made_so_far > data.num_players.unwrap();

        } else if data.anti_spoiling_rule == AntiSpoilingRule::FilledDest {
            // Check if all squares in the victory triangle are filled and the victory triangle contains at least one of your pieces
            let mut contains_pieces_of_given_player : bool = false;
            for x_hex in boundary_coords.x_min..boundary_coords.x_max+1 {
                for y_hex in boundary_coords.y_min..boundary_coords.y_max+1 {
                    for z_hex in boundary_coords.z_min..boundary_coords.z_max+1 {
                        if x_hex + y_hex + z_hex == 0 {
                            let tile : Hextile = data.board[hextile_idx_at_coordinates(x_hex, y_hex, z_hex, &data.board).unwrap()];
                            if tile.piece_idx.is_none() {
                                return false;
                            }
                            if data.pieces[tile.piece_idx.unwrap()].player_num == player_idx {
                                contains_pieces_of_given_player = true;
                            } 
                        }
                    }
                }
            }

            return contains_pieces_of_given_player && (self.num_moves_made_so_far > data.num_players.unwrap());

        } else {
            panic!("INTERNAL ERROR: Error in check_if_won, unrecognized anti-spoiling rule, exiting....")
        }
            
        return false;
    }

    // Check if some player has won. If so, returns Some(player_idx). Otherwise returns None.
    fn check_if_won(&self, ctx: &mut EventCtx, data: &mut AppState) -> Option<usize> {
        // let players_to_regions = data.regions_to_players.clone();
        let player_count = data.num_players.unwrap();
        for i in 0..player_count {
            let player_idx = i;
            let has_won = self.check_if_won_helper(data, player_idx);
            if has_won && !data.players_that_have_won.contains(&player_idx) {
                data.players_that_have_won.push_back(player_idx);

                if data.players_that_have_won.len() == 1 {
                    let next_place_player_label_widget_id_mutex = (*winner_label_id_1st_place_player).lock().unwrap();
                    let next_place_player_label_widget_id_option = (*next_place_player_label_widget_id_mutex).clone();
                    let next_place_player_label_widget_id = next_place_player_label_widget_id_option.unwrap();
                    ctx.submit_command(Selector::<PieceColor>::new("SET_COLOR_OF_NEXT_PLACE_PLAYER").with(data.player_piece_colors.get(player_idx).unwrap().clone()).to(druid::Target::Widget(next_place_player_label_widget_id)));
                } else if data.players_that_have_won.len() == 2 {
                    let next_place_player_label_widget_id_mutex = (*winner_label_id_2nd_place_player).lock().unwrap();
                    let next_place_player_label_widget_id_option = (*next_place_player_label_widget_id_mutex).clone();
                    let next_place_player_label_widget_id = next_place_player_label_widget_id_option.unwrap();
                    ctx.submit_command(Selector::<PieceColor>::new("SET_COLOR_OF_NEXT_PLACE_PLAYER").with(data.player_piece_colors.get(player_idx).unwrap().clone()).to(druid::Target::Widget(next_place_player_label_widget_id)));
                } else if data.players_that_have_won.len() == 3 {
                    let next_place_player_label_widget_id_mutex = (*winner_label_id_3rd_place_player).lock().unwrap();
                    let next_place_player_label_widget_id_option = (*next_place_player_label_widget_id_mutex).clone();
                    let next_place_player_label_widget_id = next_place_player_label_widget_id_option.unwrap();
                    ctx.submit_command(Selector::<PieceColor>::new("SET_COLOR_OF_NEXT_PLACE_PLAYER").with(data.player_piece_colors.get(player_idx).unwrap().clone()).to(druid::Target::Widget(next_place_player_label_widget_id)));
                } else if data.players_that_have_won.len() == 4 {
                    let next_place_player_label_widget_id_mutex = (*winner_label_id_4th_place_player).lock().unwrap();
                    let next_place_player_label_widget_id_option = (*next_place_player_label_widget_id_mutex).clone();
                    let next_place_player_label_widget_id = next_place_player_label_widget_id_option.unwrap();
                    ctx.submit_command(Selector::<PieceColor>::new("SET_COLOR_OF_NEXT_PLACE_PLAYER").with(data.player_piece_colors.get(player_idx).unwrap().clone()).to(druid::Target::Widget(next_place_player_label_widget_id)));
                } else if data.players_that_have_won.len() == 5 {
                    let next_place_player_label_widget_id_mutex = (*winner_label_id_5th_place_player).lock().unwrap();
                    let next_place_player_label_widget_id_option = (*next_place_player_label_widget_id_mutex).clone();
                    let next_place_player_label_widget_id = next_place_player_label_widget_id_option.unwrap();
                    ctx.submit_command(Selector::<PieceColor>::new("SET_COLOR_OF_NEXT_PLACE_PLAYER").with(data.player_piece_colors.get(player_idx).unwrap().clone()).to(druid::Target::Widget(next_place_player_label_widget_id)));
                } else { // data.players_that_have_won.len() == 6
                    let next_place_player_label_widget_id_mutex = (*winner_label_id_6th_place_player).lock().unwrap();
                    let next_place_player_label_widget_id_option = (*next_place_player_label_widget_id_mutex).clone();
                    let next_place_player_label_widget_id = next_place_player_label_widget_id_option.unwrap();
                    ctx.submit_command(Selector::<PieceColor>::new("SET_COLOR_OF_NEXT_PLACE_PLAYER").with(data.player_piece_colors.get(player_idx).unwrap().clone()).to(druid::Target::Widget(next_place_player_label_widget_id)));
                }
                ctx.request_layout();

                return Some(player_idx);
            }
        }
        return None
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

fn check_swap(start: Hextile, dest: Hextile, data: &AppState) -> bool {
    return false;
}

fn check_hop(start: Hextile, dest: Hextile, data: &AppState) -> bool {
    let mut tmp_var_tl = start.get_tl(data);
    if tmp_var_tl.is_some() && data.board[tmp_var_tl.unwrap()].piece_idx.is_some() {
        tmp_var_tl = data.board[tmp_var_tl.unwrap()].get_tl(data);
        if tmp_var_tl.is_some() && data.board[tmp_var_tl.unwrap()].same_hex_coords(dest) {
            return true;
        }
    }
    let mut tmp_var_tr = start.get_tr(data);
    if tmp_var_tr.is_some() && data.board[tmp_var_tr.unwrap()].piece_idx.is_some() {
        tmp_var_tr = data.board[tmp_var_tr.unwrap()].get_tr(data);
        if tmp_var_tr.is_some() && data.board[tmp_var_tr.unwrap()].same_hex_coords(dest) {
            return true; 
        }
    }
    let mut tmp_var_lf = start.get_lf(data);
    if tmp_var_lf.is_some() && data.board[tmp_var_lf.unwrap()].piece_idx.is_some() {
        tmp_var_lf = data.board[tmp_var_lf.unwrap()].get_lf(data);
        if tmp_var_lf.is_some() && data.board[tmp_var_lf.unwrap()].same_hex_coords(dest) {
            return true;
        }
    }
    let mut tmp_var_br = start.get_br(data);
    if tmp_var_br.is_some() && data.board[tmp_var_br.unwrap()].piece_idx.is_some() {
        tmp_var_br = data.board[tmp_var_br.unwrap()].get_br(data);
        if tmp_var_br.is_some() && data.board[tmp_var_br.unwrap()].same_hex_coords(dest) {
            return true;
        }
    }
    let mut tmp_var_bl = start.get_bl(data);
    if tmp_var_bl.is_some() && data.board[tmp_var_bl.unwrap()].piece_idx.is_some() {
        tmp_var_bl = data.board[tmp_var_bl.unwrap()].get_bl(data);
        if tmp_var_bl.is_some() && data.board[tmp_var_bl.unwrap()].same_hex_coords(dest) {
            return true;
        }
    }
    let mut tmp_var_rt = start.get_rt(data);
    if tmp_var_rt.is_some() && data.board[tmp_var_rt.unwrap()].piece_idx.is_some() {
        tmp_var_rt = data.board[tmp_var_rt.unwrap()].get_rt(data);
        if tmp_var_rt.is_some() && data.board[tmp_var_rt.unwrap()].same_hex_coords(dest) {
            return true;
        }
    }
    return false;
}

struct GlobalDelegate {}

impl GlobalDelegate {
    fn make() -> Self {
        return GlobalDelegate {};
    }
}

impl druid::AppDelegate<AppState> for GlobalDelegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx<'_>,
        window_id: WindowId,
        event: Event,
        data: &mut AppState,
        env: &Env
    ) -> Option<Event> {
        let event_copy = event.clone();
        match event {
            Event::KeyDown(key_event) => {
                if key_event.code == druid::Code::Escape {
                    // Close the End game popup window
                    let popup_window_id_mutex = (*popup_window_id).lock().unwrap();
                    let popup_window_id_option = (*popup_window_id_mutex).clone();
                    
                    if popup_window_id_option.is_some() {
                        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(druid::Target::Window(popup_window_id_option.unwrap())));
                    }

                    // Close the player won popup window
                    let player_won_window_id_mutex = (*player_won_window_id).lock().unwrap();
                    let player_won_window_id_option = (*player_won_window_id_mutex).clone();
                    
                    if player_won_window_id_option.is_some() {
                        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(druid::Target::Window(player_won_window_id_option.unwrap())));
                    }
                }
            },
            _ => {}
        }
        return Some(event_copy);
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut AppState,
        env: &Env,
        ctx: &mut DelegateCtx<'_>
    ) {
        println!("Closing the main application window....");

        let local_main_window_id_mutex_wrapper = (*main_window_id).lock();
        if local_main_window_id_mutex_wrapper.is_ok() {
            println!("Got to A");
            let local_main_window_id_mutex = local_main_window_id_mutex_wrapper.unwrap();
            let local_main_window_id_option : Option<WindowId> = (*local_main_window_id_mutex).clone();
            Mutex::unlock(local_main_window_id_mutex);
            if local_main_window_id_option.is_some() {

                println!("Got to B");

                let local_main_window_id = local_main_window_id_option.unwrap();
                if id == local_main_window_id {

                    println!("Got to C");
                    // Close all popup windows
                    let popup_window_id_mutex_wrapper = (*popup_window_id).lock();
                    if popup_window_id_mutex_wrapper.is_ok() {
                        let popup_window_id_mutex = popup_window_id_mutex_wrapper.unwrap();
                        let popup_window_id_option = (*popup_window_id_mutex).clone();
                        Mutex::unlock(popup_window_id_mutex);
                        if popup_window_id_option.is_some() {
                            println!("Got to D");
                            ctx.submit_command(druid::commands::CLOSE_WINDOW.to(druid::Target::Window(popup_window_id_option.unwrap())));
                        }
                    }   

                    // Close the player won popup window
                    let player_won_window_id_mutex_wrapper = (*player_won_window_id).lock();
                    if player_won_window_id_mutex_wrapper.is_ok() {
                        println!("Got to E");
                        let player_won_window_id_mutex = player_won_window_id_mutex_wrapper.unwrap();
                        let player_won_window_id_option = (*player_won_window_id_mutex).clone();
                        Mutex::unlock(player_won_window_id_mutex);
                        if player_won_window_id_option.is_some() {
                            println!("Got to F");
                            ctx.submit_command(druid::commands::CLOSE_WINDOW.to(druid::Target::Window(player_won_window_id_option.unwrap())));
                        }
                    }
                }

            }
        }


        // if id == local_main_window_id {
        //     // Close all popup windows
        //     let popup_window_id_mutex = (*popup_window_id).lock().unwrap();
        //     let popup_window_id_option = (*popup_window_id_mutex).clone();
            
        //     if popup_window_id_option.is_some() {
        //         ctx.submit_command(druid::commands::CLOSE_WINDOW.to(druid::Target::Window(popup_window_id_option.unwrap())));
        //     }

        //     // Close the player won popup window
        //     let player_won_window_id_mutex = (*player_won_window_id).lock().unwrap();
        //     let player_won_window_id_option = (*player_won_window_id_mutex).clone();
            
        //     if player_won_window_id_option.is_some() {
        //         ctx.submit_command(druid::commands::CLOSE_WINDOW.to(druid::Target::Window(player_won_window_id_option.unwrap())));
        //     }
        // }
    }
        
    fn command(
        &mut self,
        ctx: &mut DelegateCtx<'_>,
        target: Target,
        cmd: &Command,
        data: &mut AppState,
        env: &Env
    ) -> Handled {
        return Handled::No;
    }       
    
        
    fn window_added(
        &mut self,
        id: WindowId,
        data: &mut AppState,
        env: &Env,
        ctx: &mut DelegateCtx<'_>
    ) {}
}

fn pass_turn(ctx: &mut EventCtx, data: &mut AppState) {
    if data.players_that_have_won.len() <= data.num_players.unwrap() - 1 {
        loop {
            data.whose_turn = Some((data.whose_turn.unwrap() + 1) % data.num_players.unwrap());
            if !data.players_that_have_won.contains(&data.whose_turn.unwrap()) {
                break;
            }
        }
        data.last_hopper = None;
    
        //Update the color of the circle that indicates whose turn it is
        let local_colored_circle_label_widget_id_mutex = (*colored_circle_label_widget_id).lock().unwrap();
        let local_colored_circle_label_widget_id_option = (*local_colored_circle_label_widget_id_mutex).clone();
        let local_colored_circle_label_widget_id = local_colored_circle_label_widget_id_option.unwrap();
    
        let mut cmd = Selector::new("UPDATE_COLORED_CIRCLE_COLOR").with((data.player_piece_colors[data.whose_turn.unwrap()]).clone()).to(druid::Target::Widget(local_colored_circle_label_widget_id));
        ctx.submit_command(cmd);
    
        ctx.request_layout();
    }
    
}

fn indicate_winner(data: &mut AppState, ctx: &mut EventCtx, newly_won_player: Option<usize>) {
    let window_pos : Point = ctx.window().get_position();
    let window_size : Size =  ctx.window().get_size();
    let dialog_popup_position : Point = Point::new(window_pos.x + window_size.width / 2.0 - CLOSE_DIALOG_WIDTH / 2.0, window_pos.y + window_size.height / 2.0 - CLOSE_DIALOG_HEIGHT / 2.0);

    let mut place_str : &'static str = "1st";
    match data.players_that_have_won.len() {
        2 => {
            place_str = "2nd"
        }, 
        3 => {
            place_str = "3rd"
        },
        4 => {
            place_str = "4th"
        },
        5 => {
            place_str = "5th"
        },
        6 => {
            place_str = "6th"
        },
        _ => {}
    }
    
    let label_1 : Label<AppState> = Label::new(format!("Player {} (", newly_won_player.unwrap() + 1));
    
    let label_2 : Label<AppState> = Label::new(format!("\u{2B24}"))
        .with_text_color(data.player_piece_colors[data.newly_won_player.unwrap()].to_druid_color().clone());

    let label_3 : Label<AppState> = Label::new(format!(") has won {place} place!", place = place_str));

    let mut window_desc : WindowDesc<AppState> = WindowDesc::new(Padding::new(*CLOSE_DIALOG_POPUP_OUTER_PADDING, 
        Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(
            Flex::row()
            .main_axis_alignment(MainAxisAlignment::Center)
            .with_flex_spacer(1.0)
            .with_child(
                label_1
            )
            .with_child(
                label_2
            )
            .with_child(
                label_3
            )
            .with_flex_spacer(1.0)
        )
    ))
    .resizable(false)
    .title("Victory!")
    .set_position(dialog_popup_position)
    .window_size(Size::new(CLOSE_DIALOG_WIDTH, CLOSE_DIALOG_HEIGHT));

    window_desc.id = WindowId::next();

    let mut player_won_window_id_mutex = (*player_won_window_id).lock().unwrap();
    (*player_won_window_id_mutex) = Some(window_desc.id);
    
    ctx.new_window(window_desc);

    if data.players_that_have_won.len() <= data.num_players.unwrap() - 2 {
        pass_turn(ctx, data);
    } else { // All places have been assigned
        let k = data.num_players.unwrap();
        let mut tot = k * (k + 1) / 2;
        for i in 0..data.players_that_have_won.len() {
            tot -= data.players_that_have_won[i]+1;
        }
        data.players_that_have_won.push_back(tot-1);
        data.in_game = false;
        data.display_game_over_banner = true;
    }
}

impl Widget<AppState> for CanvasWidget {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                println!("in event::MouseDown...");
                if self.is_within_a_hextile(data, mouse_event.pos) && data.in_game {
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
                    let starting_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(starting_square)).unwrap();
                    let target_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(target_square)).unwrap();
                    let player_to_move = data.whose_turn.unwrap();

                    // Move the piece. make_move
                    let starting_region = boundary_coords_for_region(data.regions_to_players[player_to_move]);
                    let destination_region = boundary_coords_for_region(data.regions_to_players[player_to_move].opposite());
                    if data.advnset_only_enter_own_dest && !starting_region.contains(target_square.x_hex, target_square.y_hex, target_square.z_hex) && !CenterRegionBoundaryCoords.contains(target_square.x_hex, target_square.y_hex, target_square.z_hex) && !destination_region.contains(target_square.x_hex, target_square.y_hex, target_square.z_hex) {
                        return
                    }
                    if target_square.piece_idx.is_some() && data.anti_spoiling_rule != AntiSpoilingRule::Swapping {

                        println!("Error: Square already occupied: please move to an occupied square instead");

                    } else if check_step(starting_square, target_square, data) && data.last_hopper.is_none() {

                        let target_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(target_square)).unwrap();
                        let piece_idx : usize = data.pieces.iter().position(|&piece| piece.same_hex_coords(starting_square)).unwrap();

                        let dest_square_idx : usize = data.board.iter().position(|&tile| tile.same_hex_coords(target_square)).unwrap();

                        // Moving to an empty square
                        if ! target_square.piece_idx.is_some() {

                            data.board[starting_square_idx].piece_idx = None;
                            data.board[target_square_idx].piece_idx = Some(piece_idx);

                            data.pieces[piece_idx].x_hex = target_square.x_hex;
                            data.pieces[piece_idx].y_hex = target_square.y_hex;
                            data.pieces[piece_idx].z_hex = target_square.z_hex;

                            println!("Starting square coordinates: x_hex = {x_hex}, y_hex = {y_hex}, z_hex = {z_hex}", x_hex = starting_square.x_hex, y_hex = starting_square.y_hex, z_hex = starting_square.z_hex);
                            
                            data.pieces[piece_idx].hextile_idx = target_square_idx;

                            data.last_hopper = None;

                            let boundary_coords = boundary_coords_for_region(data.regions_to_players[player_to_move].opposite());
            
                            // START DEBUG CODE
                            // println!("Length of BoardVec = {bvl}, length of PiecesVec = {pvl}", bvl = data.board.len(), pvl = data.pieces.len());
                            // for x_hex in boundary_coords.x_min..boundary_coords.x_max+1 {
                            //     for y_hex in boundary_coords.y_min..boundary_coords.y_max+1 {
                            //         for z_hex in boundary_coords.z_min..boundary_coords.z_max+1 {
                            //             if x_hex + y_hex + z_hex == 0 {
                            //                 println!("x_hex = {x}, y_hex = {y}, z_hex = {z}", x = x_hex, y = y_hex, z = z_hex);
                            //                 let tile : Hextile = data.board[hextile_idx_at_coordinates(x_hex, y_hex, z_hex, &data.board).unwrap()]; 
                            //                 if tile.piece_idx.is_none() {
                            //                     //
                            //                 } else {
                            //                     println!("Player num of piece: {}", data.pieces[tile.piece_idx.unwrap()].player_num);
                            //                 }            
                            //             }
                            //         }
                            //     }
                            // }
                            // println!();
                            // END DEBUG CODE

                            self.num_moves_made_so_far += 1;

                            data.num_consecutive_passes = 0;

                            let newly_won_player = self.check_if_won(ctx, data);

                            if newly_won_player.is_some() {
                                // If the player who just moved won, don't update data.whose_turn: we will use it in the top banner
                                data.newly_won_player = newly_won_player;
                                if !data.advnset_ranked_winner {
                                    data.display_victory_banner = true;
                                    data.in_game = false;
                                } else {
                                    indicate_winner(data, ctx, newly_won_player);
                                }
                            } else {
                                pass_turn(ctx, data)
                            }

                        // Swapping with an opponent's piece in the destination triangle
                        } else if target_square.piece_idx.is_some() && data.anti_spoiling_rule == AntiSpoilingRule::Swapping {
                            let target_player_num : usize = data.pieces[target_square.piece_idx.unwrap()].player_num;
                            let starting_player_num : usize = data.pieces[starting_square.piece_idx.unwrap()].player_num;

                            if target_player_num != starting_player_num && data.regions_to_players[starting_player_num].opposite() == data.regions_to_players[target_player_num] && boundary_coords_for_region(data.regions_to_players[target_player_num]).contains(target_square.x_hex, target_square.y_hex, target_square.z_hex) {
                                // Swap the pieces in the starting square and the target square
    
                                // Update the position of the pieces
                                let x_hex_tmp = data.pieces[starting_square.piece_idx.unwrap()].x_hex;
                                let y_hex_tmp = data.pieces[starting_square.piece_idx.unwrap()].y_hex;
                                let z_hex_tmp = data.pieces[starting_square.piece_idx.unwrap()].z_hex;

                                data.pieces[starting_square.piece_idx.unwrap()].x_hex = data.pieces[target_square.piece_idx.unwrap()].x_hex;
                                data.pieces[starting_square.piece_idx.unwrap()].y_hex = data.pieces[target_square.piece_idx.unwrap()].y_hex;
                                data.pieces[starting_square.piece_idx.unwrap()].z_hex = data.pieces[target_square.piece_idx.unwrap()].z_hex;

                                data.pieces[target_square.piece_idx.unwrap()].x_hex = x_hex_tmp;
                                data.pieces[target_square.piece_idx.unwrap()].y_hex = y_hex_tmp;
                                data.pieces[target_square.piece_idx.unwrap()].z_hex = z_hex_tmp;

                                // Make sure the pieces have pointers to the correect hextiles
                                data.pieces[starting_square.piece_idx.unwrap()].hextile_idx = target_square_idx;
                                data.pieces[target_square.piece_idx.unwrap()].hextile_idx = starting_square_idx;

                                // Make sure the hextiles have pointers to the correct pieces
                                let tmp_piece_idx = data.board[starting_square_idx].piece_idx;
                                data.board[starting_square_idx].piece_idx = data.board[target_square_idx].piece_idx;
                                data.board[target_square_idx].piece_idx = tmp_piece_idx;

                                data.last_hopper = None;

                                let _boundary_coords = boundary_coords_for_region(data.regions_to_players[player_to_move].opposite());
                    
                                self.num_moves_made_so_far += 1;
    
                                data.num_consecutive_passes = 0;

                                let newly_won_player = self.check_if_won(ctx, data);
    
                                if newly_won_player.is_some() {
                                    // If the player who just moved won, don't update data.whose_turn: we will use it in the top banner
                                    data.newly_won_player = newly_won_player;
                                    if !data.advnset_ranked_winner {
                                        data.display_victory_banner = true;
                                        data.in_game = false;
                                    } else {
                                        indicate_winner(data, ctx, newly_won_player);
                                    }
                                } else {
                                    pass_turn(ctx, data)
                                }
        
                            }
                        }

                    } else if check_hop(starting_square, target_square, data) {
                    
                        println!("making hop move...");

                        // Nothing in the target square
                        if ! target_square.piece_idx.is_some() {

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

                                self.num_moves_made_so_far += 1;

                                data.num_consecutive_passes = -1;

                                println!("Got here");

                                let newly_won_player = self.check_if_won(ctx, data);

                                println!("newly won player = {:?}", newly_won_player);

                                if newly_won_player.is_some() {
                                    // If the player who just moved won, don't update data.whose_turn: we will use it in the top banner
                                    data.newly_won_player = newly_won_player;
                                    if !data.advnset_ranked_winner {
                                        data.display_victory_banner = true;
                                        data.in_game = false;
                                    } else {
                                        indicate_winner(data, ctx, newly_won_player);
                                    }
                                }
                            }

                        // Swapping with an opponent's piece in the destination triangle
                        } else if target_square.piece_idx.is_some() && data.anti_spoiling_rule == AntiSpoilingRule::Swapping {
                            let target_player_num : usize = data.pieces[target_square.piece_idx.unwrap()].player_num;
                            let starting_player_num : usize = data.pieces[starting_square.piece_idx.unwrap()].player_num;

                            if target_player_num != starting_player_num && data.regions_to_players[starting_player_num].opposite() == data.regions_to_players[target_player_num] && boundary_coords_for_region(data.regions_to_players[target_player_num]).contains(target_square.x_hex, target_square.y_hex, target_square.z_hex) {
                                // Swap the pieces in the starting square and the target square
    
                                // Update the position of the pieces
                                let x_hex_tmp = data.pieces[starting_square.piece_idx.unwrap()].x_hex;
                                let y_hex_tmp = data.pieces[starting_square.piece_idx.unwrap()].y_hex;
                                let z_hex_tmp = data.pieces[starting_square.piece_idx.unwrap()].z_hex;

                                data.pieces[starting_square.piece_idx.unwrap()].x_hex = data.pieces[target_square.piece_idx.unwrap()].x_hex;
                                data.pieces[starting_square.piece_idx.unwrap()].y_hex = data.pieces[target_square.piece_idx.unwrap()].y_hex;
                                data.pieces[starting_square.piece_idx.unwrap()].z_hex = data.pieces[target_square.piece_idx.unwrap()].z_hex;

                                data.pieces[target_square.piece_idx.unwrap()].x_hex = x_hex_tmp;
                                data.pieces[target_square.piece_idx.unwrap()].y_hex = y_hex_tmp;
                                data.pieces[target_square.piece_idx.unwrap()].z_hex = z_hex_tmp;

                                // Make sure the pieces have pointers to the correect hextiles
                                data.pieces[starting_square.piece_idx.unwrap()].hextile_idx = target_square_idx;
                                data.pieces[target_square.piece_idx.unwrap()].hextile_idx = starting_square_idx;

                                // Make sure the hextiles have pointers to the correct pieces
                                let tmp_piece_idx = data.board[starting_square_idx].piece_idx;
                                data.board[starting_square_idx].piece_idx = data.board[target_square_idx].piece_idx;
                                data.board[target_square_idx].piece_idx = tmp_piece_idx;

                                data.last_hopper = None;

                                let boundary_coords = boundary_coords_for_region(data.regions_to_players[player_to_move].opposite());
                    
                                self.num_moves_made_so_far += 1;

                                data.num_consecutive_passes = 0;
    
                                let newly_won_player = self.check_if_won(ctx, data);
    
                                if newly_won_player.is_some() {
                                    // If the player who just moved won, don't update data.whose_turn: we will use it in the top banner
                                    data.in_game = false;
                                    data.newly_won_player = newly_won_player;
                                    data.display_victory_banner = true;
                                } else { // Pass the turn to the next player because we can't make multiple swaps in the destination triangle in a single turn
                                    pass_turn(ctx, data);
                                }
                            }
                        }

                    }
                
                } 

                self.piece_is_being_dragged = false;
                self.piece_being_dragged = None;
            }
            Event::MouseMove(mouse_event) => {
                data.mouse_location_in_canvas = mouse_event.pos;
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
        let mut will_draw_piece_later : bool = false;
        let mut saved_piece_color : Option<&Color> = None;

        for hextile in data.board.iter() {

            // draw the square beneath the piece
            ctx.fill(Rect::from_center_size(Point::new(cartesian_x_to_canvas_x(hextile.cartesian_x()), cartesian_y_to_canvas_y(hextile.cartesian_y())), *square_edge_bounds).to_ellipse(), &*SQUARE_COLOR);
        }

        for piece in data.pieces.iter() {
            if self.piece_being_dragged.is_some() 
                    && piece.x_hex == self.piece_being_dragged.unwrap().x_hex 
                        && piece.y_hex == self.piece_being_dragged.unwrap().y_hex 
                            && piece.z_hex == self.piece_being_dragged.unwrap().z_hex {
                    
                    // skip over drawing the piece for now, we will draw it later
                    will_draw_piece_later = true;
                    saved_piece_color = Some(data.player_piece_colors[piece.player_num].to_druid_color());

            } else {
                // draw the piece in its resting state spot
                ctx.fill(Rect::from_center_size(Point::new(cartesian_x_to_canvas_x(data.board[piece.hextile_idx].cartesian_x()), cartesian_y_to_canvas_y(data.board[piece.hextile_idx].cartesian_y())), *piece_size_bounds).to_ellipse(), data.player_piece_colors[piece.player_num].to_druid_color());
            }
        }

        if will_draw_piece_later {
            ctx.fill(Rect::from_center_size(Point::new(data.mouse_location_in_canvas.x, data.mouse_location_in_canvas.y), *piece_size_bounds).to_ellipse(), saved_piece_color.unwrap());
        }
    }

}

// Formatter that ensures that the user can't edit the room id in the create_remote_game page
struct RoomIDFormatter<String> {
    base: String
}

impl RoomIDFormatter<String> {
    fn new(input: String) -> Self {
        return RoomIDFormatter::<String>{base: input}
    }
}

impl Formatter<String> for RoomIDFormatter<String> {
    fn format(&self, _: &String) -> String {
        return self.base.clone();
    }

    fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
        if String::from(input) ==  self.base {
            return Validation::success();
        } else {
            return Validation::failure(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        }
    }

    fn value(&self, input: &str) -> Result<String, ValidationError> {
        if String::from(input) == self.base {
            return Ok(self.base.clone())
        } else {
            return Err(ValidationError::new(std::io::Error::from(std::io::ErrorKind::InvalidInput)))
        }
    }

}

struct ChangeLabelColorController {}

impl<AppState, W: Widget<AppState>> Controller<AppState, W> for ChangeLabelColorController where W: ColorChangeableLabel::CanChangeColor {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        println!("In the change color controller");
        match event {
            Event::Command(command) => {
                let update_color_cmd = Selector::new("UPDATE_COLORED_CIRCLE_COLOR");
                if command.is(update_color_cmd) {
                    let new_color_option : Option<&PieceColor> = command.get(update_color_cmd);
                    let new_color : PieceColor = *(new_color_option.unwrap());
                    let new_druid_color : Color = new_color.to_druid_color().clone();
                    println!("Updating the Label's color...");

                    child.set_text_color(new_druid_color.clone());
                }
            },
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}

struct ChangePlacesLabelColorController {}

impl<AppState: Data, W: Widget<AppState>> Controller<AppState, W> for ChangePlacesLabelColorController where W: ColorChangeableLabel::CanChangeColor {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(command) => {
                let cmd = Selector::new("SET_COLOR_OF_NEXT_PLACE_PLAYER");
                if command.is(cmd) {
                    let new_winner_color_option : Option<&PieceColor> = command.get(cmd);
                    let new_winner_color : PieceColor = *(new_winner_color_option.unwrap());
                    let new_winner_druid_color : Color = new_winner_color.to_druid_color().clone();
                    println!("Updating the label's color to {:?}..", new_winner_color);

                    child.set_text_color(new_winner_druid_color.clone());
                }
            },
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}

#[derive(Debug, Default)]
pub struct TextCopyController {}

impl<W: Widget<String>> Controller<String, W> for TextCopyController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        match event {
            other => child.event(ctx, other, data, env)
        }
    }  

    fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &String, data: &String, env: &Env) {
        child.update(ctx, old_data, data, env)
    }
}

impl MainWidget<AppState> {

    fn new() -> IdentityWrapper<Self> {                    
        let main_widget = MainWidget::<AppState> {
            main_container: MainWidget::build_page_ui(AppPage::Start, &*NULL_DATA, &druid::Env::empty()),
        };

        let widget_id_holder : MutexGuard<WidgetId> = root_widget_id_guard.lock().unwrap();      
        main_widget.with_id(*widget_id_holder)
    } 

    fn default_initialize(players_to_regions: &mut std::vec::Vec<StartingRegion>, players_to_colors: &mut std::vec::Vec<PieceColor>, player_count: usize) {
        if players_to_regions.len() != 0 || players_to_colors.len() != 0 {
            panic!("INTERNAL ERROR: in default_initialize, the length of one of the input vectors is nonzero, exiting...")
        }
        if player_count == 6 {
            // Turns proceed clockwise. TODO: add a note about this to a help dialog
            players_to_regions.push(StartingRegion::Top);
            players_to_regions.push(StartingRegion::TopRight);
            players_to_regions.push(StartingRegion::BottomRight);
            players_to_regions.push(StartingRegion::Bottom);
            players_to_regions.push(StartingRegion::BottomLeft);
            players_to_regions.push(StartingRegion::TopLeft);

            players_to_colors.push(PieceColor::Red);
            players_to_colors.push(PieceColor::Yellow);
            players_to_colors.push(PieceColor::Green);
            players_to_colors.push(PieceColor::Blue);
            players_to_colors.push(PieceColor::Black);
            players_to_colors.push(PieceColor::White); 

        } else if player_count == 2 {

            players_to_regions.push(StartingRegion::Top);
            players_to_regions.push(StartingRegion::Bottom);

            players_to_colors.push(PieceColor::White);
            players_to_colors.push(PieceColor::Black);

        } else if player_count == 3 {

            players_to_regions.push(StartingRegion::Top);
            players_to_regions.push(StartingRegion::BottomRight);
            players_to_regions.push(StartingRegion::BottomLeft);

            players_to_colors.push(PieceColor::White);
            players_to_colors.push(PieceColor::Blue);
            players_to_colors.push(PieceColor::Red);

        } else if player_count == 4 {

            players_to_regions.push(StartingRegion::TopRight);
            players_to_regions.push(StartingRegion::BottomRight);
            players_to_regions.push(StartingRegion::BottomLeft);
            players_to_regions.push(StartingRegion::TopLeft);

            players_to_colors.push(PieceColor::White);
            players_to_colors.push(PieceColor::Blue);
            players_to_colors.push(PieceColor::Red);
            players_to_colors.push(PieceColor::Black);

        } else {
            panic!("INTERNAL ERROR: in default_initialize(), unrecognized value for player_count, exiting....")
        }
    }

    fn build_page_ui(page: AppPage, data: &AppState, env: &Env) -> Container<AppState> {
        match page {
            AppPage::JoinRemoteGame => {
                let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
                let padding_dp = (0.0, 10.0); // 0dp of horizontal padding, 10dp of vertical padding,
                
                let chinese_checkers_menu_background_color = (*MENU_GREY).clone(); 
                
                // let partial = DisablePrismWrap::new(
                //     Label::new("Hi"),
                //     String::new(),
                //     Closures(
                //         |_outer| {
                //             return None
                //         },
                //         |_data, _inner| {}
                //     )
                // );

                let inner_menu = SizedBox::new(
                    Padding::new(INNER_MENU_CONTAINER_PADDING, Flex::column()
                        .with_child(
                            Padding::new(padding_dp,
                                Label::new("Join Remote Game").with_font(font)
                            )
                        )
                        .with_child(
                            Flex::column()
                            .cross_axis_alignment(CrossAxisAlignment::Start)
                            .with_child(
                                Label::new("Enter Room ID:")
                            )
                            .with_child(
                                Flex::row()
                                .with_flex_child(
                                    TextBox::new().expand_width().lens(AppState::join_remote_game_entered_room_id)
                                , 1.0)
                                .with_child(
                                    Padding::new((5.0, 0.0), WidgetExt::fix_height(Button::new("Make Ticket"), 30.0))
                                )
                            )
                           // .with_child(
                           //     Flex::row()
                           // )
                        )
                    )
                ).background(chinese_checkers_menu_background_color);
                
                let join_remote_game_page = Flex::column().main_axis_alignment(MainAxisAlignment::Center).with_child(
                    Flex::row().main_axis_alignment(MainAxisAlignment::Center).with_child(WidgetExt::fix_size(inner_menu, 400.0, 400.0))
                );

                let painter = Painter::new(|ctx, data: &AppState, env| {
                    let svg_background = match include_str!("./start-page-background.svg").parse::<SvgData>() {
                        Ok(svg) => svg,
                        Err(err) => {
                            error!("{}", err);
                            error!("Using an empty SVG instead.");
                            SvgData::default()
                        }
                    };
                    Svg::new(svg_background.clone()).fill_mode(FillStrat::Contain).paint(ctx,data,env);        
                });

                return Container::new(join_remote_game_page).background(painter);
            },
            AppPage::CreateLocalGame => {
                let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(*FONT_SIZE_H2).with_weight(FontWeight::BOLD);
                let padding_dp = (0.0, 10.0); // 0dp of horizontal padding, 10dp of vertical padding,
                
                let chinese_checkers_menu_background_color = (*MENU_GREY).clone(); 
                
                let mut number_of_players_dropdown_entries = druid::im::Vector::new();
                number_of_players_dropdown_entries.push_back(("2", PlayerCount::TwoPlayerGame));
                number_of_players_dropdown_entries.push_back(("3", PlayerCount::ThreePlayerGame));
                number_of_players_dropdown_entries.push_back(("4", PlayerCount::FourPlayerGame));
                number_of_players_dropdown_entries.push_back(("6", PlayerCount::SixPlayerGame));

                let inner_menu = SizedBox::new(
                    Padding::new(INNER_MENU_CONTAINER_PADDING, Flex::column()
                        .with_child(
                            Padding::new(padding_dp,
                                Label::new("New Game").with_font(font)
                            )
                        )
                        .with_child(Flex::row()
                            .with_child(Label::new(|data: &AppState, _env: &Env| format!("Number of Players: {}", data.number_of_players_selected)))
                            .with_child(
                                Button::new("Set Player Count")
                                .on_click(|ctx, data: &mut AppState, _env| {
                                    let item2 = MenuItem::<AppState>::new(LocalizedString::new("2")).on_activate(
                                        |ctx: &mut MenuEventCtx, data: &mut AppState, _env: &Env| {
                                            data.number_of_players_selected = 2;
                                        }
                                    );
                                    let item3 = MenuItem::<AppState>::new(LocalizedString::new("3")).on_activate(
                                        |ctx: &mut MenuEventCtx, data: &mut AppState, _env: &Env| { 
                                            data.number_of_players_selected = 3;
                                        }
                                    );
                                    let item4 = MenuItem::<AppState>::new(LocalizedString::new("4")).on_activate(
                                        |ctx: &mut MenuEventCtx, data: &mut AppState, _env: &Env| { 
                                            data.number_of_players_selected = 4;
                                        }
                                    );
                                    let item6 = MenuItem::<AppState>::new(LocalizedString::new("6")).on_activate(
                                        |ctx: &mut MenuEventCtx, data: &mut AppState, _env: &Env| { 
                                            data.number_of_players_selected = 6;
                                        }
                                    );
                                    let new_game_context_menu = Menu::new("How Many Players?").entry(item2).entry(item3).entry(item4).entry(item6);
                                    ctx.show_context_menu(new_game_context_menu, data.mouse_click_screen_coordinates.unwrap());
                                })
                            )
                        )
                        .with_child(
                            Button::new("Advanced Settings").on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                                data.window_type = AppPage::AdvancedSettings;
                            })
                        )
                        .with_child(
                            Button::new("Start Game").on_click(|ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                                data.window_type = AppPage::LocalGame;

                                let player_count = data.number_of_players_selected;
                                data.num_players = Some(player_count);
                                println!("Attempting to start a new game with {} players...", player_count);
                                                    
                                data.display_draw_banner = false;
                                data.display_victory_banner = false;
                                data.display_game_over_banner = false;
                                data.board.clear();
                                data.pieces.clear();
        
                                let mut regions_to_players  = std::vec::Vec::new();
                                let mut player_piece_colors = std::vec::Vec::new();

                                MainWidget::default_initialize(&mut regions_to_players, &mut player_piece_colors, player_count);
                                
                                data.board = create_board();
        
                                initialize_pieces_for_board(&mut data.board, &mut data.pieces , data.num_players.unwrap(), regions_to_players.clone(), player_piece_colors.clone());
        
                                data.in_game = true;
        
                                data.regions_to_players  = im::vector::Vector::from(regions_to_players.clone());
                                data.player_piece_colors = im::vector::Vector::from(player_piece_colors.clone());

                                data.whose_turn = Some(0);

                                data.players_that_have_won.clear();
                                data.last_hopper = None;
                                data.num_consecutive_passes = 0;
        
                                ctx.request_paint();
                            })
                        )
                    )   
                ).background(chinese_checkers_menu_background_color);

                let inner_menu_aligned = Flex::column().main_axis_alignment(MainAxisAlignment::Center).with_child(
                    Flex::row().main_axis_alignment(MainAxisAlignment::Center).with_child(WidgetExt::fix_width(inner_menu, 400.0))
                );

                let create_local_game_page = Flex::column()
                    .with_child(
                        Flex::row()
                        .with_child(Padding::new(*TOP_BAR_BUTTON_PADDING, 
                            Button::new("Back")
                            .on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                                data.window_type = AppPage::Start;
                            })))
                        .with_flex_spacer(1.0)
                        .with_child(Padding::new(*TOP_BAR_BUTTON_PADDING, Button::new("Help")))
                    )
                    .with_flex_spacer(1.0)
                    .with_child(inner_menu_aligned)
                    .with_flex_spacer(1.0);

                let painter = Painter::new(|ctx, data: &AppState, env| {
                    let svg_background = match include_str!("./start-page-background.svg").parse::<SvgData>() {
                        Ok(svg) => svg,
                        Err(err) => {
                            error!("{}", err);
                            error!("Using an empty SVG instead.");
                            SvgData::default()
                        }
                    };
                    Svg::new(svg_background.clone()).fill_mode(FillStrat::Contain).paint(ctx,data,env);        
                });

                return Container::new(create_local_game_page).background(painter);

            },

            AppPage::AdvancedSettings => {
                let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(*FONT_SIZE_H2).with_weight(FontWeight::BOLD);
                
                let chinese_checkers_menu_background_color = (*MENU_GREY).clone(); 
                let little_font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(*FONT_SIZE_H3).with_weight(FontWeight::BOLD);
                
                let inner_menu = SizedBox::new(
                    Padding::new(INNER_MENU_CONTAINER_PADDING_ADVANCED_SETTINGS_PAGE, Flex::column().cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(
                            Padding::new(*ADVANCED_SETTINGS_MENU_HEADER_PADDING,
                                WidgetExt::expand_width(Flex::row().main_axis_alignment(MainAxisAlignment::Center).with_child(Label::new("Advanced Settings").with_font(font.clone())))
                            )
                        )
                        .with_child(
                            Padding::new(*ADVANCED_SETTINGS_MENU_SUBHEADER_PADDING,
                                WidgetExt::expand_width(Label::new("Anti-Spoiling Rules").with_font(little_font.clone()))
                            )
                        )
                        .with_child(
                            RadioGroup::new(vector![(SWAPPING_ANTI_SPOILING_RULE_TEXT, AntiSpoilingRule::Swapping), (FILLED_DEST_WEAK_ANTI_SPOILING_RULE_TEXT, AntiSpoilingRule::FilledDest)]).lens(AppState::anti_spoiling_rule)
                        )
                        .with_child(
                            Padding::new(*ADVANCED_SETTINGS_MENU_SUBHEADER_PADDING,
                                WidgetExt::expand_width(Label::new("Variations").with_font(little_font.clone()))
                            )
                        )
                        .with_child(
                            Padding::new(*ADVANCED_SETTINGS_MENU_ITEMS_PADDING,
                                Checkbox::new(ONLY_ENTER_OWN_DEST_CHECKBOX_LABEL_TEXT).lens(AppState::advnset_only_enter_own_dest)
                            )
                        )
                        .with_child(
                            Padding::new(*ADVANCED_SETTINGS_MENU_ITEMS_PADDING,
                                Checkbox::new(RANKED_WINNER_CHECKBOX_LABEL_TEXT).lens(AppState::advnset_ranked_winner)
                            )
                        )
                        .with_child(
                            Padding::new(*ADVANCED_SETTINGS_MENU_ITEMS_PADDING,
                                Checkbox::new(ALL_PASS_EQUALS_DRAW_CHECKBOX_LABEL_TEXT).lens(AppState::advnset_all_pass_equals_draw)
                            )
                        )
                    )   
                ).background(chinese_checkers_menu_background_color);

                let inner_menu_aligned = Flex::column().main_axis_alignment(MainAxisAlignment::Center).with_child(
                    Flex::row().main_axis_alignment(MainAxisAlignment::Center).with_child(WidgetExt::fix_width(inner_menu, 400.0))
                );

                let create_local_game_page = Flex::column()
                    .with_child(
                        Flex::row()
                        .with_child(Padding::new(*TOP_BAR_BUTTON_PADDING, 
                            Button::new("Back")
                            .on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                                data.window_type = AppPage::CreateLocalGame;
                            })))
                        .with_flex_spacer(1.0)
                        .with_child(Padding::new(*TOP_BAR_BUTTON_PADDING, Button::new("Help")))
                    )
                    .with_flex_spacer(1.0)
                    .with_child(inner_menu_aligned)
                    .with_flex_spacer(1.0);

                let painter = Painter::new(|ctx, data: &AppState, env| {
                    let svg_background = match include_str!("./start-page-background.svg").parse::<SvgData>() {
                        Ok(svg) => svg,
                        Err(err) => {
                            error!("{}", err);
                            error!("Using an empty SVG instead.");
                            SvgData::default()
                        }
                    };
                    Svg::new(svg_background.clone()).fill_mode(FillStrat::Contain).paint(ctx,data,env);        
                });

                return Container::new(create_local_game_page).background(painter);

            },
            AppPage::CreateRemoteGame => {

                let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
                let padding_dp = (0.0, 10.0); // 0dp of horizontal padding, 10dp of vertical padding,
                
                let chinese_checkers_menu_background_color = (*MENU_GREY).clone(); 

                let list_padding = (30.0, 10.0);
                let added_players_label_padding = (0.0, 10.0);
                
                let last_room_id_mutex = (*last_room_id).lock().unwrap();
                let room_id = (*last_room_id_mutex).clone();

                let right_column_item_padding = (0.0, 10.0, 0.0, 10.0);

                let inner_menu = SizedBox::new(
                    Padding::new(INNER_MENU_CONTAINER_PADDING, Flex::column()
                        .with_child(
                            Padding::new(padding_dp,
                                Label::new("New Remote Game").with_font(font)
                            )
                        )
                        .with_child(
                            Flex::row()
                                .cross_axis_alignment(CrossAxisAlignment::Start)
                                .with_flex_child(
                                    Padding::new(list_padding,
                                        Flex::column()
                                            .with_child(
                                                Padding::new(added_players_label_padding, Label::new("Added Players:"))
                                            )
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
                                                                    // .fix_size(30.0, 30.0)
                                                            )
                                                            .padding(10.0)
                                                            //.background(Color::rgba(0.3,0.3,0.3,0.5))
                                                            .background(Color::rgba(0.0,0.0,0.0,0.5))
                                                            .fix_height(50.0)
                                                            .fix_width(100.0)
                                                    })
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
                                                ))
                                            ,1.0)
                                    )
                                , 1.0)
                                //.with_flex_spacer(2.0)
                                .with_flex_child(
                                    Flex::column()
                                    .cross_axis_alignment(CrossAxisAlignment::Start)
                                    .with_child(
                                        Padding::new(right_column_item_padding,
                                            Flex::column()
                                            .cross_axis_alignment(CrossAxisAlignment::Start)
                                            .with_child(Label::new("Room ID:"))
                                            .with_child( 
                                                Flex::row()
                                                .with_flex_child(
                                                    TextBox::with_formatter(TextBox::new(), RoomIDFormatter::new((*ROOM_ID).get().unwrap().clone())).expand_width()
                                                    .lens(
                                                        lens::Map::new(
                                                            |data: &AppState| -> String {
                                                                let res = (*ROOM_ID).get();
                                                                if res.is_some() {
                                                                    let tmp : String = res.unwrap().clone();
                                                                    return tmp;
                                                                } else {
                                                                    println!("ERROR in build_page_ui when page = AppState::CreateRemoteGame: the ROOM_ID OnceCell has not been set yet even though it should have been...");
                                                                    return String::from("");
                                                                }
                                                            },
                                                            |_data: &mut AppState, _lens_data: String| {}
                                                        )
                                                    )
                                                , 1.0)
                                                .with_child(
                                                    WidgetExt::fix_height(
                                                        Padding::new((5.0, 0.0),
                                                            Button::new("Copy")
                                                            .on_click(|ctx, data: &mut AppState, _env| {
                                                                ctx.submit_command(druid::commands::COPY);
                                                            })
                                                        )
                                                    , 30.0)
                                                )
                                            )   
                                        )
                                    )
                                    .with_child(
                                        Padding::new(right_column_item_padding,
                                            Flex::column()
                                            .cross_axis_alignment(CrossAxisAlignment::Start)
                                            .with_child(Label::new("Process Tickets:"))
                                            .with_child(
                                                Flex::row()
                                                .with_flex_child(
                                                    TextBox::new().expand_width().lens(AppState::registration_ticket)
                                                , 1.0)
                                                .with_child(
                                                    WidgetExt::fix_height(
                                                        Padding::new((5.0, 0.0),
                                                            Button::new("Go")
                                                            .on_click(|_ctx, data: &mut AppState, _env| {
                                                                MainWidget::process_registration_ticket(data);
                                                            })
                                                        )
                                                    , 30.0)
                                                ),
                                            )
                                        )
                                    )
                                    .with_child(
                                        Padding::new(right_column_item_padding,
                                            Button::new("Advanced Settings").on_click(|_ctx, data: &mut AppState, _env| {
                                                data.window_type = AppPage::AdvancedSettings;
                                            }).expand_width()
                                        )
                                    )
                                    .with_child(Button::new("Start Game").expand_width())
                            , 1.0)
                        )
                    )
                ).background(chinese_checkers_menu_background_color);
                                
                let inner_menu_aligned = Flex::column().main_axis_alignment(MainAxisAlignment::Center).with_child(
                    Flex::row().main_axis_alignment(MainAxisAlignment::Center).with_child(WidgetExt::fix_size(inner_menu, 400.0, 400.0))
                );

                let create_remote_game_page = Flex::column()
                    .with_child(
                        Flex::row()
                        .with_child(Padding::new(*TOP_BAR_BUTTON_PADDING, 
                            Button::new("Back")
                            .on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                                data.window_type = AppPage::NewGame;
                            })
                        ))
                        .with_flex_spacer(1.0)
                        .with_child(Padding::new(*TOP_BAR_BUTTON_PADDING, Button::new("Help")))
                    )
                    .with_flex_spacer(1.0)
                    .with_child(inner_menu_aligned)
                    .with_flex_spacer(1.0);

                let painter = Painter::new(|ctx, data: &AppState, env| {
                    let svg_background = match include_str!("./start-page-background.svg").parse::<SvgData>() {
                        Ok(svg) => svg,
                        Err(err) => {
                            error!("{}", err);
                            error!("Using an empty SVG instead.");
                            SvgData::default()
                        }
                    };
                    Svg::new(svg_background.clone()).fill_mode(FillStrat::Contain).paint(ctx,data,env);        
                });

                return Container::new(create_remote_game_page).background(painter);

            },
            AppPage::LocalGame => {
                // Store the widget ID in the mutex
                let widget_id = Some(WidgetId::next());
                let mut colored_circle_label_widget_id_mutex = (*colored_circle_label_widget_id).lock().unwrap();
                (*colored_circle_label_widget_id_mutex) = widget_id;

                let local_first_place_player_label_widget_id = Some(WidgetId::next());
                let mut first_place_player_label_widget_id_mutex = (*winner_label_id_1st_place_player).lock().unwrap();
                (*first_place_player_label_widget_id_mutex) = local_first_place_player_label_widget_id;

                let local_second_place_player_label_widget_id = Some(WidgetId::next());
                let mut second_place_player_label_widget_id_mutex = (*winner_label_id_2nd_place_player).lock().unwrap();
                (*second_place_player_label_widget_id_mutex) = local_second_place_player_label_widget_id;
                
                let local_third_place_player_label_widget_id = Some(WidgetId::next());
                let mut third_place_player_label_widget_id_mutex = (*winner_label_id_3rd_place_player).lock().unwrap();
                (*third_place_player_label_widget_id_mutex) = local_third_place_player_label_widget_id;

                let local_fourth_place_player_label_widget_id = Some(WidgetId::next());
                let mut fourth_place_player_label_widget_id_mutex = (*winner_label_id_4th_place_player).lock().unwrap();
                (*fourth_place_player_label_widget_id_mutex) = local_fourth_place_player_label_widget_id;

                let local_fifth_place_player_label_widget_id = Some(WidgetId::next());
                let mut fifth_place_player_label_widget_id_mutex = (*winner_label_id_5th_place_player).lock().unwrap();
                (*fifth_place_player_label_widget_id_mutex) = local_fifth_place_player_label_widget_id;

                let local_sixth_place_player_label_widget_id = Some(WidgetId::next());
                let mut sixth_place_player_label_widget_id_mutex = (*winner_label_id_6th_place_player).lock().unwrap();
                (*sixth_place_player_label_widget_id_mutex) = local_sixth_place_player_label_widget_id;

                return Container::new(
                    Flex::column()
                    .with_child(
                        Flex::row()
                        .with_flex_child(
                            Padding::new(20.0, 
                                Container::new(
                                    Align::centered(
                                        Button::new("Quit").on_click(|ctx, _data: &mut AppState, _env| {

                                            let window_pos : Point = ctx.window().get_position();
                                            let window_size : Size =  ctx.window().get_size();
                                            let dialog_popup_position : Point = Point::new(window_pos.x + window_size.width / 2.0 - CLOSE_DIALOG_WIDTH / 2.0, window_pos.y + window_size.height / 2.0 - CLOSE_DIALOG_HEIGHT / 2.0);

                                            let mut window_desc : WindowDesc<AppState> = WindowDesc::new(Padding::new(*CLOSE_DIALOG_POPUP_OUTER_PADDING, CloseDialogWidget::<AppState>::make()))
                                            .resizable(false)
                                            .title("End Current Game?")
                                            .set_position(dialog_popup_position)
                                            .window_size(Size::new(CLOSE_DIALOG_WIDTH, CLOSE_DIALOG_HEIGHT));

                                            window_desc.id = WindowId::next();

                                            let mut popup_window_id_mutex = (*popup_window_id).lock().unwrap();
                                            (*popup_window_id_mutex) = Some(window_desc.id);
                                            
                                            ctx.new_window(window_desc);
                                        })
                                    )
                                )
                            )
                        , 1.0)
                        .with_flex_child(
                            Padding::new(20.0, 
                                Container::new(
                                    Align::centered(
                                        Button::new("Help").on_click(|ctx: &mut EventCtx, _data: &mut AppState, _env| {
                                            println!("Opening the help dialog...");
                                            let window_desc : WindowDesc<AppState> = WindowDesc::new(|| -> druid::widget::Flex<AppState> {
                                                let main = Flex::column()
                                                .with_child(
                                                    Label::new("Help")
                                                );
                                                return main;
                                            }()).resizable(false)
                                            .title("Help")
                                            .window_size(Size::new(400f64, 200f64));

                                            ctx.new_window(window_desc);
                                        })
                                    )
                                )    
                            )
                        , 1.0)
                    )
                    .with_child(Flex::row()
                        .with_child(Label::<AppState>::dynamic(|data: &AppState, _: &Env| { 
                                if data.whose_turn == None {
                                    return format!("");
                                } else if data.display_game_over_banner {
                                    return format!("The Game has Ended!");
                                } else if data.display_draw_banner { 
                                    return format!("Draw!"); 
                                } else if data.display_victory_banner {
                                    return format!("Player {} has won the game!", data.newly_won_player.unwrap() + 1);
                                } else {
                                    return format!("Player {} \u{fe59}", data.whose_turn.unwrap() + 1);
                                }
                            }).with_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(FontWeight::BOLD).with_size(48.0))
                        )
                        .with_child(
                            WidgetExt::with_id(
                                druid::widget::ControllerHost::new(
                                    ColorChangeableLabel::ColorChangeableLabel::<AppState>::dynamic(|data: &AppState, _: &Env| { 
                                        if data.whose_turn == None {
                                            return format!("");
                                        } else if data.display_draw_banner {
                                            return format!("");
                                        } else if data.display_victory_banner {
                                            return format!("");
                                        } else {
                                            return format!("\u{2B24}");
                                        }
                                    }).with_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(FontWeight::BOLD).with_size(48.0))
                                    .with_text_color(data.player_piece_colors[data.whose_turn.unwrap()].to_druid_color().clone())
                                , ChangeLabelColorController{}), 
                            widget_id.unwrap())
                        )
                        .with_child(Label::<AppState>::dynamic(|data: &AppState, _: &Env| { 
                            if data.whose_turn == None {
                                return format!("");
                            } else if data.display_draw_banner { 
                                return format!(""); 
                            } else if data.display_victory_banner {
                                return format!("");
                            } else {
                                return format!("\u{fe5a} to move");
                            }
                            }).with_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(FontWeight::BOLD).with_size(48.0))
                        )
                    )
                    .with_child(Flex::row()
                        .with_child(Button::new("End Turn").on_click(|ctx, data: &mut AppState, _env| {
                            if data.in_game {
                                data.num_consecutive_passes += 1;
                                let mut incr : usize = 0;
                                if data.num_consecutive_passes > 0 {
                                    incr = data.num_consecutive_passes as usize;
                                }
                                // println!("data.num_consecutive_passes = {passes}, num_players = {players}, data.advnset_three_equals_draw = {set}", passes = data.num_consecutive_passes, players = data.num_players.unwrap(), set = data.advnset_all_pass_equals_draw);
                                if ((data.num_consecutive_passes < 0 && data.players_that_have_won.len() == data.num_players.unwrap() + 1) || (data.num_consecutive_passes >= 0 && data.players_that_have_won.len() + incr == data.num_players.unwrap())) && data.advnset_all_pass_equals_draw {
                                    data.in_game = false;
                                    data.display_draw_banner = true;
                                    println!("Currently displaying draw banner....")
                                } else {
                                    pass_turn(ctx, data);  
                                }            
                            }})
                        )
                    )
                    .with_child(SizedBox::new(CanvasWidget {num_moves_made_so_far: 0, piece_is_being_dragged: false, piece_being_dragged: None, hextile_over_which_mouse_event_happened: None}))
                    .with_child(
                        Either::new(|data: &AppState, _env: &Env| {return data.advnset_ranked_winner && data.players_that_have_won.len() > 0},
                            Flex::column()
                            .with_child(
                                Label::new("Places:")
                                .with_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(FontWeight::BOLD).with_size(36.0))
                            )
                            .with_child(
                                WidgetExt::with_id(    
                                    druid::widget::ControllerHost::new(    
                                        ColorChangeableLabel::ColorChangeableLabel::new(|data: &AppState, _env: &_| {
                                            if data.players_that_have_won.len() < 1 {
                                               return format!("");
                                            }
                                            return format!("Player {}", data.players_that_have_won[0]+1);
                                        }),
                                        ChangePlacesLabelColorController{}
                                    ),  local_first_place_player_label_widget_id.unwrap()
                                )
                            )
                            .with_child(
                                WidgetExt::with_id(
                                    druid::widget::ControllerHost::new(
                                        ColorChangeableLabel::ColorChangeableLabel::new(|data: &AppState, _env: &_| {
                                            if data.players_that_have_won.len() < 2 {
                                                return format!("");
                                            }
                                            return format!("Player {}", data.players_that_have_won[1]+1);
                                        }),
                                        ChangePlacesLabelColorController{}
                                    ), local_second_place_player_label_widget_id.unwrap()
                                )
                            )
                            .with_child(
                                WidgetExt::with_id(
                                    druid::widget::ControllerHost::new(
                                        ColorChangeableLabel::ColorChangeableLabel::new(|data: &AppState, _env: &_| { 
                                            if data.players_that_have_won.len() < 3 {
                                                return format!("");
                                            }
                                            return format!("Player {}", data.players_that_have_won[2]+1);
                                        }), 
                                        ChangePlacesLabelColorController{}
                                    ), local_third_place_player_label_widget_id.unwrap()
                                )
                            )
                            .with_child(
                                WidgetExt::with_id(
                                    druid::widget::ControllerHost::new(
                                        ColorChangeableLabel::ColorChangeableLabel::new(|data: &AppState, _env: &_| {
                                            if data.players_that_have_won.len() < 4 {
                                                return format!("");
                                            }
                                            format!("Player {}", data.players_that_have_won[3]+1)
                                        }), 
                                        ChangePlacesLabelColorController{}      
                                    ), local_fourth_place_player_label_widget_id.unwrap()
                                )
                            )
                            .with_child(
                                WidgetExt::with_id(
                                    druid::widget::ControllerHost::new(
                                        ColorChangeableLabel::ColorChangeableLabel::new(|data: &AppState, _env: &_| {
                                            if data.players_that_have_won.len() < 5 {
                                                return format!("");
                                            }
                                            format!("Player {}", data.players_that_have_won[4]+1)
                                        }), ChangePlacesLabelColorController{}
                                    ), local_fifth_place_player_label_widget_id.unwrap()
                                )
                            )
                            .with_child(
                                WidgetExt::with_id(
                                    druid::widget::ControllerHost::new(
                                        ColorChangeableLabel::ColorChangeableLabel::new(|data: &AppState, _env: &_| {
                                            if data.players_that_have_won.len() < 6 {
                                                return format!("");
                                            }
                                            format!("Player {}", data.players_that_have_won[5]+1)
                                        }), ChangePlacesLabelColorController{}
                                    ), local_sixth_place_player_label_widget_id.unwrap()
                                )
                            ),
                            SizedBox::empty()
                        )
                    )
                );
            },
            AppPage::RemoteGame => {
                return Container::new(Align::centered(Flex::column().with_child(Label::new("RemoteGame"))));
            },
            AppPage::NewGame => {
                let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
                let padding_dp = (0.0, 10.0); // 0dp of horizontal padding, 10dp of vertical padding,
                
                let chinese_checkers_menu_background_color = (*MENU_GREY).clone(); 
                
                let inner_menu = SizedBox::new(
                    Padding::new(INNER_MENU_CONTAINER_PADDING, Flex::column()
                        .with_child(
                            Padding::new(padding_dp,
                                Label::new("New Game").with_font(font)
                            )
                        )
                        .with_child(
                            Padding::new(padding_dp,
                                WidgetExt::fix_width(
                                    Button::new("New Local Game")
                                    .on_click(|_ctx, data : &mut AppState, _env| {
                                        data.window_type = AppPage::CreateLocalGame;
                                        println!("New Local Game button pressed....");
                                    })
                                , 300.0)
                            )
                        )
                        .with_child(
                            Padding::new(padding_dp,
                                WidgetExt::fix_width(
                                    Button::new("New Remote Game")
                                    .on_click(|_ctx, data : &mut AppState, _env| {
                                        data.window_type = AppPage::CreateRemoteGame;
                                        println!("New Remote Game button pressed....");
                                    })
                                , 300.0)
                            )
                        )
                        .with_child(
                            Padding::new(padding_dp,
                                WidgetExt::fix_width(
                                    Button::new("Back")
                                    .on_click(|_ctx, data : &mut AppState, _env| {
                                        data.window_type = AppPage::Start;
                                        println!("Back button pressed from new game page....");
                                    })
                                , 300.0)
                            )
                        )
                    )
                ).background(chinese_checkers_menu_background_color);
                
                let new_game_page = Flex::column().main_axis_alignment(MainAxisAlignment::Center).with_child(
                    Flex::row().main_axis_alignment(MainAxisAlignment::Center).with_child(inner_menu)
                );

                let painter = Painter::new(|ctx, data: &AppState, env| {
                    let svg_background = match include_str!("./start-page-background.svg").parse::<SvgData>() {
                        Ok(svg) => svg,
                        Err(err) => {
                            error!("{}", err);
                            error!("Using an empty SVG instead.");
                            SvgData::default()
                        }
                    };
                    Svg::new(svg_background.clone()).fill_mode(FillStrat::Contain).paint(ctx,data,env);        
                });

                return Container::new(new_game_page).background(painter);
            },
            AppPage::Settings => {
                return Container::new(Align::centered(Flex::column().with_child(Label::new("ATTEMPTED TO ENTER Settings PAGE"))));
            },
            AppPage::Start => {
                let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(36.0).with_weight(FontWeight::BOLD);
                let padding_dp = (0.0, 10.0); // 0dp of horizontal padding, 10dp of vertical padding,
                
                let chinese_checkers_menu_background_color = (*MENU_GREY).clone();  
                
                let inner_menu = SizedBox::new(Padding::new(INNER_MENU_CONTAINER_PADDING, Flex::column()
                    .with_child(
                        Padding::new((0.0, 10.0, 0.0, 5.0), 
                            Label::new("Chinese Checkers").with_font(font)
                        )
                    )
                    .with_child(
                        Padding::new(padding_dp, 
                            WidgetExt::fix_width(
                                Button::new("New Game")
                                .on_click(|_ctx, data : &mut AppState, _env| {
                                    data.window_type = AppPage::CreateLocalGame;
                                })
                            , 300.0)
                        )
                    )
                    // .with_child(
                    //     Padding::new(padding_dp, 
                    //         WidgetExt::fix_width(
                    //             Button::new("Join Game")
                    //             .on_click(|_ctx, data : &mut AppState, _env| {
                    //                 data.window_type = AppPage::JoinRemoteGame;
                    //                 println!("Join game button pressed....");
                    //             })
                    //         , 300.0)
                    //     )
                    // )
                    .with_child(
                        Padding::new(padding_dp, 
                            WidgetExt::fix_width(
                                Button::new("Settings")
                            , 300.0)
                        )
                    )
                    .with_child(
                        Padding::new((0.0, 10.0, 0.0, 20.0), 
                            WidgetExt::fix_width(
                                Button::new("Quit")
                                .on_click(|ctx, _data: &mut AppState, _env| {
                                    println!("closing the application....");
                                    ctx.window().close();
                                })
                            , 300.0)
                        )
                    )
                )).background(chinese_checkers_menu_background_color);
                
                let start_page = Flex::column().main_axis_alignment(MainAxisAlignment::Center).with_child(
                    Flex::row().main_axis_alignment(MainAxisAlignment::Center).with_child(inner_menu)
                );

                let painter = Painter::new(|ctx, data: &AppState, env| {
                    let svg_background = match include_str!("./start-page-background.svg").parse::<SvgData>() {
                        Ok(svg) => svg,
                        Err(err) => {
                            error!("{}", err);
                            error!("Using an empty SVG instead.");
                            SvgData::default()
                        }
                    };
                    Svg::new(svg_background.clone()).fill_mode(FillStrat::Contain).paint(ctx,data,env);        
                });

                return Container::new(start_page).background(painter);
            }
        }
    }

    fn process_registration_ticket(data: &mut AppState) {

    }

}

fn boundary_coords_for_region(region: StartingRegion) -> BoardRegionBoundaryHexCoords {
    match region {
        StartingRegion::Top => {
            return TopTriangleBoundaryCoords;
        }, 
        StartingRegion::TopRight => {
            return TopRightTriangleBoundaryCoords;
        }, 
        StartingRegion::BottomRight => {
            return BottomRightTriangleBoundaryCoords;
        }, 
        StartingRegion::Bottom => {
            return BottomTriangleBoundaryCoords;
        },
        StartingRegion::BottomLeft => {
            return BottomLeftTriangleBoundaryCoords;
        },
        StartingRegion::TopLeft => {
            return TopLeftTriangleBoundaryCoords;
        },
        _ => {
            panic!("Internal Error: boundary_coords_for_region(): unrecognized StartingRegion value, exiting immediately....");
        }
    }
}

// Returns the index into the board Vector of the Hextile with hex coordinates (x_hex, y_hex, z_hex), or None if no hextile with those coordinates is present
fn hextile_idx_at_coordinates(x_hex: i32, y_hex: i32, z_hex: i32, board: &im::Vector<Hextile>) -> Option<usize> {
    for i in 0..board.len() {
        let mut hextile : &Hextile = &board[i];
        if hextile.x_hex == x_hex && hextile.y_hex == y_hex && hextile.z_hex == z_hex {
            return Some(i);
        }
    }
    return None;
}

fn initialize_pieces_for_board(board: &mut im::Vector<Hextile>, pieces: &mut im::Vector<Piece>, num_players: usize, players_to_regions_vec: std::vec::Vec<StartingRegion>, players_to_colors_vec: std::vec::Vec<PieceColor>) {

    if players_to_regions_vec.len() != players_to_colors_vec.len() {
        panic!("INTERNAL ERROR: in initialize_pieces_for_board, playersToRegions.size() != playersToColors.size(), exiting....")
    }

    println!("From inside initialize_pieces_for_board(): size of board Vec = {x}", x = board.len());

    let players_to_regions: &[StartingRegion] = &players_to_regions_vec;
    let players_to_colors:  &[PieceColor] = &players_to_colors_vec;

    for i in 0..num_players {
        let player_num = i;
        let starting_region : StartingRegion = players_to_regions[i];
        let boundary_coords = boundary_coords_for_region(starting_region);
        let player_color : PieceColor = players_to_colors[i];
        
        for x in boundary_coords.x_min..boundary_coords.x_max+1 {
            for y in boundary_coords.y_min..boundary_coords.y_max+1 {
                for z in boundary_coords.z_min..boundary_coords.z_max+1 {
                    if x + y + z == 0 {
                        let hextile_idx_wrapper : Option<usize> = hextile_idx_at_coordinates(x, y, z,board);

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

struct CloseDialogWidget<AppState> {
    inner: druid::widget::Flex<AppState>
}

impl CloseDialogWidget<AppState> {
    fn make() -> Self {
        let inner =  Flex::column()
        .with_child(
            Label::new("A game is in progress. Would you like to end the current game and return to the start page?")
            .with_line_break_mode(LineBreaking::WordWrap)
            .expand_width()
        )
        .with_child(Padding::new(*DIALOG_POPUP_BUTTONS_CONTAINER_PADDING,
            Flex::row()
            .with_flex_child(
                Flex::row()
                .with_child(
                    Button::new("Yes").on_click(|ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                        println!("Yes button pressed...");
                        data.window_type = AppPage::Start;
                        data.board.clear();
                        data.pieces.clear();
                        data.player_piece_colors.clear();
                        data.in_game = false;
                        data.whose_turn = None;
                        data.last_hopper = None;
                        data.num_players = None;
                        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(Target::Auto));
                    })
                )
                .main_axis_alignment(MainAxisAlignment::Center)
                .expand_width()
            , 1.0)
            .with_flex_child(
                Flex::row()
                .with_child(
                    Button::new("No").on_click(|ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                        println!("No button pressed...");
                        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(Target::Auto));
                    })
                )
                .main_axis_alignment(MainAxisAlignment::Center)
                .expand_width()
            , 1.0)
        ));
        return CloseDialogWidget {inner: inner};
    } 
}

impl Widget<AppState> for CloseDialogWidget<AppState> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        println!("HERE!!!");
        match event {
            Event::WindowConnected => {
                ctx.window().bring_to_front_and_focus();
            },
            // Event::KeyDown(key) => {
            //     print!("key = {:?}", key)
            // },
            _ => {}
        }

        self.inner.event(ctx, event, data, _env);
    }

    fn layout(&mut self,  layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, window_type: &AppState, env: &Env) -> Size {
        self.inner.layout(layout_ctx, bc, window_type, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, window_type: &AppState, env: &Env) {
        self.inner.lifecycle(ctx, event, window_type, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &AppState, env: &Env) {
        self.inner.paint(ctx,data,env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx<'_, '_>, old_data: &AppState, data: &AppState, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }
}


impl Widget<AppState> for MainWidget<AppState> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        self.main_container.event(ctx, event, data, _env);

        match event {
            Event::MouseDown(mouse_event) => {
                data.mouse_click_screen_coordinates = Some(mouse_event.window_pos);
            },
            _ => {} // handle the event as normal
        }
        // print!("event = {:?}", event)
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
        self.main_container.update(ctx, old_data, data, env);
        if data.window_type != old_data.window_type {
            self.main_container = MainWidget::build_page_ui(data.window_type, data, env);
            ctx.children_changed();
        }
    }
    
}

// Create the main (root) Widget 
fn build_root_widget() -> impl Widget<AppState> {
    MainWidget::<AppState>::new()
}

// Add all Hextiles in the given region to the board
fn add_appropriate_hextiles_to_board(board: &mut im::Vector<Hextile>, region: BoardRegionBoundaryHexCoords) {
    let x_min: i32 = region.x_min;
    let x_max: i32 = region.x_max;
    let y_min: i32 = region.y_min;
    let y_max: i32 = region.y_max;
    let z_min: i32 = region.z_min;
    let z_max: i32 = region.z_max;

    for x in x_min..(x_max + 1) {
        for y in y_min..(y_max + 1) {
            for z in z_min..(z_max + 1) {
                if x + y + z == 0 {
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

fn create_board() -> im::Vector<Hextile> {
    let mut board: im::Vector<Hextile> = im::Vector::new();

    add_appropriate_hextiles_to_board(&mut board, TopLeftTriangleBoundaryCoords);
    add_appropriate_hextiles_to_board(&mut board, TopTriangleBoundaryCoords);
    add_appropriate_hextiles_to_board(&mut board, TopRightTriangleBoundaryCoords);
    add_appropriate_hextiles_to_board(&mut board, BottomLeftTriangleBoundaryCoords);
    add_appropriate_hextiles_to_board(&mut board, BottomTriangleBoundaryCoords);
    add_appropriate_hextiles_to_board(&mut board, BottomRightTriangleBoundaryCoords);
    add_appropriate_hextiles_to_board(&mut board, CenterRegionBoundaryCoords);

    return board; 
}

// RoomID: str(private_key_of_creator_encrypt(public_ip_address_of_creator)) + str(public_key_of_creator)

// Ticket: private_key_of_joiner_encrypt(public_ip_address_of_joiner + private_key_of_joiner + public_key_of_joiner)

// Creator sends their private key to the joiner, but encrypted with the public key of the joiner

// Alternatively,

// Both players exchange public keys beforehand

// The creator sends their public ip encrypyed with the joiner's public key to the joiner

// Upon receiving this the joiner decrypts the message, extracts the creator's public ip, and then sends their own public ip encrypted with the creator's public key to the creator

// Now both players have each others ip addresses as well as the ability to securely send messages to them

fn main() {
    let mut main_window = WindowDesc::new(MainWidget::<AppState>::new())
                    .with_min_size(Size::new(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT))
                    .resizable(true)
                    .title("Chinese Checkers");
    let local_main_window_id = WindowId::next();
    main_window.id = local_main_window_id;
    let mut global_main_window_id_mutex = (*main_window_id).lock().unwrap();
    (*global_main_window_id_mutex) = Some(local_main_window_id);
    Mutex::unlock(global_main_window_id_mutex);

    let initial_state = AppState {whose_turn : None, window_type : AppPage::Start, board: im::Vector::new(), 
        in_game: false, display_victory_banner: false, mouse_location_in_canvas : Point::new(0.0, 0.0), pieces : vector![], 
        player_piece_colors: im::Vector::new(), last_hopper : None, num_players : None, regions_to_players: im::Vector::new(),
        create_remote_game_players_added: Some(vector!["Tommy", "Karina", "Joseph"]), 
        newly_won_player: None,
        players_that_have_won: im::Vector::<usize>::new(),
        room_id: Some(String::from("1515")),
        join_remote_game_entered_room_id: String::from("jHfjHsdkmcjFhdkSjfjf"),
        join_remote_game_ticket: None,
        registration_ticket: String::from("registration ticket"),
        mouse_click_screen_coordinates: None,
        number_of_players_selected: 2,
        anti_spoiling_rule: AntiSpoilingRule::FilledDest,
        advnset_ranked_winner: false,
        advnset_all_pass_equals_draw: false,
        advnset_three_players_two_triangles: false,
        advnset_two_players_three_triangles: false,
        advnset_forced_move_if_available: false,
        advnset_only_enter_own_dest: false,
        colored_circle_text: Arc::from(CIRCLE_STR), 
        num_consecutive_passes: 0,
        display_draw_banner: false,
        display_game_over_banner: false
    };

    AppLauncher::with_window(main_window)
        // .configure_env(|_env, _data| { // OnceCell

        //     // Create the user's public/private key pair
        //     // let mut rng = rand::thread_rng();
        //     // let mut builder = RsaPrivateKeyBuilder::new(BigNum::from_u32(rng.gen::<u32>()).unwrap(), BigNum::from_u32(rng.gen::<u32>()).unwrap(), BigNum::from_u32(rng.gen::<u32>()).unwrap()).unwrap();
        //     // builder = builder.set_factors(BigNum::from_u32(rng.gen::<u32>()).unwrap(), BigNum::from_u32(rng.gen::<u32>()).unwrap()).unwrap();
        //     let result = Rsa::generate(2048).unwrap();
        //     let public_key_bytes = result.public_key_to_pem().unwrap();

        //     let ip_addr : String = local_ip().unwrap().to_string();
        
        //     let keypair : PKey<Private>= openssl::pkey::PKey::try_from(result).unwrap();

        //     let encrypter = Encrypter::new(&keypair).unwrap();

        //     let s = ip_addr.to_owned();
        //     let input = &s[..];

        //     let buffer_len = encrypter.encrypt_len(&input.as_bytes()).unwrap();
        //     let mut encrypted = std::vec::Vec::<u8>::new();
        //     encrypted.extend(iter::repeat(0).take(buffer_len));

        //     let encrypted_len = encrypter.encrypt(input.as_bytes(), &mut encrypted).unwrap();

        //     println!("encyrpted_len = {}", encrypted_len);

        //     encrypted.truncate(encrypted_len);

        //     let string_list : std::vec::Vec<String> = encrypted.iter().map(|val| val.to_string()).collect();
        //     let room_id_str = string_list.join("-");

        //     let pubkey_bytes : std::vec::Vec<String> = public_key_bytes.iter().map(|val| val.to_string()).collect();
        //     let pubkey_str = pubkey_bytes.join("-");

        //     // let public_key_bytes = &public_key_bytes_tmp[..];

        //     let res = ROOM_ID.set(
        //         room_id_str + "@" + &pubkey_str
        //     );

        //     if res.is_err() {
        //         println!("ERROR: attempting to set the ROOM_ID OnceCell in configure_env produced an error...");
        //     }
        //})
        .delegate(GlobalDelegate::make())
        .launch(initial_state)
        .expect("ERROR: Failed to launch application, exiting immediately....");
}