use std::io;
#[cfg(windows)] use winres::WindowsResource;

fn main() -> io::Result<()> {
    #[cfg(windows)] {
        println!("build.rs running...");
        WindowsResource::new()
        //    .set_icon_with_id("chinese-checkers.ico", "icon_main")        
            .set_icon_with_id("chinese-checkers.ico", "icon_main")
            .compile().unwrap();
    }
    Ok(())
}