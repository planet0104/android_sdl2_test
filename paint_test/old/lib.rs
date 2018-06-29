#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate android_log_sys;
// extern crate num;
// extern crate sdl2;
// extern crate libloading;
#[macro_use]
extern crate lazy_static;
//mod sdl2_sys;
//mod sdl2;
use libloading::Library;
use std::ffi::CString;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
// use std::os::raw::{c_int, c_char, c_void};
// use sdl2_sys::*;
// use sdl2::*;

// #[cfg(all(target_arch = "aarch64", target_os = "android"))]//target_family=unix
// lazy_static!{
//     static ref LIB:Library = libloading::Library::new("/data/user/0/jna.test.com.testjna/files/arm64-v8a/libSDL2.so").unwrap();
// }
// #[cfg(all(target_arch = "armv7", target_os = "android"))]
// lazy_static!{
//     pub static ref LIB:Library = libloading::Library::new("/data/user/0/jna.test.com.testjna/files/armeabi-v7a/libSDL2.so").unwrap();
// }
// #[cfg(all(target_arch = "i686", target_os = "android"))]
// lazy_static!{
//     pub static ref LIB:Library = libloading::Library::new("/data/user/0/jna.test.com.testjna/files/x86/libSDL2.so").unwrap();
// }
// #[cfg(all(target_arch = "x86_64", target_os = "android"))]
// lazy_static!{
//     pub static ref LIB:Library = libloading::Library::new("/data/user/0/jna.test.com.testjna/files/x86_64/libSDL2.so").unwrap();
// }
// #[cfg(all(target_arch = "x86_64", target_os="windows"))]
// lazy_static!{
//     pub static ref LIB:Library = libloading::Library::new("SDL2.dll").unwrap();
// }
// #[cfg(all(target_arch = "x86_64", target_os="linux"))]
// lazy_static!{
//     pub static ref LIB:Library = libloading::Library::new("libSDL2.so").unwrap();
// }

#[no_mangle]
pub fn test() ->f32 {
    println!("test ok.");
    1024.0
}
//linux安装:
//sudo apt-get install libsdl2-dev
//export PATH=$PATH:/home/planet/ndk-standalone-21-aarch64/bin/

/*
下载安卓NDK
./make-standalone-toolchain.sh --platform=android-21 --toolchain=aarch64-linux-android --install-dir=/home/planet/ndk-standalone-21-aarch64 --verbose
./make-standalone-toolchain.sh --platform=android-21 --toolchain=arm-linux-androideabi --install-dir=/home/planet/ndk-standalone-21-arm --verbose

sudo apt-get install libc6-i386 lib32z1 lib32stdc++6
sudo apt install build-essential

配置cargo
nano ~/.cargo/config

[target.aarch64-linux-android]
ar = "/home/planet/ndk-standalone-21-aarch64/bin/aarch64-linux-android-ar"
linker = "/home/planet/ndk-standalone-21-aarch64/bin/aarch64-linux-android-gcc"

[target.arm-linux-androideabi]
ar = "/home/planet/ndk-standalone-21-arm/bin/arm-linux-androideabi-ar"
linker = "/home/planet/ndk-standalone-21-arm/bin/arm-linux-androideabi-gcc"

配置环境变量
export PATH=$PATH:~/ndk-standalone-21-aarch64/bin
export PATH=$PATH:~/ndk-standalone-21-arm/bin

 */

//返回字符串
// CString::new("呵呵呵OK.").unwrap().into_raw() as *mut i8
//返回buffer
//

//cargo run --manifest-path ..\android_sdl2_test\Cargo.toml

#[no_mangle]
pub fn start() -> *mut u8{
    println!("程序启动!");
    //let rs = String::from("哈喽");
    // let mut window: *mut SDL_Window = unsafe{ ::std::mem::uninitialized() };
    // let mut renderer: *mut SDL_Renderer = unsafe{ ::std::mem::uninitialized() };
    // let ret = unsafe{(SDL2.SDL_CreateWindowAndRenderer)(0, 0, 0, &mut window, &mut renderer)};
    // if ret < 0 {
    //     log("Main", "SDL_CreateWindowAndRenderer Error!");
    //     return ::std::ptr::null_mut();
    // }

    //let surface = Surface::new(64, 64, PixelFormatEnum::RGB24).unwrap();
    // let masks_result = PixelFormatEnum::RGB24.into_masks();
    // if masks_result.is_err(){
    //     return ::std::ptr::null_mut();
    // }
    // let masks = masks_result.unwrap();
    // let surface = unsafe { (SDL2.SDL_CreateRGBSurface)(0, 400, 400, masks.bpp as c_int, masks.rmask, masks.gmask, masks.bmask, masks.amask) };
    // let renderer = unsafe { (SDL2.SDL_CreateSoftwareRenderer)(surface) };
    // let ret = unsafe { (SDL2.SDL_SetRenderDrawColor)(renderer, 0, 0, 0, 255) };
    // if ret != 0 {
    //     return ::std::ptr::null_mut();
    // }
    // unsafe { (SDL2.SDL_RenderClear)(renderer) };
    // let ret = unsafe { (SDL2.SDL_SetRenderDrawColor)(renderer, 255, 210, 0, 255) };
    // if ret != 0 {
    //     return ::std::ptr::null_mut();
    // }
    // let ret = unsafe { (SDL2.SDL_RenderFillRect)(renderer, &SDL_Rect::new(10, 10, 30, 30)) };
    // if ret != 0 {
    //     return ::std::ptr::null_mut();
    // }
    // unsafe { (SDL2.SDL_RenderPresent)(renderer) };

    //保存文件
    // let path = if cfg!(target_os = "android"){"/storage/emulated/0/Pictures/sdl_android_test.bmp"}else{
    //         "sdl_android_test.bmp"
    // };
    // let path_c = CString::new(path).unwrap();
    // //let path_c = CString::new().unwrap();
    // let mode_c = CString::new("wb").unwrap();
    // let file = unsafe { (SDL2.SDL_RWFromFile)(path_c.as_ptr(), mode_c.as_ptr()) };

    // if file.is_null() {
    //     return ::std::ptr::null_mut();
    // }

    // let format = PixelFormatEnum::RGB24;
    // let pitch = 400 * format.byte_size_per_pixel(); // calculated pitch
    // let size = format.byte_size_of_pixels(400 * 400);
    // let mut pixels = Vec::with_capacity(size);
    // unsafe{ pixels.set_len(size); }

    // let ret = unsafe{ (SDL2.SDL_RenderReadPixels)(renderer, &SDL_Rect::new(0, 0, 400, 400), format as u32, pixels.as_mut_ptr(), pitch as i32) };
    // if ret == 0 { }

    // let mut data = vec![0; 400*400*4];
    // let buf:&mut [u8] = &mut data;
    
    // let file = unsafe{ (SDL2.SDL_RWFromMem)(buf.as_ptr() as *mut c_void, buf.len() as i32) };
    // println!("size={}", unsafe{ (*file).size.unwrap()(file) });
    // let ret = unsafe { (SDL2.SDL_SaveBMP_RW)(surface, file, buf.len() as i32) };
    // if ret != 0 {
    //     return ::std::ptr::null_mut();
    // }
    // println!("size={}", unsafe{ (*file).size.unwrap()(file) });
    
    // //bmp.push(222);
    // //CString::new("呵呵呵OK.").unwrap().into_raw() as *mut i8

    // let var1 = 999;
    // let raw_bytes: [u8; 4] = unsafe { std::mem::transmute(var1) };
    // for byte in &raw_bytes {
    //     println!("{}", byte);
    // }

    // let mut a = vec![8, 8, 8, 9];
    // a.insert(0, raw_bytes[0]);
    // a.insert(1, raw_bytes[1]);
    // a.insert(2, raw_bytes[2]);
    // a.insert(3, raw_bytes[3]);
    
    // println!("{:?}", a);
    // a.as_mut_ptr()


    // let surface = Surface::new(64, 64, PixelFormatEnum::RGB24).unwrap();
    // let mut canvas = Canvas::from_surface(surface).unwrap();
    
    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();
    // canvas.set_draw_color(Color::RGB(255, 210, 0));
    // canvas.fill_rect(Rect::new(10, 10, 30, 30)).unwrap();
    // canvas.present();
    // log("TAG", "canvas.present()");
    
    0 as *mut u8
}

//定义Android JNI接口
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    extern crate android_log_sys;
    use android_log_sys::{__android_log_print, LogPriority};

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    fn log(tag:&str, text:&str){
        let tag = CString::new(tag).unwrap();
        let text = CString::new(text).unwrap();
        unsafe{ __android_log_print(LogPriority::DEBUG as i32, tag.as_ptr(), text.as_ptr()); }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_jna_test_com_testjna_Lib_start(env: JNIEnv, _: JClass, from: JString) -> jstring {
        let name:String = env.get_string(from).expect("无法取到名字!").into();
        let echo = "HelloWorld!";
        let output = env.new_string(echo).expect("java字符串创建失败!");
        output.into_inner()
    }
}