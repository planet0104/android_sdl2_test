extern crate paint;
extern crate image;
extern crate piston;
extern crate graphics;
extern crate piston_window;
use piston::window::WindowSettings;
use piston_window::*;

fn main(){
    let buffer:Vec<u8> = paint::process("head.png");
    let img = image::load_from_memory(&buffer).unwrap();

    let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();

    let img_texture: G2dTexture = Texture::from_image(
                &mut window.factory,
                &img.to_rgba(),
                &TextureSettings::new()
    ).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            image(&img_texture, c.transform, g);
        });
    }
}