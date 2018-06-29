use std::ffi::CString;
use num::FromPrimitive;
use std::mem::transmute;
use ::LIB;
use sdl2_sys::*;
use libloading::Symbol;

pub struct LibSDL2<'a>{
    pub val: i32,
    pub SDL_LoadBMP_RW: Symbol<'a, fn_SDL_LoadBMP_RW>,
    pub SDL_RWFromFile: Symbol<'a, fn_SDL_RWFromFile>,
    pub SDL_CreateTextureFromSurface: Symbol<'a, fn_SDL_CreateTextureFromSurface>,
    pub SDL_FreeSurface: Symbol<'a, fn_SDL_FreeSurface>,
    pub SDL_GetError: Symbol<'a, fn_SDL_GetError>,
    pub SDL_CreateWindowAndRenderer: Symbol<'a, fn_SDL_CreateWindowAndRenderer>,
    pub SDL_GetWindowSize: Symbol<'a, fn_SDL_GetWindowSize>,
    pub SDL_RenderCopy: Symbol<'a, fn_SDL_RenderCopy>,
    pub SDL_PollEvent: Symbol<'a, fn_SDL_PollEvent>,
    pub SDL_SetRenderDrawColor: Symbol<'a, fn_SDL_SetRenderDrawColor>,
    pub SDL_RenderClear: Symbol<'a, fn_SDL_RenderClear>,
    pub SDL_RenderPresent: Symbol<'a, fn_SDL_RenderPresent>,
    pub SDL_Delay: Symbol<'a, fn_SDL_Delay>,
    pub SDL_CreateRGBSurface: Symbol<'a, fn_SDL_CreateRGBSurface>,
    pub SDL_MasksToPixelFormatEnum: Symbol<'a, fn_SDL_MasksToPixelFormatEnum>,
    pub SDL_PixelFormatEnumToMasks: Symbol<'a, fn_SDL_PixelFormatEnumToMasks>,
    pub SDL_CreateSoftwareRenderer: Symbol<'a, fn_SDL_CreateSoftwareRenderer>,
    pub SDL_RenderFillRect: Symbol<'a, fn_SDL_RenderFillRect>,
    pub SDL_SaveBMP_RW: Symbol<'a, fn_SDL_SaveBMP_RW>,
    pub SDL_RWFromMem: Symbol<'a, fn_SDL_RWFromMem>,
    pub SDL_RenderReadPixels: Symbol<'a, fn_SDL_RenderReadPixels>,
}

impl <'a> LibSDL2<'a>{
    fn test(&self){
        println!("{}", self.val);
    }
}

pub type fn_SDL_LoadBMP_RW = unsafe extern fn(src: *mut SDL_RWops, freesrc: ::std::os::raw::c_int)-> *mut SDL_Surface;
pub type fn_SDL_RWFromFile = unsafe extern fn(file: *const ::std::os::raw::c_char, mode: *const ::std::os::raw::c_char)-> *mut SDL_RWops;
pub type fn_SDL_CreateTextureFromSurface = unsafe extern fn(renderer: *mut SDL_Renderer, surface: *mut SDL_Surface)-> *mut SDL_Texture;
pub type fn_SDL_FreeSurface = unsafe extern fn(surface: *mut SDL_Surface);
pub type fn_SDL_GetError = unsafe extern fn() -> *const ::std::os::raw::c_char;
pub type fn_SDL_CreateWindowAndRenderer = unsafe extern fn(width: ::std::os::raw::c_int,
                                       height: ::std::os::raw::c_int,
                                       window_flags: Uint32,
                                       window: *mut *mut SDL_Window,
                                       renderer: *mut *mut SDL_Renderer)
     -> ::std::os::raw::c_int;

pub type fn_SDL_GetWindowSize = unsafe extern fn(window: *mut SDL_Window,
                             w: *mut ::std::os::raw::c_int,
                             h: *mut ::std::os::raw::c_int);
pub type fn_SDL_RenderCopy = unsafe extern fn(renderer: *mut SDL_Renderer,
                          texture: *mut SDL_Texture, srcrect: *const SDL_Rect,
                          dstrect: *const SDL_Rect) -> ::std::os::raw::c_int;
pub type fn_SDL_PollEvent = unsafe extern fn(event: *mut SDL_Event) -> ::std::os::raw::c_int;
pub type fn_SDL_SetRenderDrawColor = unsafe extern fn(renderer: *mut SDL_Renderer, r: Uint8,
                                  g: Uint8, b: Uint8, a: Uint8)
     -> ::std::os::raw::c_int;
pub type fn_SDL_RenderClear = unsafe extern fn(renderer: *mut SDL_Renderer)
     -> ::std::os::raw::c_int;
pub type fn_SDL_RenderPresent = unsafe extern fn(renderer: *mut SDL_Renderer);
pub type fn_SDL_Delay = unsafe extern fn(ms: Uint32);
pub type fn_SDL_CreateRGBSurface = unsafe extern fn(flags: Uint32, width: ::std::os::raw::c_int,
                                height: ::std::os::raw::c_int,
                                depth: ::std::os::raw::c_int, Rmask: Uint32,
                                Gmask: Uint32, Bmask: Uint32, Amask: Uint32)
     -> *mut SDL_Surface;
pub type fn_SDL_MasksToPixelFormatEnum = unsafe extern fn(bpp: ::std::os::raw::c_int,
                                      Rmask: Uint32, Gmask: Uint32,
                                      Bmask: Uint32, Amask: Uint32) -> Uint32;
pub type fn_SDL_PixelFormatEnumToMasks = unsafe extern fn(format: Uint32,
                                      bpp: *mut ::std::os::raw::c_int,
                                      Rmask: *mut Uint32, Gmask: *mut Uint32,
                                      Bmask: *mut Uint32, Amask: *mut Uint32)
     -> SDL_bool;
pub type fn_SDL_CreateSoftwareRenderer = unsafe extern fn(surface: *mut SDL_Surface)
     -> *mut SDL_Renderer;
pub type fn_SDL_RenderFillRect = unsafe extern fn(renderer: *mut SDL_Renderer,
                              rect: *const SDL_Rect) -> ::std::os::raw::c_int;
pub type fn_SDL_SaveBMP_RW = unsafe extern fn(surface: *mut SDL_Surface, dst: *mut SDL_RWops,
                          freedst: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
pub type fn_SDL_RWFromMem = unsafe extern fn(mem: *mut ::std::os::raw::c_void,
                         size: ::std::os::raw::c_int) -> *mut SDL_RWops;
pub type fn_SDL_RenderReadPixels = unsafe extern fn(renderer: *mut SDL_Renderer,
                                rect: *const SDL_Rect, format: Uint32,
                                pixels: *mut ::std::os::raw::c_void,
                                pitch: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;

lazy_static!{
    pub static ref SDL2:LibSDL2<'static> = LibSDL2{
        val: 3,
        SDL_LoadBMP_RW: unsafe{ LIB.get(b"SDL_LoadBMP_RW").unwrap() },
        SDL_RWFromFile: unsafe{ LIB.get(b"SDL_RWFromFile").unwrap() },
        SDL_CreateTextureFromSurface: unsafe{ LIB.get(b"SDL_CreateTextureFromSurface").unwrap() },
        SDL_FreeSurface: unsafe{ LIB.get(b"SDL_FreeSurface").unwrap() },
        SDL_GetError: unsafe{ LIB.get(b"SDL_GetError").unwrap() },
        SDL_CreateWindowAndRenderer: unsafe{ LIB.get(b"SDL_CreateWindowAndRenderer").unwrap() },
        SDL_GetWindowSize: unsafe{ LIB.get(b"SDL_GetWindowSize").unwrap() },
        SDL_RenderCopy: unsafe{ LIB.get(b"SDL_RenderCopy").unwrap() },
        SDL_PollEvent: unsafe{ LIB.get(b"SDL_PollEvent").unwrap() },
        SDL_SetRenderDrawColor: unsafe{ LIB.get(b"SDL_SetRenderDrawColor").unwrap() },
        SDL_RenderClear: unsafe{ LIB.get(b"SDL_RenderClear").unwrap() },
        SDL_RenderPresent: unsafe{ LIB.get(b"SDL_RenderPresent").unwrap() },
        SDL_Delay: unsafe{ LIB.get(b"SDL_Delay").unwrap() },
        SDL_CreateRGBSurface: unsafe{ LIB.get(b"SDL_CreateRGBSurface").unwrap() },
        SDL_MasksToPixelFormatEnum: unsafe{ LIB.get(b"SDL_MasksToPixelFormatEnum").unwrap() },
        SDL_PixelFormatEnumToMasks: unsafe{ LIB.get(b"SDL_PixelFormatEnumToMasks").unwrap() },
        SDL_CreateSoftwareRenderer: unsafe{ LIB.get(b"SDL_CreateSoftwareRenderer").unwrap() },
        SDL_RenderFillRect: unsafe{ LIB.get(b"SDL_RenderFillRect").unwrap() },
        SDL_SaveBMP_RW: unsafe{ LIB.get(b"SDL_SaveBMP_RW").unwrap() },
        SDL_RWFromMem: unsafe{ LIB.get(b"SDL_RWFromMem").unwrap() },
        SDL_RenderReadPixels: unsafe{ LIB.get(b"SDL_RenderReadPixels").unwrap() },
    };
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PixelFormatEnum {
    Unknown = SDL_PIXELFORMAT_UNKNOWN as i32,
    Index1LSB = SDL_PIXELFORMAT_INDEX1LSB as i32,
    Index1MSB = SDL_PIXELFORMAT_INDEX1MSB as i32,
    Index4LSB = SDL_PIXELFORMAT_INDEX4LSB as i32,
    Index4MSB = SDL_PIXELFORMAT_INDEX4MSB as i32,
    Index8 = SDL_PIXELFORMAT_INDEX8 as i32,
    RGB332 = SDL_PIXELFORMAT_RGB332 as i32,
    RGB444 = SDL_PIXELFORMAT_RGB444 as i32,
    RGB555 = SDL_PIXELFORMAT_RGB555 as i32,
    BGR555 = SDL_PIXELFORMAT_BGR555 as i32,
    ARGB4444 = SDL_PIXELFORMAT_ARGB4444 as i32,
    RGBA4444 = SDL_PIXELFORMAT_RGBA4444 as i32,
    ABGR4444 = SDL_PIXELFORMAT_ABGR4444 as i32,
    BGRA4444 = SDL_PIXELFORMAT_BGRA4444 as i32,
    ARGB1555 = SDL_PIXELFORMAT_ARGB1555 as i32,
    RGBA5551 = SDL_PIXELFORMAT_RGBA5551 as i32,
    ABGR1555 = SDL_PIXELFORMAT_ABGR1555 as i32,
    BGRA5551 = SDL_PIXELFORMAT_BGRA5551 as i32,
    RGB565 = SDL_PIXELFORMAT_RGB565 as i32,
    BGR565 = SDL_PIXELFORMAT_BGR565 as i32,
    RGB24 = SDL_PIXELFORMAT_RGB24 as i32,
    BGR24 = SDL_PIXELFORMAT_BGR24 as i32,
    RGB888 = SDL_PIXELFORMAT_RGB888 as i32,
    RGBX8888 = SDL_PIXELFORMAT_RGBX8888 as i32,
    BGR888 = SDL_PIXELFORMAT_BGR888 as i32,
    BGRX8888 = SDL_PIXELFORMAT_BGRX8888 as i32,
    ARGB8888 = SDL_PIXELFORMAT_ARGB8888 as i32,
    RGBA8888 = SDL_PIXELFORMAT_RGBA8888 as i32,
    ABGR8888 = SDL_PIXELFORMAT_ABGR8888 as i32,
    BGRA8888 = SDL_PIXELFORMAT_BGRA8888 as i32,
    ARGB2101010 = SDL_PIXELFORMAT_ARGB2101010 as i32,
    YV12 = SDL_PIXELFORMAT_YV12 as i32,
    IYUV = SDL_PIXELFORMAT_IYUV as i32,
    YUY2 = SDL_PIXELFORMAT_YUY2 as i32,
    UYVY = SDL_PIXELFORMAT_UYVY as i32,
    YVYU = SDL_PIXELFORMAT_YVYU as i32
}

impl PixelFormatEnum {
    pub fn from_masks(masks: PixelMasks) -> PixelFormatEnum {
        unsafe {
            let format = (SDL2.SDL_MasksToPixelFormatEnum)(masks.bpp as i32, masks.rmask, masks.gmask, masks.bmask, masks.amask);
            PixelFormatEnum::from_u64(format as u64).unwrap()
        }
    }

    pub fn into_masks(self) -> Result<PixelMasks, String> {
        let format: u32 = self as u32;
        let mut bpp = 0;
        let mut rmask = 0;
        let mut gmask = 0;
        let mut bmask = 0;
        let mut amask = 0;
        let result = unsafe {
            (SDL2.SDL_PixelFormatEnumToMasks)(format, &mut bpp, &mut rmask, &mut gmask, &mut bmask, &mut amask)
        };
        if result == SDL_bool::SDL_FALSE {
            // SDL_FALSE
            Err(unsafe{ get_sdl_error() })
        } else {
            Ok(PixelMasks {
                bpp: bpp as u8,
                rmask: rmask,
                gmask: gmask,
                bmask: bmask,
                amask: amask
            })
        }
    }

    /// Calculates the total byte size of an image buffer, given its pitch
    /// and height.
    pub fn byte_size_from_pitch_and_height(&self, pitch: usize, height: usize) -> usize {
        match *self {
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV => {
                // YUV is 4:2:0.
                // `pitch` is the width of the Y component, and
                // `height` is the height of the Y component.
                // U and V have half the width and height of Y.
                pitch * height + 2 * (pitch / 2 * height / 2)
            },
            _ => pitch * height
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    pub fn byte_size_of_pixels(&self, num_of_pixels: usize) -> usize {
        match *self {
            PixelFormatEnum::RGB332
                => num_of_pixels,
            PixelFormatEnum::RGB444 | PixelFormatEnum::RGB555 |
            PixelFormatEnum::BGR555 | PixelFormatEnum::ARGB4444 |
            PixelFormatEnum::RGBA4444 | PixelFormatEnum::ABGR4444 |
            PixelFormatEnum::BGRA4444 | PixelFormatEnum::ARGB1555 |
            PixelFormatEnum::RGBA5551 | PixelFormatEnum::ABGR1555 |
            PixelFormatEnum::BGRA5551 | PixelFormatEnum::RGB565 |
            PixelFormatEnum::BGR565
                => num_of_pixels * 2,
            PixelFormatEnum::RGB24 | PixelFormatEnum::BGR24
                => num_of_pixels * 3,
            PixelFormatEnum::RGB888 | PixelFormatEnum::RGBX8888 |
            PixelFormatEnum::BGR888 | PixelFormatEnum::BGRX8888 |
            PixelFormatEnum::ARGB8888 | PixelFormatEnum::RGBA8888 |
            PixelFormatEnum::ABGR8888 | PixelFormatEnum::BGRA8888 |
            PixelFormatEnum::ARGB2101010
                => num_of_pixels * 4,
            // YUV formats
            // FIXME: rounding error here?
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV
                => num_of_pixels / 2 * 3,
            PixelFormatEnum::YUY2 | PixelFormatEnum::UYVY |
            PixelFormatEnum::YVYU
                => num_of_pixels * 2,
            // Unsupported formats
            PixelFormatEnum::Index8
                => num_of_pixels,
            PixelFormatEnum::Unknown | PixelFormatEnum::Index1LSB |
            PixelFormatEnum::Index1MSB | PixelFormatEnum::Index4LSB |
            PixelFormatEnum::Index4MSB
                => panic!("not supported format: {:?}", *self),
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    pub fn byte_size_per_pixel(&self) -> usize {
        match *self {
            PixelFormatEnum::RGB332
                => 1,
            PixelFormatEnum::RGB444 | PixelFormatEnum::RGB555 |
            PixelFormatEnum::BGR555 | PixelFormatEnum::ARGB4444 |
            PixelFormatEnum::RGBA4444 | PixelFormatEnum::ABGR4444 |
            PixelFormatEnum::BGRA4444 | PixelFormatEnum::ARGB1555 |
            PixelFormatEnum::RGBA5551 | PixelFormatEnum::ABGR1555 |
            PixelFormatEnum::BGRA5551 | PixelFormatEnum::RGB565 |
            PixelFormatEnum::BGR565
                => 2,
            PixelFormatEnum::RGB24 | PixelFormatEnum::BGR24
                => 3,
            PixelFormatEnum::RGB888 | PixelFormatEnum::RGBX8888 |
            PixelFormatEnum::BGR888 | PixelFormatEnum::BGRX8888 |
            PixelFormatEnum::ARGB8888 | PixelFormatEnum::RGBA8888 |
            PixelFormatEnum::ABGR8888 | PixelFormatEnum::BGRA8888 |
            PixelFormatEnum::ARGB2101010
                => 4,
            // YUV formats
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV
                => 2,
            PixelFormatEnum::YUY2 | PixelFormatEnum::UYVY |
            PixelFormatEnum::YVYU
                => 2,
            // Unsupported formats
            PixelFormatEnum::Index8
                => 1,
            PixelFormatEnum::Unknown | PixelFormatEnum::Index1LSB |
            PixelFormatEnum::Index1MSB | PixelFormatEnum::Index4LSB |
            PixelFormatEnum::Index4MSB
                => panic!("not supported format: {:?}", *self),
        }
    }

    pub fn supports_alpha(&self) -> bool {
        use self::PixelFormatEnum::*;
        match *self {
            ARGB4444 | ARGB1555 | ARGB8888 | ARGB2101010 |
            ABGR4444 | ABGR1555 | ABGR8888 |
            BGRA4444 | BGRA5551 | BGRA8888 |
            RGBA4444 | RGBA5551 | RGBA8888 => true,
            _ => false
        }
    }
}

impl FromPrimitive for PixelFormatEnum {
    fn from_i64(n: i64) -> Option<PixelFormatEnum> {
        use self::PixelFormatEnum::*;
        let n = n as u32;

        Some( match unsafe { transmute(n) } {
            SDL_PIXELFORMAT_UNKNOWN     => Unknown,
            SDL_PIXELFORMAT_INDEX1LSB   => Index1LSB,
            SDL_PIXELFORMAT_INDEX1MSB   => Index1MSB,
            SDL_PIXELFORMAT_INDEX4LSB   => Index4LSB,
            SDL_PIXELFORMAT_INDEX4MSB   => Index4MSB,
            SDL_PIXELFORMAT_INDEX8      => Index8,
            SDL_PIXELFORMAT_RGB332      => RGB332,
            SDL_PIXELFORMAT_RGB444      => RGB444,
            SDL_PIXELFORMAT_RGB555      => RGB555,
            SDL_PIXELFORMAT_BGR555      => BGR555,
            SDL_PIXELFORMAT_ARGB4444    => ARGB4444,
            SDL_PIXELFORMAT_RGBA4444    => RGBA4444,
            SDL_PIXELFORMAT_ABGR4444    => ABGR4444,
            SDL_PIXELFORMAT_BGRA4444    => BGRA4444,
            SDL_PIXELFORMAT_ARGB1555    => ARGB1555,
            SDL_PIXELFORMAT_RGBA5551    => RGBA5551,
            SDL_PIXELFORMAT_ABGR1555    => ABGR1555,
            SDL_PIXELFORMAT_BGRA5551    => BGRA5551,
            SDL_PIXELFORMAT_RGB565      => RGB565,
            SDL_PIXELFORMAT_BGR565      => BGR565,
            SDL_PIXELFORMAT_RGB24       => RGB24,
            SDL_PIXELFORMAT_BGR24       => BGR24,
            SDL_PIXELFORMAT_RGB888      => RGB888,
            SDL_PIXELFORMAT_RGBX8888    => RGBX8888,
            SDL_PIXELFORMAT_BGR888      => BGR888,
            SDL_PIXELFORMAT_BGRX8888    => BGRX8888,
            SDL_PIXELFORMAT_ARGB8888    => ARGB8888,
            SDL_PIXELFORMAT_RGBA8888    => RGBA8888,
            SDL_PIXELFORMAT_ABGR8888    => ABGR8888,
            SDL_PIXELFORMAT_BGRA8888    => BGRA8888,
            SDL_PIXELFORMAT_ARGB2101010 => ARGB2101010,
            SDL_PIXELFORMAT_YV12        => YV12,
            SDL_PIXELFORMAT_IYUV        => IYUV,
            SDL_PIXELFORMAT_YUY2        => YUY2,
            SDL_PIXELFORMAT_UYVY        => UYVY,
            SDL_PIXELFORMAT_YVYU        => YVYU,
            _                               => return None,
        })
    }

    fn from_u64(n: u64) -> Option<PixelFormatEnum> { FromPrimitive::from_i64(n as i64) }
}

pub struct PixelMasks {
    /// Bits per pixel; usually 15, 16, or 32
    pub bpp: u8,
    /// The red mask
    pub rmask: u32,
    /// The green mask
    pub gmask: u32,
    /// The blue mask
    pub bmask: u32,
    /// The alpha mask
    pub amask: u32
}

#[cfg(windows)]
pub unsafe fn get_sdl_error() -> String{
    CString::from_raw((SDL2.SDL_GetError)() as *mut i8).into_string().unwrap_or_else(|err|{
        String::new()
    })
}

#[cfg(unix)]
pub unsafe fn get_sdl_error() -> String{
    CString::from_raw((SDL2.SDL_GetError)() as *mut u8).into_string().unwrap_or_else(|err|{
        String::new()
    })
}