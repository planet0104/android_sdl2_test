#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[cfg(target_os = "android")]
extern crate android_log_sys;
#[cfg(target_os = "android")]
use android_log_sys::{__android_log_print, LogPriority};
extern crate num;
extern crate libloading;
#[macro_use]
extern crate lazy_static;
mod sdl2_sys;
mod sdl2;
use libloading::Library;
use std::ffi::CString;
use std::os::raw::{c_int, c_char};
use sdl2_sys::*;
use sdl2::*;

#[cfg(all(target_arch = "aarch64", target_os = "android"))]//target_family=unix
lazy_static!{
    static ref LIB:Library = libloading::Library::new("/data/user/0/jna.test.com.testjna/files/arm64-v8a/libSDL2.so").unwrap();
}
#[cfg(all(target_arch = "armv7", target_os = "android"))]
lazy_static!{
    pub static ref LIB:Library = libloading::Library::new("/data/user/0/jna.test.com.testjna/files/armeabi-v7a/libSDL2.so").unwrap();
}
#[cfg(all(target_arch = "i686", target_os = "android"))]
lazy_static!{
    pub static ref LIB:Library = libloading::Library::new("/data/user/0/jna.test.com.testjna/files/x86/libSDL2.so").unwrap();
}
#[cfg(all(target_arch = "x86_64", target_os = "android"))]
lazy_static!{
    pub static ref LIB:Library = libloading::Library::new("/data/user/0/jna.test.com.testjna/files/x86_64/libSDL2.so").unwrap();
}
#[cfg(all(target_arch = "x86_64", target_os="windows"))]
lazy_static!{
    pub static ref LIB:Library = libloading::Library::new("SDL2.dll").unwrap();
}
#[cfg(all(target_arch = "x86_64", target_os="linux"))]
lazy_static!{
    pub static ref LIB:Library = libloading::Library::new("libSDL2.so").unwrap();
}

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

sudo apt-get install libc6-i386 lib32z1 lib32stdc++6
sudo apt install build-essential

配置cargo
nano ~/.cargo/config

[target.aarch64-linux-android]
ar = "/home/planet/ndk-standalone-21-aarch64/bin/aarch64-linux-android-ar"
linker = "/home/planet/ndk-standalone-21-aarch64/bin/aarch64-linux-android-gcc"

配置环境变量
export PATH=$PATH:~/ndk-standalone-21-aarch64/bin

 */

fn log(tag:&str, text:&str){
    if cfg!(target_os = "android"){
        #[cfg(target_os = "android")]{
            let tag = CString::new(tag).unwrap();
            let text = CString::new(text).unwrap();
            unsafe{ __android_log_print(LogPriority::DEBUG as i32, tag.as_ptr(), text.as_ptr()); }
        }
    }else{
        println!("{}: {}", tag, text);
    }
}

//返回字符串
// CString::new("呵呵呵OK.").unwrap().into_raw() as *mut i8
//返回buffer
//

//cargo run --manifest-path ..\android_sdl2_test\Cargo.toml
#[no_mangle]
pub fn start() -> *mut i8{
    log("Main", "程序启动!");
    let rs = String::from("哈喽");
    // let mut window: *mut SDL_Window = unsafe{ ::std::mem::uninitialized() };
    // let mut renderer: *mut SDL_Renderer = unsafe{ ::std::mem::uninitialized() };
    // let ret = unsafe{(SDL2.SDL_CreateWindowAndRenderer)(0, 0, 0, &mut window, &mut renderer)};
    // if ret < 0 {
    //     log("Main", "SDL_CreateWindowAndRenderer Error!");
    //     return ::std::ptr::null_mut();
    // }

    //let surface = Surface::new(64, 64, PixelFormatEnum::RGB24).unwrap();
    let masks_result = PixelFormatEnum::RGB24.into_masks();
    if masks_result.is_err(){
        return ::std::ptr::null_mut();
    }
    let masks = masks_result.unwrap();
    let surface = unsafe { (SDL2.SDL_CreateRGBSurface)(0, 400, 400, masks.bpp as c_int, masks.rmask, masks.gmask, masks.bmask, masks.amask) };
    let renderer = unsafe { (SDL2.SDL_CreateSoftwareRenderer)(surface) };
    let ret = unsafe { (SDL2.SDL_SetRenderDrawColor)(renderer, 0, 0, 0, 255) };
    if ret != 0 {
        return ::std::ptr::null_mut();
    }
    unsafe { (SDL2.SDL_RenderClear)(renderer) };
    let ret = unsafe { (SDL2.SDL_SetRenderDrawColor)(renderer, 255, 210, 0, 255) };
    if ret != 0 {
        return ::std::ptr::null_mut();
    }
    let ret = unsafe { (SDL2.SDL_RenderFillRect)(renderer, &SDL_Rect::new(10, 10, 30, 30)) };
    if ret != 0 {
        return ::std::ptr::null_mut();
    }
    unsafe { (SDL2.SDL_RenderPresent)(renderer) };

    //保存文件
    let path = if cfg!(target_os = "android"){"/storage/emulated/0/Pictures/sdl_android_test.bmp"}else{
            "sdl_android_test.bmp"
    };
    let path_c = CString::new(path).unwrap();
    //let path_c = CString::new().unwrap();
    let mode_c = CString::new("wb").unwrap();
    let file = unsafe { (SDL2.SDL_RWFromFile)(path_c.as_ptr(), mode_c.as_ptr()) };

    if file.is_null() {
        return ::std::ptr::null_mut();
    }

    let ret = unsafe { (SDL2.SDL_SaveBMP_RW)(surface, file, 0) };
    if ret != 0 {
        return ::std::ptr::null_mut();
    }
    //bmp.push(222);
    //CString::new("呵呵呵OK.").unwrap().into_raw() as *mut i8
    let mut a = vec![8, 1, 2, 3];
    println!("{:?}", a);
    a.as_mut_ptr()
}