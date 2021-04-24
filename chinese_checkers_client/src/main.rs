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

use druid::widget::{Align, Padding, Button, Flex};
use druid::AppLauncher;
use druid::{Widget, Data, Lens, WindowDesc};

#[derive(Clone, Data, Lens)]
struct GameState {

}

#[derive(PartialEq, Clone, Data)]
enum WindowTypeValue {
    START_WINDOW,
    SINGLE_PLAYER,
    MULTI_PLAYER,
}

#[derive(PartialEq, Clone, Data, Lens)]
struct WindowType {
    window_type : WindowTypeValue
}

// Create the main (root) Widget
fn build_root_widget() -> impl Widget<WindowType> {
    let padding_dp = (0.0, 10.0); // 4dp of vertical padding, 0dp of horizontal padding 
    let column_layout = Flex::column()
        .with_child(Padding::new(padding_dp, Button::new("Single-Player")))
        .with_child(Padding::new(padding_dp, Button::new("Multi-player")))
        .with_child(Padding::new(padding_dp, Button::new("Settings")))
        .with_child(Padding::new(padding_dp, Button::new("Feedback")))
        .with_child(Padding::new(padding_dp, Button::new("Quit")));
    
    Align::centered(column_layout) 
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget);

    let initial_state = WindowType {window_type : WindowTypeValue::START_WINDOW};

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("ERROR: Failed to launch application, exiting immediately....");
}