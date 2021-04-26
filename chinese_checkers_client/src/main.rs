use druid::widget::{SizedBox, Align, Padding, Button, Flex, Container, Label, IdentityWrapper};
use druid::AppLauncher;
use druid::{Affine, Point, Rect, FontDescriptor, TextLayout, Color, Handled, DelegateCtx, AppDelegate, Command, Selector, Target, Widget, Data, Lens, WindowDesc, EventCtx, Event, Env, LayoutCtx, BoxConstraints, LifeCycle, LifeCycleCtx, Size, PaintCtx, UpdateCtx, WidgetId, WidgetExt, MouseButton};
use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use druid::widget::prelude::*;

use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};

#[derive(Clone, Data, Lens)]
struct GameState {

}

#[derive(PartialEq, Clone, Data, Copy)]
enum WindowTypeValue {
    START,
    SINGLE_PLAYER,
    MULTI_PLAYER,
}

#[derive(PartialEq, Clone, Data, Lens, Copy)]
struct WindowType {
    window_type : WindowTypeValue
}

struct MainWidget<T: Data> {
    main_container: Container<T>,
}

struct CanvasWidget {}

impl Widget<WindowType> for CanvasWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut WindowType, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &WindowType, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &WindowType, _data: &WindowType, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &WindowType,
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
    fn paint(&mut self, ctx: &mut PaintCtx, data: &WindowType, env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

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

impl MainWidget<WindowType> {
    fn new<'a>() -> Self {
        let padding_dp = (0.0, 10.0); // 4dp of vertical padding, 0dp of horizontal padding 

        let column_layout = Flex::column()
            .with_child(Padding::new(padding_dp, Button::new("Single-Player").on_click(|ctx, data : &mut WindowType, env| {
                data.window_type = WindowTypeValue::SINGLE_PLAYER;
                println!("Single-player button pressed....");
            })))
            .with_child(Padding::new(padding_dp, Button::new("Multi-Player").on_click(|ctx, data : &mut WindowType, env| {
                data.window_type = WindowTypeValue::MULTI_PLAYER;
                println!("Multi-player button pressed....");
            })))
            .with_child(Padding::new(padding_dp, Button::new("Settings")))
            .with_child(Padding::new(padding_dp, Button::new("Feedback")))
            .with_child(Padding::new(padding_dp, Button::new("Quit")));
             
        let initial_layout = Align::centered(column_layout);
        
        let main_widget = MainWidget::<WindowType> {
            main_container: Container::new(initial_layout),
        };
            
        main_widget
    } 
}

impl Widget<WindowType> for MainWidget<WindowType> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut WindowType, _env: &Env) {
        self.main_container.event(ctx,event,_data,_env)
    }

    fn layout(&mut self,  layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _window_type: &WindowType, _env: &Env) -> Size {
        self.main_container.layout(layout_ctx,bc,_window_type,_env)
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _window_type: &WindowType, _env: &Env) {
        self.main_container.lifecycle(_ctx,_event,_window_type,_env);
    }

    fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &WindowType, env: &Env) {
        self.main_container.paint(ctx,data,env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx<'_, '_>, old_data: &WindowType, data: &WindowType, env: &Env) {
        if data.window_type == WindowTypeValue::START {
            self.main_container.update(ctx,old_data,data,env)
        } else if data.window_type == WindowTypeValue::SINGLE_PLAYER {    
            self.main_container =  Container::new(Flex::column().with_child(Label::new("SINGLE-PLAYER-MODE-ENTERED")).with_child(SizedBox::new(CanvasWidget {})));
            ctx.children_changed();
        } else if data.window_type == WindowTypeValue::MULTI_PLAYER {
            self.main_container =  Container::new(Align::centered(Flex::column().with_child(Label::new("MULTI-PLAYER-MODE-ENTERED"))));
            ctx.children_changed();
        } else {
            panic!("ERROR: Internal error, unrecognized window type, exiting immediately....");
        }
    }
}

// Create the main (root) Widget 
fn build_root_widget() -> impl Widget<WindowType> {
    MainWidget::<WindowType>::new()
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget);

    let initial_state = WindowType {window_type : WindowTypeValue::START};

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("ERROR: Failed to launch application, exiting immediately....");
}