/* use druid::widget::{Align, Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Env, Lens, Widget, WindowDesc, WidgetExt, LocalizedString}; 

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
    .title("Chinese Checkers")
    .window_size((400.0, 400.0));

   // create the initial app state
   let initial_state = HelloState {
       name: "World".into(),
   };

   // start the application
   AppLauncher::with_window(main_window)
       .launch(initial_state)
       .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
   // a label that will determine its text based on the current app data.
   let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
   // a textbox that modifies `name`.
   let textbox = TextBox::new()
       .with_placeholder("Who are we greeting?")
       .fix_width(TEXT_BOX_WIDTH)
       .lens(HelloState::name);

    let button = Button::new("Increment").on_click(|_ctx, data: &mut HelloState, _env| {
        data.name += " + buttonpress";
    });

   // arrange the two widgets vertically, with some padding
   let layout = Flex::column()
       .with_child(label)
       .with_spacer(VERTICAL_WIDGET_SPACING)
       .with_child(textbox)
       .with_child(button);
       
   // center the two widgets in the available space
   Align::centered(layout)
} */

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
    single_player_button_widget_id: WidgetId,
}

struct ChineseCheckersAppDelegate {}

static change_to_single_player_mode_selector : Selector = Selector::new("CHANGE_TO_SINGLE_PLAYER_MODE");

// impl AppDelegate<WindowType> for ChineseCheckersAppDelegate {
//     fn command(&mut self, ctx: &mut DelegateCtx<'_>, target: Target, cmd: &Command, data: &mut WindowType, env: &Env) -> Handled {
//         if cmd.get(change_to_single_player_mode_selector).is_some() {
//             println!("THE COMMAND IS BEING REGISTERED!!!!");
//             let root_widget = *cmd.get(change_to_single_player_mode_selector).unwrap();
//             return Handled::Yes
//         }
//         return Handled::Yes
//     }    
// }

impl MainWidget<WindowType> {
    fn new<'a>() -> Self {
        let mut hasher = DefaultHasher::new();

        let single_player_button = Button::<WindowType>::new("Single-player");
        let mut raw = rand::thread_rng().gen();
        let single_player_button_widget_id  = WidgetId::reserved(raw);
        let wrapped_single_player_button = WidgetExt::with_id(single_player_button, single_player_button_widget_id);
        single_player_button_widget_id.hash(&mut hasher);
        println!("Single_player_button_widget_id = {}", hasher.finish());
        wrapped_single_player_button.id().unwrap().hash(&mut hasher);
        println!("Wrapped_single_player_button_widget_id = {}", hasher.finish());

        let multi_player_button = Button::<WindowType>::new("Multi-player");
        raw = rand::thread_rng().gen();
        let multi_player_button_widget_id : WidgetId = WidgetId::reserved(raw);
        let wrapped_multi_player_button = WidgetExt::with_id(multi_player_button, multi_player_button_widget_id);
        multi_player_button_widget_id.hash(&mut hasher);
        println!("Multi_player_button_widget_id = {}", hasher.finish());
        wrapped_multi_player_button.id().unwrap().hash(&mut hasher);
        println!("Wrapped_multi_player_button_widget_id = {}", hasher.finish());

        let settings_button = Button::<WindowType>::new("Settings");
        raw = rand::thread_rng().gen();
        let settings_button_widget_id : WidgetId = WidgetId::reserved(raw);
        let wrapped_settings_button = WidgetExt::with_id(settings_button, settings_button_widget_id);
        settings_button_widget_id.hash(&mut hasher);
        println!("Settings_button_widget_id = {}", hasher.finish());
        wrapped_settings_button.id().unwrap().hash(&mut hasher);
        println!("Wrapped_settings_button_widget_id = {}", hasher.finish());

        let feedback_button = Button::<WindowType>::new("Feedback");
        raw = rand::thread_rng().gen();
        let feedback_button_widget_id : WidgetId = WidgetId::reserved(raw);
        let wrapped_feedback_button = WidgetExt::with_id(feedback_button, feedback_button_widget_id);
        feedback_button_widget_id.hash(&mut hasher);
        println!("Feedback_button_widget_id = {}", hasher.finish());
        wrapped_feedback_button.id().unwrap().hash(&mut hasher);
        println!("Wrapped_feedback_button_widget_id = {}", hasher.finish());

        let quit_button = Button::<WindowType>::new("Quit");
        raw = rand::thread_rng().gen();
        let quit_button_widget_id : WidgetId = WidgetId::reserved(raw);
        let wrapped_quit_button = WidgetExt::with_id(quit_button, quit_button_widget_id);
        quit_button_widget_id.hash(&mut hasher);
        println!("Quit_button_widget_id = {}", hasher.finish());
        wrapped_quit_button.id().unwrap().hash(&mut hasher);
        println!("Wrapped_quit_button_widget_id = {}", hasher.finish());

        let padding_dp = (0.0, 10.0); // 4dp of vertical padding, 0dp of horizontal padding 

        raw = rand::thread_rng().gen();
        let padded_single_player_button = Padding::new(padding_dp, wrapped_single_player_button);
        let padded_single_player_button_widget_id : WidgetId = WidgetId::reserved(raw);
        let padded_single_player_button_with_widget_id = WidgetExt::with_id(padded_single_player_button, padded_single_player_button_widget_id);
        padded_single_player_button_widget_id.hash(&mut hasher);
        println!("Padded_single+player_button_id = {}", hasher.finish());

        // wrap the main container in an ID
        raw = rand::thread_rng().gen();
        let main_container_widget_id : WidgetId = WidgetId::reserved(raw);
        let copy_of_main_container_widget_id : WidgetId = main_container_widget_id;

        let column_layout = Flex::column()
            .with_child(Padding::new(padding_dp, Button::new("Single-Player").on_click(|ctx, data : &mut WindowType, env| {
                data.window_type = WindowTypeValue::SINGLE_PLAYER;
                println!("Single-player button pressed....");
            })))
            .with_child(Padding::new(padding_dp, Button::new("Settings")))
            .with_child(Padding::new(padding_dp, Button::new("Feedback")))
            .with_child(Padding::new(padding_dp, Button::new("Quit")));
             
       /*  let column_layout = Flex::column()
        .with_child(Padding::new(padding_dp, wrapped_single_player_button))
        .with_child(Padding::new(padding_dp, wrapped_multi_player_button))
        .with_child(Padding::new(padding_dp, wrapped_settings_button))
        .with_child(Padding::new(padding_dp, wrapped_feedback_button))
        .with_child(Padding::new(padding_dp, wrapped_quit_button)); */


        let initial_layout = Align::centered(column_layout);
        
        let main_widget = MainWidget::<WindowType> {
            main_container: Container::new(initial_layout),
            single_player_button_widget_id: single_player_button_widget_id,
        };

        if main_widget.id().is_some() {
            main_widget.id().unwrap().hash(&mut hasher);
            println!("Main_widget_id = {}", hasher.finish());
        }
            
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
        if data.window_type == WindowTypeValue::SINGLE_PLAYER {    
            self.main_container =  Container::new(Flex::column().with_child(Label::new("SINGLE-PLAYER-MODE-ENTERED")));
            ctx.children_changed();
        } else if data.window_type == WindowTypeValue::MULTI_PLAYER || data.window_type == WindowTypeValue::START {
            // do nothing for the time being
            self.main_container.update(ctx,old_data,data,env)
        } else {
            panic!("ERROR: Internal error, unrecognized window type, exiting immediately....");
        }
        // self.main_container.update(ctx,old_data,data,env)
    }
}

// Create the main (root) Widget 
fn build_root_widget() -> impl Widget<WindowType> {
/*     let padding_dp = (0.0, 10.0); // 4dp of vertical padding, 0dp of horizontal padding 
    let column_layout = Flex::column()
        .with_child(Padding::new(padding_dp, Button::new("Single-Player")))
        .with_child(Padding::new(padding_dp, Button::new("Multi-player")))
        .with_child(Padding::new(padding_dp, Button::new("Settings")))
        .with_child(Padding::new(padding_dp, Button::new("Feedback")))
        .with_child(Padding::new(padding_dp, Button::new("Quit")));
    
    Align::centered(column_layout) */
    MainWidget::<WindowType>::new()
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget);

    let initial_state = WindowType {window_type : WindowTypeValue::START};

    let chinese_checkers_app_delegator = ChineseCheckersAppDelegate {};

    AppLauncher::with_window(main_window)
        // .delegate(chinese_checkers_app_delegator)
        .launch(initial_state)
        .expect("ERROR: Failed to launch application, exiting immediately....");
}