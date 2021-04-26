use druid::widget::{SizedBox, Align, Padding, Button, Flex, Container, Label, IdentityWrapper};
use druid::AppLauncher;
use druid::{Affine, Point, Rect, FontDescriptor, TextLayout, Color, Handled, DelegateCtx, AppDelegate, Command, Selector, Target, Widget, Data, Lens, WindowDesc, EventCtx, Event, Env, LayoutCtx, BoxConstraints, LifeCycle, LifeCycleCtx, Size, PaintCtx, UpdateCtx, WidgetId, WidgetExt, MouseButton};
use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use druid::widget::prelude::*;
use std::sync::Arc;
use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};


#[derive(PartialEq, Clone, Data, Copy)]
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

// Stores which window we're in and the entire state of the game 
#[derive(PartialEq, Clone, Data, Lens)]
struct AppState {
    window_type : AppStateValue,
    board : Arc::<Vec::<Hextile>>
}

struct MainWidget<T: Data> {
    main_container: Container<T>,
}

struct CanvasWidget {}

impl Widget<AppState> for CanvasWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

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
            println!("Min width = {}", bc.min().width);
            println!("Min height = {}", bc.min().height);
            println!("Max width = {}", bc.max().width);
            println!("Max height = {}", bc.max().height);

            let size = Size::new(600.0, 600.0);
            bc.constrain(size)
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
        ctx.fill(rect, &Color::WHITE);

        ctx.fill(Rect::from_center_size(rect.center(), Size::new(rect.width() * 3.0 / 4.0, rect.height() * 3.0 / 4.0)).to_ellipse(), &Color::rgb8(255,248,220));
        
        let board = Arc::try_unwrap(data.board.clone()).unwrap_or_default();

        // loop through the board, draw each hextile
       // for hextile in Arc::try_unwrap(data.board).unwrap_or_default().into_iter() {

        //}

        // We can paint with a Z index, this indicates that this code will be run
        // after the rest of the painting. Painting with z-index is done in order,
        // so first everything with z-index 1 is painted and then with z-index 2 etc.
        // As you can see this(red) curve is drawn on top of the green curve
        // ctx.paint_with_z_index(1, move |ctx| {
        //     let mut path = BezPath::new();
        //     path.move_to((0.0, size.height));
        //     path.quad_to((40.0, 50.0), (size.width, 0.0));
        //     // Create a color
        //     let stroke_color = Color::rgb8(128, 0, 0);
        //     // Stroke the path with thickness 1.0
        //     ctx.stroke(path, &stroke_color, 5.0);
        // });

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
    fn new<'a>() -> Self {
        let padding_dp = (0.0, 10.0); // 4dp of vertical padding, 0dp of horizontal padding 

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
             
        let initial_layout = Align::centered(column_layout);
        
        let main_widget = MainWidget::<AppState> {
            main_container: Container::new(initial_layout),
        };
            
        main_widget
    } 
}

impl Widget<AppState> for MainWidget<AppState> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppState, _env: &Env) {
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
        if data.window_type == AppStateValue::START {
            self.main_container.update(ctx,old_data,data,env)
        } else if data.window_type == AppStateValue::SINGLE_PLAYER {    
            self.main_container =  Container::new(Flex::column().with_child(Label::new("SINGLE-PLAYER-MODE-ENTERED")).with_child(SizedBox::new(CanvasWidget {})));
            ctx.children_changed();
        } else if data.window_type == AppStateValue::MULTI_PLAYER {
            self.main_container =  Container::new(Align::centered(Flex::column().with_child(Label::new("MULTI-PLAYER-MODE-ENTERED"))));
            ctx.children_changed();
        } else {
            panic!("ERROR: Internal error, unrecognized window type, exiting immediately....");
        }
    }
}

// Create the main (root) Widget 
fn build_root_widget() -> impl Widget<AppState> {
    MainWidget::<AppState>::new()
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

    let initial_state = AppState {window_type : AppStateValue::START, board: Arc::<Vec<Hextile>>::new(Vec::<Hextile>::new()) };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("ERROR: Failed to launch application, exiting immediately....");
}