use ggez::{graphics, Context, ContextBuilder, GameResult, nalgebra as na};
use ggez::event::{self, EventHandler};
use ggez::graphics::{DrawMode, MeshBuilder, DrawParam};
use ggez::conf::{WindowSetup, NumSamples};

fn main() {
    // Set up the Window
    let game_window_setup : WindowSetup = WindowSetup{
        title: "Chinese Checkers".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_setup(game_window_setup)
        .build()
        .expect("Error, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => {
            println!("Error occurred: {}", e); 
        }
    }
}

struct MyGame {
    // Your state here...
    // Everything that we need to render the game board.
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let (window_width, window_height) : (f32,f32) = graphics::drawable_size(ctx); 
        let (circle_x, circle_y) : (f32, f32) = (window_width / 2.0, window_height / 2.0);
        let circle_radius : f32 = window_width.min(window_height) / 6.0;

        let board_circle_mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), na::Point2::new(circle_x, circle_y), circle_radius, 0.00001, graphics::BLACK,)
            .build(ctx)?;

        graphics::draw(ctx, &board_circle_mesh, DrawParam::default())?;

        // Draw code here...
        graphics::present(ctx)?;
        
        return Ok(())
    }
}