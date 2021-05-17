use druid::widget::{ControllerHost, Click, SizedBox, Align, Padding, Button, Flex, Container, Label, IdentityWrapper};
use druid::AppLauncher;
use druid::{WindowId, MenuDesc, MenuItem, Screen, LocalizedString, ContextMenu, Affine, Point, Rect, FontDescriptor, TextLayout, Color, Handled, DelegateCtx, AppDelegate, Command, Selector, Target, Widget, Data, Lens, WindowDesc, EventCtx, Event, Env, LayoutCtx, BoxConstraints, LifeCycle, LifeCycleCtx, Size, PaintCtx, UpdateCtx, WidgetId, WidgetExt, MouseButton};
use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use druid::widget::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};
use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid_shell::{Menu, HotKey, KbKey, KeyEvent, RawMods, SysMods};
use druid::im;
use druid::im::vector;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    // Global mutable variable storing the WidgetId of the root widget. 
    static ref root_widget_id_guard : Mutex::<WidgetId> = Mutex::<WidgetId>::new(WidgetId::next());  // global variable always storing the widget id of the root widget
    static ref start_game_selector : Selector<u32> = Selector::new("START_GAME");
    static ref piece_size_bounds : Size = Size::new(20.0, 20.0);
    static ref square_edge_bounds : Size = Size::new(26.5, 26.5);
}

static CANVAS_WIDTH : f64 = 600.0;
static CANVAS_HEIGHT: f64 = 600.0;
//static ABSTRACT_BOARD_WIDTH: f64 = 25.0;  // horizontal length from size to size of the board, with the origin right in the middle
//static ABSTRACT_BOARD_WIDTH: f64 = 25.0;  // horizontal length from size to size of the board, with the origin right in the middle
//static ABSTRACT_BOARD_HEIGHT: f64 = 15.0; // vertical length from size to size of the board, with the origin right in the middle

static SQRT_3: f64 = 1.732050808;
static ABSTRACT_BOARD_WIDTH: f64 = SQRT_3 * 8.0; 
static ABSTRACT_BOARD_HEIGHT: f64 = SQRT_3 * 8.0;

static BOARD_WIDTH : f64= 400.0;
static BOARD_HEIGHT : f64 = 400.0;

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
}

static PLAYER_ONE_NUMBER : i32 = 0;
static PLAYER_TWO_NUMBER : i32 = 1;
static PLAYER_THREE_NUMBER: i32 = 2;
static PLAYER_FOUR_NUMBER: i32 = 3;
static PLAYER_FIVE_NUMBER : i32 = 4;
static PLAYER_SIX_NUMBER : i32 = 5;
static NO_PLAYER : i32 = i32::MIN;

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
enum AppStateValue {
    START,
    SINGLE_PLAYER,
    MULTI_PLAYER,
}

#[derive(Clone, Copy)]
enum StartingRegion {
    TOP,
    TOP_LEFT,
    TOP_RIGHT,
    BOTTOM_LEFT,
    BOTTOM_RIGHT,
    BOTTOM,
}

#[derive(PartialEq, Clone, Data, Lens, Copy)]
struct WindowType {
    window_type : AppStateValue
}

#[derive(PartialEq, Data, Clone)]
struct Hextile {
    y_hex: i32,
    x_hex: i32,
    z_hex: i32,
    c: Color,
    // p: Option<i32>,
    piece: Option<Arc<Piece>>,
}

// use the same pieces over and over again if the user starts a second game
#[derive(PartialEq, Data, Clone)]
struct Piece {
    player_num: i32,
    hextile: Arc<Hextile>,
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
}

// Stores which window we're in and the entire state of the game 
#[derive(PartialEq, Clone, Data, Lens)]
struct AppState {
    window_type : AppStateValue,
    board : Arc::<Vec::<Hextile>>,
    pieces: druid::im::Vector<Piece>,
    in_game : bool,
    mouse_location_in_canvas : Point
}

struct MainWidget<T: Data> {
    main_container: Container<T>,
}

struct CanvasWidget<'a> {
    piece_is_being_dragged : bool,
    piece_being_dragged : Option<&'a Hextile>
}

impl<'a> CanvasWidget<'a> {
    fn cartesian_x_to_screen_x(x: f64) -> f64 {
        return (BOARD_WIDTH / 2.0) + (x / (ABSTRACT_BOARD_WIDTH / 2.0)) * (BOARD_WIDTH / 2.0) + (CANVAS_WIDTH - BOARD_WIDTH) / 2.0;
    }
    
    fn cartesian_y_to_screen_y(y: f64) -> f64 {
        return (BOARD_HEIGHT / 2.0) + (-(y / (ABSTRACT_BOARD_HEIGHT / 2.0))) * (BOARD_HEIGHT / 2.0) + (CANVAS_HEIGHT - BOARD_HEIGHT) / 2.0;
    }

    // Returns true iff the Point on the Canvas where the user clicked is inside of a piece
    fn is_within_a_hextile(&mut self, board_wrapper: Arc::<Vec<Hextile>>, p: Point) -> bool {
        println!("calling is_within_a_hextile!");

        // On the screen each hextile is contained in a 20px x 20px rectangle, so the radius is 10px
        unsafe {
            for hextile in (*Arc::as_ptr(&board_wrapper)).iter() {
                if ((CanvasWidget::cartesian_x_to_screen_x(hextile.cartesian_x()) - p.x).powi(2) + (CanvasWidget::cartesian_y_to_screen_y(hextile.cartesian_y()) - p.y).powi(2)).sqrt() < 10.0 {
                    self.piece_being_dragged = Some(hextile);
                    println!("success!");
                    return true;
                }
            }
        }
        return false;
    }
    
}

impl<'a> Widget<AppState> for CanvasWidget<'a> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                let board_data_copy : Arc::<Vec<Hextile>> = data.board.clone(); 

                if self.is_within_a_hextile(board_data_copy, mouse_event.pos) {
                    self.piece_is_being_dragged = true; 
                } else {
                    self.piece_is_being_dragged = false;
                    self.piece_being_dragged = None;
                }
            },
            Event::MouseUp(_mouse_event) => {
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
        let size = ctx.size();
        let rect = size.to_rect();
        //ctx.fill(rect, &Color::WHITE);

        //ctx.fill(size.to_rect().to_ellipse(), &Color::rgb8(255,248,220));

        // draw light brown outer circle of board
        ctx.fill(Rect::from_center_size(rect.center(), Size::new(rect.width() * 3.0 / 4.0, rect.height() * 3.0 / 4.0)).to_ellipse(), &Color::rgb8(BOARD_CIRCLE_COLOR_r,BOARD_CIRCLE_COLOR_g,BOARD_CIRCLE_COLOR_b));

        // loop through the board, draw each hextile
        // let size_bounds = Size::new(20.0,20.0);
        // let edge_bounds = Size::new(22.0,22.0);

        let data_copy = data.clone(); 

        //ctx.paint_with_z_index(1, move |ctx| {

            unsafe {

                //let board : Vec<Hextile> = Arc::try_unwrap(data.board.clone()).unwrap_or(Vec::new());
                let board_unsafe_ptr : *const Vec<Hextile> = Arc::as_ptr(&data_copy.board);
                let board_ref = board_unsafe_ptr.as_ref();
                //if board_ref.is_none() {
                //    panic!("ERROR, the board pointer is null, exiting immediately.");
                //}
                //if board_ref.unwrap().len() == 0 {
                //    panic!("ERROR, the board has size 0, exiting immediately.");
                //}
                let board = board_ref.unwrap();
                
                let screen_x = |x: f64| -> f64 {
                    //return (x / ABSTRACT_BOARD_WIDTH + 1.0/2.0) * BOARD_WIDTH;
                    return (BOARD_WIDTH / 2.0) + (x / (ABSTRACT_BOARD_WIDTH / 2.0)) * (BOARD_WIDTH / 2.0) + (CANVAS_WIDTH - BOARD_WIDTH) / 2.0;
                };
                
                let screen_y = |y: f64| -> f64 {
                    //return (-1.0) * (y / ABSTRACT_BOARD_HEIGHT - 1.0/2.0) * BOARD_HEIGHT;
                    return (BOARD_HEIGHT / 2.0) + (-(y / (ABSTRACT_BOARD_HEIGHT / 2.0))) * (BOARD_HEIGHT / 2.0) + (CANVAS_HEIGHT - BOARD_HEIGHT) / 2.0;
                };    

                //println!("Size of board Vec = {}", board.len());

                let mut x_hex_saved : i32 = 0;
                let mut y_hex_saved : i32 = 0;
                let mut z_hex_saved : i32 = 0;
                let mut will_draw_piece_later : bool = false;
                let mut saved_piece_color : Option<Color> = None;

                for hextile in board.into_iter() {
                    //println!("x_hex = {x_hex}, y_hex = {y_hex}, z = {z_hex}", x_hex = hextile.x_hex, y_hex = hextile.y_hex, z_hex = hextile.z_hex);
                    //let bounding_rect = Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())),size_bounds);
                    //println!("x_screen = {x_screen}, y_screen = {y_screen}", x_screen = screen_x(hextile.cartesian_x()), y_screen = screen_y(hextile.cartesian_y()));

                    // draw the square beneath the piece
                    ctx.fill(Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())), *square_edge_bounds).to_ellipse(), &Color::rgb8(96,54,15));

                    //ctx.fill(Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())),size_bounds).to_ellipse(), &hextile.c)
                    // println!("Painting coordinate: (x, y) = ({cartesian_x}, {cartesian_y})  |  x_hex = {x_hex}, y_hex = {y_hex}, z_hex = {z_hex}", x_hex = hextile.x_hex, y_hex = hextile.y_hex, z_hex = hextile.z_hex, cartesian_x = hextile.cartesian_x(), cartesian_y = hextile.cartesian_y());
                    if self.piece_being_dragged.is_some() 
                            && hextile.x_hex == self.piece_being_dragged.unwrap().x_hex 
                                && hextile.y_hex == self.piece_being_dragged.unwrap().y_hex 
                                    && hextile.z_hex == self.piece_being_dragged.unwrap().z_hex {
                            
                            // skip over drawing the piece for now, we will draw it later
                            will_draw_piece_later = true;
                            saved_piece_color = Some(hextile.c.clone());
                            println!("will draw some hextile later!");

                    } else {
                        // draw the piece in its resting state spot
                        ctx.fill(Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())), *piece_size_bounds).to_ellipse(), &hextile.c);
                    }

                }

                if will_draw_piece_later {
                    println!("x_hex_saved = {x_hex_saved}, y_hex_saved = {y_hex_saved}, z_hex_saved = {z_hex_saved}", x_hex_saved = x_hex_saved, y_hex_saved = y_hex_saved, z_hex_saved = z_hex_saved);
                    println!("DRAWING THE PIECE!!!");
                    ctx.fill(Rect::from_center_size(Point::new(data.mouse_location_in_canvas.x, data.mouse_location_in_canvas.y), *piece_size_bounds).to_ellipse(), &(saved_piece_color.unwrap()));
                }
            }

       // });
    // add_appropriate_hextiles_to_board(
    //     &mut board,
    //     x_min,
    //     x_max,
    //     y_min,
    //     y_max,
    //     z_min,
    //     z_max,
    //     &yellow_color_array.clone(),
    // );


        // Create an arbitrary bezier path
        // let mut path = BezPath::new();
        // path.move_to(Point::ORIGIN);
        // path.quad_to((40.0, 50.0), (size.width, size.height));
        // // Create a color
        // let stroke_color = Color::rgb8(0, 128, 0);
        // // Stroke the path with thickness 5.0
        // ctx.stroke(path, &stroke_color, 5.0);

        // Rectangles: the path for practical people
        // let rect = Rect::from_origin_size((10.0, 10.0), (100.0, 100.0));
        // // Note the Color:rgba8 which includes an alpha channel (7F in this case)
        // let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
        // ctx.fill(rect, &fill_color);

        // Text is easy; in real use TextLayout should either be stored in the
        // widget and reused, or a label child widget to manage it all.
        // This is one way of doing it, you can also use a builder-style way.
        // let mut layout = TextLayout::<String>::from_text("SINGLE-PLAYER-MODE");
        // layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.0));
        // layout.set_text_color(fill_color);
        // layout.rebuild_if_needed(ctx.text(), env);

        // Let's rotate our text slightly. First we save our current (default) context:
        // ctx.with_save(|ctx| {
        //     // Now we can rotate the context (or set a clip path, for instance):
        //     // This makes it so that anything drawn after this (in the closure) is
        //     // transformed.
        //     // The transformation is in radians, but be aware it transforms the canvas,
        //     // not just the part you are drawing. So we draw at (80.0, 40.0) on the rotated
        //     // canvas, this is NOT the same position as (80.0, 40.0) on the original canvas.
        //     ctx.transform(Affine::rotate(std::f64::consts::FRAC_PI_4));
        //     layout.draw(ctx, (80.0, 40.0));
        // });
        // When we exit with_save, the original context's rotation is restored

        // This is the builder-style way of drawing text.
        // let text = ctx.text();
        // let layout = text
        //     .new_text_layout("SINGLE-PLAYER-MODE")
        //     .font(FontFamily::SERIF, 24.0)
        //     .text_color(Color::rgb8(128, 0, 0))
        //     .build()
        //     .unwrap();
        // ctx.draw_text(&layout, (100.0, 25.0));

        // Let's burn some CPU to make a (partially transparent) image buffer
        // let image_data = make_image_data(256, 256);
        // let image = ctx
        //     .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
        //     .unwrap();
        // // The image is automatically scaled to fit the rect you pass to draw_image
        // ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
    }
}

impl MainWidget<AppState> {

    fn make_start_menu() -> Container<AppState> {
        let padding_dp = (0.0, 10.0); // 10dp of vertical padding, 0dp of horizontal padding 
        let column_layout = Flex::column()
        .with_child(Padding::new(padding_dp, Button::new("Single-Player").on_click(|ctx, data : &mut AppState, env| {
            data.window_type = AppStateValue::SINGLE_PLAYER;
            println!("Single-player button pressed....");
        })))
        .with_child(Padding::new(padding_dp, Button::new("Multi-Player").on_click(|ctx, data : &mut AppState, env| {
            data.window_type = AppStateValue::MULTI_PLAYER;
            println!("Multi-player button pressed....");
        })))
        .with_child(Padding::new(padding_dp, Button::new("Settings")))
        .with_child(Padding::new(padding_dp, Button::new("Feedback")))
        .with_child(Padding::new(padding_dp, Button::new("Quit").on_click(|ctx, data: &mut AppState, env| {
            println!("closing the application....");
            ctx.window().close();
        })));

        return Container::new(Align::centered(column_layout));
    }

    fn new() -> IdentityWrapper<Self> {
        // let padding_dp = (0.0, 10.0); // 4dp of vertical padding, 0dp of horizontal padding 

        // let column_layout = Flex::column()
        //     .with_child(Padding::new(padding_dp, Button::new("Single-Player").on_click(|ctx, data : &mut AppState, env| {
        //         data.window_type = AppStateValue::SINGLE_PLAYER;
        //         println!("Single-player button pressed....");
        //     })))
        //     .with_child(Padding::new(padding_dp, Button::new("Multi-Player").on_click(|ctx, data : &mut AppState, env| {
        //         data.window_type = AppStateValue::MULTI_PLAYER;
        //         println!("Multi-player button pressed....");
        //     })))
        //     .with_child(Padding::new(padding_dp, Button::new("Settings")))
        //     .with_child(Padding::new(padding_dp, Button::new("Feedback")))
        //     .with_child(Padding::new(padding_dp, Button::new("Quit")));
                     
        let main_widget = MainWidget::<AppState> {
            main_container: MainWidget::make_start_menu()
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

unsafe fn get_hextile_at_coordinates(x_hex: i32, y_hex: i32, z_hex: i32, board_arc: Arc<Vec<Hextile>>) -> Option<Arc::<Hextile>> {
    let board_ptr : *const Vec<Hextile> = Arc::as_ptr(&board_arc);
    let board : Vec<Hextile> = *board_ptr;
    for hextile in board.iter() {
        if hextile.x_hex == x_hex && hextile.y_hex == y_hex && hextile.z_hex == z_hex {
            // return Some(Arc::<Hextile>::new(hextile));
        }
    }
    return None;
}

fn initialize_pieces_for_board(board: Arc<Vec<Hextile>>, mut pieces: im::Vector<Piece>, num_players: u32) {
    if num_players == 6 {

        let regions_to_players : [(StartingRegion, i32); 6] = [
            // turns proceed clockwise
            (StartingRegion::TOP, PLAYER_ONE_NUMBER),
            (StartingRegion::TOP_RIGHT, PLAYER_TWO_NUMBER),
            (StartingRegion::BOTTOM_RIGHT, PLAYER_THREE_NUMBER),
            (StartingRegion::BOTTOM, PLAYER_FOUR_NUMBER),
            (StartingRegion::BOTTOM_LEFT, PLAYER_FIVE_NUMBER),
            (StartingRegion::TOP_LEFT, PLAYER_FIVE_NUMBER),
        ];

        unsafe {
            for i in 0..6 {
                let pair : &(StartingRegion, i32) = &regions_to_players[i];
                let starting_region = (*pair).0;
                let num = (*pair).1;

                if num != NO_PLAYER {
                    let player_number = num;

                    let boundary_coords = get_boundary_coords_struct_for_region(starting_region);
                    
                    for x in boundary_coords.x_min..boundary_coords.x_max+1 {
                        for y in boundary_coords.y_min..boundary_coords.y_max+1 {
                            for z in boundary_coords.z_min..boundary_coords.z_max+1 {

                                let hextile_at_coordinates = get_hextile_at_coordinates(x,y,z,board);

                                if hextile_at_coordinates.is_none() {
                                    panic!("Internal Error: initialize_pieces_for_board(): Unable to find a square on the board with the given hex coordinates. Exiting immediately....");
                                }
                                let piece : Piece = Piece {
                                    player_num: player_number,
                                    hextile: hextile_at_coordinates.unwrap(),
                                };
                                pieces.push_back(piece);

                                hextile_at_coordinates.unwrap().piece = Some(Arc::new(piece));
                            }
                        }
                    }
                    
                }
            }
        }
    }
}   


impl Widget<AppState> for MainWidget<AppState> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::Command(command) => {
                if command.is::<u32>(*start_game_selector) {
                    let num_players : u32 = *command.get_unchecked::<u32>(*start_game_selector);
                    println!("Received a start game command for {} players", num_players);
                    if num_players == 6 {
                       data.board = Arc::<Vec<Hextile>>::new(create_board());
                       data.pieces.clear();

                       //initialize_pieces_for_board(data.board, data.pieces , num_players);

                       data.in_game = true;
                       ctx.request_paint();
                    }
                }   
            }
            _ => {} // handle the event as normal
        }
        self.main_container.event(ctx,event, data,_env)
    }

    fn layout(&mut self,  layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _window_type: &AppState, _env: &Env) -> Size {
        self.main_container.layout(layout_ctx,bc,_window_type,_env)
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _window_type: &AppState, _env: &Env) {
        self.main_container.lifecycle(_ctx,_event,_window_type,_env);
    }

    fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &AppState, env: &Env) {
        self.main_container.paint(ctx,data,env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx<'_, '_>, old_data: &AppState, data: &AppState, env: &Env) {
        if data.window_type == AppStateValue::START && old_data.window_type == AppStateValue::SINGLE_PLAYER {
            self.main_container = MainWidget::make_start_menu();
            ctx.children_changed();
        }
        else if data.window_type == AppStateValue::START {
            self.main_container.update(ctx,old_data,data,env)
        } else if data.window_type == AppStateValue::SINGLE_PLAYER && old_data.window_type == AppStateValue::START {    
            self.main_container =  Container::new(
                                    Flex::column()
                                        .with_child(
                                            Flex::row()
                                                .with_flex_child(Padding::new(20.0, Container::new(Align::centered(Button::new("New Game").on_click(|ctx, data: &mut AppState, _env| {
                                                    let context_menu_desc = MenuDesc::<AppState>::new(LocalizedString::new("Number of Players"));
                                                    let item = MenuItem::<AppState>::new(LocalizedString::new("How many players?"), Selector::new("My Selector"));
                                                    let widget_id_holder : MutexGuard<WidgetId> = root_widget_id_guard.lock().unwrap();            
                                                    let item2 = MenuItem::<AppState>::new(LocalizedString::new("2"), Command::new(*start_game_selector, 2, Target::Widget(*widget_id_holder)));
                                                    let item3 = MenuItem::<AppState>::new(LocalizedString::new("3"), Command::new(*start_game_selector, 3, Target::Widget(*widget_id_holder)));
                                                    let item4 = MenuItem::<AppState>::new(LocalizedString::new("4"), Command::new(*start_game_selector, 4, Target::Widget(*widget_id_holder)));
                                                    let item5 = MenuItem::<AppState>::new(LocalizedString::new("5"), Command::new(*start_game_selector, 5, Target::Widget(*widget_id_holder)));
                                                    let item6 = MenuItem::<AppState>::new(LocalizedString::new("6"), Command::new(*start_game_selector, 6, Target::Widget(*widget_id_holder)));
                                                    let new_game_context_menu = ContextMenu::new(context_menu_desc.append(item.disabled()).append(item2).append(item3).append(item4).append(item5).append(item6), data.mouse_location_in_canvas.clone());
                                                    ctx.show_context_menu(new_game_context_menu);
                                                    println!("new game buttton pressed!!");
                                                })))),1.0)
                                                .with_flex_child(Container::new(Align::centered(Button::new("Quit").on_click(|_ctx, data: &mut AppState, _env| {
                                                    data.window_type = AppStateValue::START;
                                                    data.board = Arc::new(Vec::new());
                                                    data.in_game = false;
                                                    println!("Quit button pressed in single-player mode....");                                    
                                                }))),1.0)
                                        )
                                        .with_child(SizedBox::new(CanvasWidget {piece_is_being_dragged: false, piece_being_dragged: None})));
            ctx.children_changed();
        } else if data.window_type == AppStateValue::MULTI_PLAYER {
            self.main_container =  Container::new(Align::centered(Flex::column().with_child(Label::new("MULTI-PLAYER-MODE-ENTERED"))));
            ctx.children_changed();
        }
    }
}

// Create the main (root) Widget 
fn build_root_widget() -> impl Widget<AppState> {
    MainWidget::<AppState>::new()
}

fn create_board() -> Vec<Hextile> {
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
        &yellow_color_array.clone(),
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
        &red_color_array.clone(),
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
        &blue_color_array.clone(),
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
        &black_color_array.clone(),
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
        &green_color_array.clone(),
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
        &white_color_array.clone(),
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
        &center_color_array.clone()
    );
    println!("Being called from create_board, size of board Vec = {}", board.len());
    return board; 
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
        hex_color: &Color,
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
                            c: (*hex_color).clone(),
                            // p: None,
                            piece: None,
                        };
                        (*board).push(tile)
                    }
                }
            }
        }
    }

fn main() {
    let main_window = WindowDesc::new(build_root_widget);

    //let initial_state = AppState {window_type : AppStateValue::START, board: Arc::<Vec<Hextile>>::new(create_board()), in_game: false};
    let initial_state = AppState {window_type : AppStateValue::START, board: Arc::<Vec<Hextile>>::new(Vec::new()), in_game: false, mouse_location_in_canvas : Point::new(0.0, 0.0), pieces : vector![]};

    //let command_handler = ApplicationCommandHandler::new();

    AppLauncher::with_window(main_window)
        //.delegate(command_handler)
        .launch(initial_state)
        .expect("ERROR: Failed to launch application, exiting immediately....");
}