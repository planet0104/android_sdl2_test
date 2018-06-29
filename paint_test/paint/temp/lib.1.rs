#[cfg(target_os="android")]
extern crate android_log_sys;
extern crate lodepng;
//extern crate ui_sys;
//extern crate iui;
extern crate rand;
// #[macro_use]
// extern crate bitflags;

use rand::prelude::*;
use std::time::{Duration, Instant};
use lodepng::{Image, ColorType, Bitmap, RGBA};
// mod iui_area;
// use iui::prelude::*;
// use iui::controls::{HorizontalBox, LayoutStrategy};
// use iui::draw::{StrokeParams, DrawContext, LineCap, LineJoin, Path, FillMode, Brush, SolidBrush};
// use iui_area::{AreaHandler, Area, AreaDrawParams};

// fn set_pixel_rgba(ui: &UI, context: &DrawContext, x:f64, y:f64, r:f64, g:f64, b:f64, a:f64){
//     let path = Path::new(ui, FillMode::Winding);
//     path.add_rectangle(ui, x, y, 1.0, 1.0);
//     path.end(ui);
//     context.fill(ui, &path, &Brush::Solid(SolidBrush{r, g, b, a}));
// }

// fn draw_line(ui: &UI, context: &DrawContext, from_x:f64, from_y:f64, to_x:f64, to_y:f64, brush:&Brush, stroke_params:&StrokeParams){
//     let path = Path::new(ui, FillMode::Winding);
//     path.new_figure(ui, from_x, from_y);
//     path.line_to(ui, to_x, to_y);
//     path.end(ui);
//     context.stroke(ui, &path, brush, stroke_params);
// }


// extern crate piston;
// extern crate graphics;
// extern crate glutin_window;
// extern crate opengl_graphics;
extern crate piston_window;

use piston::window::WindowSettings;
use piston_window::PistonWindow;


pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}

#[no_mangle]
pub fn test(path: &str){
    println!("paint测试");
    let bitmap = lodepng::decode32_file(path).unwrap();
    let (width, height) = (bitmap.width, bitmap.height);
    println!("width={},height={}", width, height);

    use graphics::*;

    let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
    while let Some(e) = window.next() {
        println!("{:?}", e);
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
        });
    }
    
    // // Change this to OpenGL::V2_1 if not working.
    // let opengl = OpenGL::V3_2;

    // // Create an Glutin window.
    // let mut window: Window = WindowSettings::new(
    //         "spinning-square",
    //         [200, 200]
    //     )
    //     .opengl(opengl)
    //     .exit_on_esc(true)
    //     .build()
    //     .unwrap();

    // // Create a new game and run it.
    // let mut app = App {
    //     gl: GlGraphics::new(opengl),
    //     rotation: 0.0
    // };

    // let mut events = Events::new(EventSettings::new());
    // while let Some(e) = events.next(&mut window) {
    //     if let Some(r) = e.render_args() {
    //         app.render(&r);
    //     }

    //     if let Some(u) = e.update_args() {
    //         app.update(&u);
    //     }
    // }
    

    // let mut events_loop = winit::EventsLoop::new();
    // let window = winit::Window::new(&events_loop).unwrap();

    // events_loop.run_forever(|event| {
    //     match event {
    //         winit::Event::WindowEvent {
    //           event: winit::WindowEvent::CloseRequested,
    //           ..
    //         } => winit::ControlFlow::Break,
    //         _ => winit::ControlFlow::Continue,
    //     }
    // });

    // let ui = UI::init().expect("UI库初始化失败");
    // let mut win = Window::new(&ui, "测试", width as i32, height as i32, WindowType::NoMenubar);

    // struct DrawHandler{
    //     ui: UI,
    //     bitmap: Bitmap<RGBA>
    // }
    // impl DrawHandler{
    //     fn new(ui: UI, bitmap: Bitmap<RGBA>) -> DrawHandler{
    //         DrawHandler{ui, bitmap}
    //     }
    // }
    // impl AreaHandler for DrawHandler{
    //     fn draw(&mut self, _area: &Area, area_draw_params: &AreaDrawParams) {
    //         let now = Instant::now();
    //         for x in 0..self.bitmap.width{
    //             for y in 0..self.bitmap.height{
    //                 let pixel = self.bitmap.buffer[(y*self.bitmap.width)+x];
    //                 set_pixel_rgba(&self.ui, &area_draw_params.context, x as f64, y as f64, pixel.r as f64/255.0, pixel.g as f64/255.0, pixel.b as f64/255.0, pixel.a as f64/255.0);
    //             }
    //         }
    //         println!("{}ms", duration_to_milis(&now.elapsed()));

    //         let stroke_params = StrokeParams{
    //             cap: LineCap::Flat,
    //             join: LineJoin::Miter,
    //             thickness: 1.0,
    //             miter_limit: 0.0,
    //             dashes: vec![],
    //             dash_phase: 0.0,
    //         };
    //         let brush = Brush::Solid(SolidBrush{r:1.0, g:0.0, b:0.0, a:1.0});
    //         let mut rng = rand::thread_rng();
    //         for _ in 0..1000{
    //             let (x1, x2) = (rng.gen_range(0, self.bitmap.width) as f64, rng.gen_range(0, self.bitmap.width) as f64);
    //             let (y1, y2) = (rng.gen_range(0, self.bitmap.height) as f64, rng.gen_range(0, self.bitmap.height) as f64);
    //             draw_line(&self.ui, &area_draw_params.context, x1, y1, x2, y2, &brush, &stroke_params);
    //         }
    //     }
    // }
    // let area = Area::new(&ui, Box::new(DrawHandler::new(ui.clone(), bitmap)));

    // let mut hbox = HorizontalBox::new(&ui);
    // hbox.append(&ui, area, LayoutStrategy::Stretchy);
    // win.set_child(&ui, hbox);
    // win.set_margined(&ui, false);
    // win.show(&ui);
    // ui.main();
}

//定义Android JNI接口
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use android_log_sys::{__android_log_print, LogPriority};

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};
    use std::ffi::CString;

    fn log(tag:&str, text:&str){
        // let tag = CString::new(tag).unwrap();
        // let text = CString::new(text).unwrap();
        // unsafe{ __android_log_print(LogPriority::DEBUG as i32, tag.as_ptr(), text.as_ptr()); }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_jna_test_com_testjna_Lib_start(env: JNIEnv, _: JClass, from: JString) -> jstring {
        log("libpaint", "start.");
        let name:String = env.get_string(from).expect("无法取到名字!").into();
        let echo = format!("{}HelloWorld!", name);
        let output = env.new_string(echo).expect("java字符串创建失败!");
        output.into_inner()
    }
}