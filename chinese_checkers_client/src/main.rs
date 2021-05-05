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

#[macro_use]
extern crate lazy_static;

lazy_static! {
    // Global mutable variable storing the WidgetId of the root widget. 
    static ref root_widget_id_guard : Mutex::<WidgetId> = Mutex::<WidgetId>::new(WidgetId::next());  // global variable always storing the widget id of the root widget
    static ref start_game_selector : Selector<u32> = Selector::new("START_GAME");
}

static CANVAS_WIDTH : f64 = 600.0;
static CANVAS_HEIGHT: f64 = 600.0;
//static ABSTRACT_BOARD_WIDTH: f64 = 25.0;  // horizontal length from size to size of the board, with the origin right in the middle
//static ABSTRACT_BOARD_WIDTH: f64 = 25.0;  // horizontal length from size to size of the board, with the origin right in the middle
//static ABSTRACT_BOARD_HEIGHT: f64 = 15.0; // vertical length from size to size of the board, with the origin right in the middle

static SQRT_3: f64 = 1.732050808;
static ABSTRACT_BOARD_WIDTH: f64 = SQRT_3 * 8.0; 
static ABSTRACT_BOARD_HEIGHT: f64 = SQRT_3 * 8.0;

static START_NEW_GAME_2_PLAYERS_ID : u32 = 1000;
static START_NEW_GAME_3_PLAYERS_ID : u32 = 1001;
static START_NEW_GAME_4_PLAYERS_ID : u32 = 1002;
static START_NEW_GAME_5_PLAYERS_ID : u32 = 1003;
static START_NEW_GAME_6_PLAYERS_ID : u32 = 1004;

#[derive(PartialEq, Clone, Data, Copy, Debug)]
enum AppStateValue {
    START,
    SINGLE_PLAYER,
    MULTI_PLAYER,
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
    p: Option<i32>,
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
}

// Stores which window we're in and the entire state of the game 
#[derive(PartialEq, Clone, Data, Lens)]
struct AppState {
    window_type : AppStateValue,
    board : Arc::<Vec::<Hextile>>,
    in_game : bool
}

struct MainWidget<T: Data> {
    main_container: Container<T>,
}

struct CanvasWidget {}

impl Widget<AppState> for CanvasWidget {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, _data: &mut AppState, env: &Env) {
        
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
        ctx.fill(Rect::from_center_size(rect.center(), Size::new(rect.width() * 3.0 / 4.0, rect.height() * 3.0 / 4.0)).to_ellipse(), &Color::rgb8(255,248,220));

        let BOARD_WIDTH : f64= 400.0;
        let BOARD_HEIGHT : f64 = 400.0;

        let data_copy = data.clone(); 

        ctx.paint_with_z_index(1, move |ctx| {

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

                // loop through the board, draw each hextile
                let size_bounds = Size::new(20.0,20.0);

                //println!("Size of board Vec = {}", board.len());

                for hextile in board.into_iter() {
                    //println!("x_hex = {x_hex}, y_hex = {y_hex}, z = {z_hex}", x_hex = hextile.x_hex, y_hex = hextile.y_hex, z_hex = hextile.z_hex);
                    //let bounding_rect = Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())),size_bounds);
                    //println!("x_screen = {x_screen}, y_screen = {y_screen}", x_screen = screen_x(hextile.cartesian_x()), y_screen = screen_y(hextile.cartesian_y()));

                    //ctx.fill(Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())),size_bounds).to_ellipse(), &hextile.c)
                    // println!("Painting coordinate: (x, y) = ({cartesian_x}, {cartesian_y})  |  x_hex = {x_hex}, y_hex = {y_hex}, z_hex = {z_hex}", x_hex = hextile.x_hex, y_hex = hextile.y_hex, z_hex = hextile.z_hex, cartesian_x = hextile.cartesian_x(), cartesian_y = hextile.cartesian_y());
                    ctx.fill(Rect::from_center_size(Point::new(screen_x(hextile.cartesian_x()), screen_y(hextile.cartesian_y())),size_bounds).to_ellipse(), &hextile.c)
                }
            }

        });
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
        .with_child(Padding::new(padding_dp, Button::new("Quit")));

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
    fn create_start_game_popup_window_layout<'a>() -> Label<AppState> {
        return Label::<AppState>::new("Enter a number, between 1 and 6");
    }
}

impl Widget<AppState> for MainWidget<AppState> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppState, _env: &Env) {
        match event {
            Event::Command(command) => {
                if command.is::<u32>(*start_game_selector) {
                    let num_players : u32 = *command.get_unchecked::<u32>(*start_game_selector);
                    println!("Received a start game command for {} players", num_players);
                }   
            }
            _ => {} // handle the event as normal
        }
        self.main_container.event(ctx,event,_data,_env)
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
        if data.window_type == AppStateValue::SINGLE_PLAYER && data.in_game == true && old_data.in_game == false {
            // game has been started....
        }
        if data.window_type == AppStateValue::START && old_data.window_type == AppStateValue::SINGLE_PLAYER {
            self.main_container = MainWidget::make_start_menu();
            ctx.children_changed();
        }
        else if data.window_type == AppStateValue::START {
            self.main_container.update(ctx,old_data,data,env)
        } else if data.window_type == AppStateValue::SINGLE_PLAYER && old_data.window_type == AppStateValue::START {    
            //self.main_container =  Container::new(Flex::column().with_child(Label::new("SINGLE-PLAYER-MODE-ENTERED")).with_child(SizedBox::new(CanvasWidget {})));
            self.main_container =  Container::new(
                                    Flex::column()
                                        .with_child(
                                            Flex::row()
                                                .with_flex_child(Padding::new(20.0, Container::new(Align::centered(Button::new("New Game").on_click(|ctx, _data: &mut AppState, _env| {
                                                    //let popup_window_descrip = WindowDesc::new(MainWidget::<AppState>::create_start_game_popup_window_layout);
                                                    //ctx.new_window(popup_window_descrip);
                                                    let window_handle = ctx.window();
                                                    let context_menu_desc = MenuDesc::<AppState>::new(LocalizedString::new("Number of Players"));
                                                    // let location = Rect::from_origin_size(Point::new(0.0,0.0),window_handle.get_size()).center(); // center of the winodw
                                                    let location = Point::new(0.0,0.0);

                                                    let item = MenuItem::<AppState>::new(LocalizedString::new("How many players?"), Selector::new("My Selector"));
                                                    //let item2 = MenuItem::<AppState>::new(LocalizedString::new("2"), Selector::new("START_NEW_GAME_WITH_2_PLAYERS"));
                                                    //let item3 = MenuItem::<AppState>::new(LocalizedString::new("3"), Selector::new("START_NEW_GAME_WITH_3_PLAYERS"));
                                                    //let item4 = MenuItem::<AppState>::new(LocalizedString::new("4"), Selector::new("START_NEW_GAME_WITH_4_PLAYERS"));
                                                    //let item5 = MenuItem::<AppState>::new(LocalizedString::new("5"), Selector::new("START_NEW_GAME_WITH_5_PLAYERS"));
                                                    //let item6 = MenuItem::<AppState>::new(LocalizedString::new("6"), Selector::new("START_NEW_GAME_WITH_6_PLAYERS"));

                                                    // dereference this to get the widget id of the root widget
                                                    let widget_id_holder : MutexGuard<WidgetId> = root_widget_id_guard.lock().unwrap();            

                                                    let item2 = MenuItem::<AppState>::new(LocalizedString::new("2"), Command::new(*start_game_selector, 2, Target::Widget(*widget_id_holder)));
                                                    let item3 = MenuItem::<AppState>::new(LocalizedString::new("3"), Command::new(*start_game_selector, 3, Target::Widget(*widget_id_holder)));
                                                    let item4 = MenuItem::<AppState>::new(LocalizedString::new("4"), Command::new(*start_game_selector, 4, Target::Widget(*widget_id_holder)));
                                                    let item5 = MenuItem::<AppState>::new(LocalizedString::new("5"), Command::new(*start_game_selector, 5, Target::Widget(*widget_id_holder)));
                                                    let item6 = MenuItem::<AppState>::new(LocalizedString::new("6"), Command::new(*start_game_selector, 6, Target::Widget(*widget_id_holder)));

                                                    // let widget_id_holder : MutexGuard<WidgetId> = root_widget_id_guard.lock().unwrap();            

                                                    let new_game_context_menu = ContextMenu::new(context_menu_desc.append(item.disabled()).append(item2).append(item3).append(item4).append(item5).append(item6), Point::new(0.0,0.0));
                                                    //let mut context_menu = Menu::new_for_popup();
                                                    // let mut number_of_players_list = Menu::new_for_popup();
                                                    // context_menu.add_item(1, "How many players:", None, false, false);
                                                    // context_menu.add_item(START_NEW_GAME_2_PLAYERS_ID, "2", None, true, true);
                                                    // context_menu.add_item(START_NEW_GAME_3_PLAYERS_ID, "3", None, true, true);
                                                    // context_menu.add_item(START_NEW_GAME_4_PLAYERS_ID, "4", None, true, true);
                                                    // context_menu.add_item(START_NEW_GAME_5_PLAYERS_ID, "5", None, true, true);
                                                    // context_menu.add_item(START_NEW_GAME_6_PLAYERS_ID, "6", None, true, true);

                                                    //context_menu.add_dropdown(number_of_players_list, "How many players:", true);
                                                    //let id : u32 = 10;
                                                    //window_handle.show_context_menu(context_menu, location);
                                                    ctx.show_context_menu(new_game_context_menu);
                                                    println!("new game buttton pressed!!");
                                                })))),1.0)
                                                .with_flex_child(Container::new(Align::centered(Button::new("Quit").on_click(|_ctx, data: &mut AppState, _env| {
                                                    data.window_type = AppStateValue::START;
                                                    println!("Quit button pressed in single-player mode....");                                    
                                                }))),1.0)
                                        )
                                        .with_child(SizedBox::new(CanvasWidget {})));
            ctx.children_changed();
        } else if data.window_type == AppStateValue::MULTI_PLAYER {
            self.main_container =  Container::new(Align::centered(Flex::column().with_child(Label::new("MULTI-PLAYER-MODE-ENTERED"))));
            ctx.children_changed();
        } 
        //else if data.window_type == AppStateValue::SINGLE_PLAYER {
        //    println!("in single-player mode, starting game...");
        //} 
        else {
            println!("data.window_type == {:?}", data.window_type);
            panic!("ERROR: Internal error, unrecognized window type, exiting immediately....");
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
    let center_color_array: Color = Color::rgba(0.5, 0.5, 0.5, 0.5);

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
                            p: None,
                        };
                        (*board).push(tile)
                    }
                }
            }
        }
    }

fn main() {
    let main_window = WindowDesc::new(build_root_widget);

    let initial_state = AppState {window_type : AppStateValue::START, board: Arc::<Vec<Hextile>>::new(create_board()), in_game: false};

    //let command_handler = ApplicationCommandHandler::new();

    AppLauncher::with_window(main_window)
        //.delegate(command_handler)
        .launch(initial_state)
        .expect("ERROR: Failed to launch application, exiting immediately....");
}