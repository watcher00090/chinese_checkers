// use druid::widget::{Align, Padding, Stepper, Button}

// #[derive(Clone, Data, Lens)]
// struct GameState {

// }

// // Create the main (root) Widget
// fn build_root_widget() -> impl Widget<GameState> {
//     let let stepper = Stepper::new().with_range();
//     let padding_dp = (4.0, 0.0); // 4dp of vertical padding, 0dp of horizontal padding 
//     let column_layout : Widget = Flex::column();
//         .with_child(Padding::new(padding_dp, Button::new("Single-Player"))
//         .with_child(Button::new("Multi-player"))
//         .with_child(Button::new("Settings"))
//         .with_child(Button::new("Feedback"))
//         .with_child(Button::new("Quit"));
    
//     Align::centered(column_layout) 
// }

// fn main() {
//     let main_window = WindowDesc::new(build_root_widget);

//     let initial_state = GameState {};

//     AppLauncher::with_window(main_window)
//         .launch(initial_state)
//         .expect("ERROR: Failed to launch application, exiting immediately....");
// }