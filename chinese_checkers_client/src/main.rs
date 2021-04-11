use ggez::{graphics, Context, ContextBuilder, GameResult, nalgebra as na};
use ggez::event::{self, EventHandler};
use ggez::graphics::{DrawMode, MeshBuilder, DrawParam, Color};
use ggez::conf::{WindowSetup, NumSamples};

fn main() {
    println!("Hello, world!");

     // Set up the Window
     let game_window_setup : WindowSetup = WindowSetup{
        title: "Chinese Checkers".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };
}
