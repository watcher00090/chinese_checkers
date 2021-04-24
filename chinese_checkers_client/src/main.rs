use druid::widget::{Align, Padding, Button, Flex, Container, Label, IdentityWrapper};
use druid::AppLauncher;
use druid::{Handled, DelegateCtx, AppDelegate, Command, Selector, Target, Widget, Data, Lens, WindowDesc, EventCtx, Event, Env, LayoutCtx, BoxConstraints, LifeCycle, LifeCycleCtx, Size, PaintCtx, UpdateCtx, WidgetId, WidgetExt, MouseButton};
use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};


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
            self.main_container =  Container::new(Align::centered(Flex::column().with_child(Label::new("SINGLE-PLAYER-MODE-ENTERED"))));
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