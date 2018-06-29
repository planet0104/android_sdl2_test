#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SDL_bool { SDL_FALSE = 0, SDL_TRUE = 1, }
/// \brief A signed 8-bit integer type.
pub type Sint8 = i8;
/// \brief An unsigned 8-bit integer type.
pub type Uint8 = u8;
/// \brief A signed 16-bit integer type.
pub type Sint16 = i16;
/// \brief An unsigned 16-bit integer type.
pub type Uint16 = u16;
/// \brief A signed 32-bit integer type.
pub type Sint32 = i32;
/// \brief An unsigned 32-bit integer type.
pub type Uint32 = u32;
/// \brief A signed 64-bit integer type.
pub type Sint64 = i64;
/// \brief An unsigned 64-bit integer type.
pub type Uint64 = u64;

pub type SDL_TouchID = Sint64;
pub type SDL_FingerID = Sint64;
pub type SDL_GestureID = Sint64;
pub type SDL_JoystickID = Sint32;

/// \brief A collection of pixels used in software blitting.
///
/// \note  This structure should be treated as read-only, except for \c pixels,
/// which, if not NULL, contains the raw pixel data for the surface.
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_Surface {
    /// < Read-only
    pub flags: Uint32,
    /// < Read-only
    pub format: *mut SDL_PixelFormat,
    /// < Read-only
    pub w: ::std::os::raw::c_int,
    /// < Read-only
    pub h: ::std::os::raw::c_int,
    /// < Read-only
    pub pitch: ::std::os::raw::c_int,
    /// < Read-write
    pub pixels: *mut ::std::os::raw::c_void,
    /// < Read-write
    pub userdata: *mut ::std::os::raw::c_void,
    /// < Read-only
    pub locked: ::std::os::raw::c_int,
    /// < Read-only
    pub lock_data: *mut ::std::os::raw::c_void,
    /// < Read-only
    pub clip_rect: SDL_Rect,
    /// < Private
    pub map: *mut SDL_BlitMap,
    /// < Read-mostly
    pub refcount: ::std::os::raw::c_int,
}
impl Clone for SDL_Surface {
    fn clone(&self) -> Self { *self }
}

/// < Private
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_BlitMap {
    pub _address: u8,
}
impl Clone for SDL_BlitMap {
    fn clone(&self) -> Self { *self }
}

/// \brief A rectangle, with the origin at the upper left.
///
/// \sa SDL_RectEmpty
/// \sa SDL_RectEquals
/// \sa SDL_HasIntersection
/// \sa SDL_IntersectRect
/// \sa SDL_UnionRect
/// \sa SDL_EnclosePoints
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_Rect {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
}
impl Clone for SDL_Rect {
    fn clone(&self) -> Self { *self }
}
impl SDL_Rect{
    pub fn new(x: i32, y: i32, width: i32, height: i32) ->SDL_Rect{
        SDL_Rect{
            x, y, w:width, h:height
        }
    }
}

/// \note Everything in the pixel format structure is read-only.
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_PixelFormat {
    pub format: Uint32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub padding: [Uint8; 2usize],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub refcount: ::std::os::raw::c_int,
    pub next: *mut SDL_PixelFormat,
}
impl Clone for SDL_PixelFormat {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_Renderer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_Texture {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_Palette {
    pub ncolors: ::std::os::raw::c_int,
    pub colors: *mut SDL_Color,
    pub version: Uint32,
    pub refcount: ::std::os::raw::c_int,
}
impl Clone for SDL_Palette {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
impl Clone for SDL_Color {
    fn clone(&self) -> Self { *self }
}

/// This is the read/write operation structure -- very basic.
#[repr(C)]
#[derive(Copy)]
pub struct SDL_RWops {
    /// Return the size of the file in this rwops, or -1 if unknown
    pub size: ::std::option::Option<unsafe extern "C" fn(context:
                                                             *mut SDL_RWops)
                                        -> Sint64>,
    /// Seek to \c offset relative to \c whence, one of stdio's whence values:
    /// RW_SEEK_SET, RW_SEEK_CUR, RW_SEEK_END
    ///
    /// \return the final offset in the data stream, or -1 on error.
    pub seek: ::std::option::Option<unsafe extern "C" fn(context:
                                                             *mut SDL_RWops,
                                                         offset: Sint64,
                                                         whence:
                                                             ::std::os::raw::c_int)
                                        -> Sint64>,
    /// Read up to \c maxnum objects each of size \c size from the data
    /// stream to the area pointed at by \c ptr.
    ///
    /// \return the number of objects read, or 0 at error or end of file.
    pub read: ::std::option::Option<unsafe extern "C" fn(context:
                                                             *mut SDL_RWops,
                                                         ptr:
                                                             *mut ::std::os::raw::c_void,
                                                         size: usize,
                                                         maxnum: usize)
                                        -> usize>,
    /// Write exactly \c num objects each of size \c size from the area
    /// pointed at by \c ptr to data stream.
    ///
    /// \return the number of objects written, or 0 at error or end of file.
    pub write: ::std::option::Option<unsafe extern "C" fn(context:
                                                              *mut SDL_RWops,
                                                          ptr:
                                                              *const ::std::os::raw::c_void,
                                                          size: usize,
                                                          num: usize)
                                         -> usize>,
    /// Close and free an allocated SDL_RWops structure.
    ///
    /// \return 0 if successful or -1 on write error when flushing data.
    pub close: ::std::option::Option<unsafe extern "C" fn(context:
                                                              *mut SDL_RWops)
                                         -> ::std::os::raw::c_int>,
    pub type_: Uint32,
    pub hidden: SDL_RWops__bindgen_ty_1,
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_RWops__bindgen_ty_1__bindgen_ty_1 {
    pub base: *mut Uint8,
    pub here: *mut Uint8,
    pub stop: *mut Uint8,
}
impl Clone for SDL_RWops__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub union SDL_RWops__bindgen_ty_1 {
    pub mem: SDL_RWops__bindgen_ty_1__bindgen_ty_1,
    pub unknown: SDL_RWops__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u64; 3usize],
}
impl Clone for SDL_RWops__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_RWops__bindgen_ty_1__bindgen_ty_2 {
    pub data1: *mut ::std::os::raw::c_void,
    pub data2: *mut ::std::os::raw::c_void,
}
impl Clone for SDL_RWops__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self { *self }
}

impl Clone for SDL_RWops {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_Window {
    _unused: [u8; 0],
}

/// \brief General event structure
#[repr(C)]
#[derive(Copy)]
pub union SDL_Event {
    /// < Event type, shared with all events
    pub type_: Uint32,
    /// < Common event data
    pub common: SDL_CommonEvent,
    /// < Window event data
    pub window: SDL_WindowEvent,
    /// < Keyboard event data
    pub key: SDL_KeyboardEvent,
    /// < Text editing event data
    pub edit: SDL_TextEditingEvent,
    /// < Text input event data
    pub text: SDL_TextInputEvent,
    /// < Mouse motion event data
    pub motion: SDL_MouseMotionEvent,
    /// < Mouse button event data
    pub button: SDL_MouseButtonEvent,
    /// < Mouse wheel event data
    pub wheel: SDL_MouseWheelEvent,
    /// < Joystick axis event data
    pub jaxis: SDL_JoyAxisEvent,
    /// < Joystick ball event data
    pub jball: SDL_JoyBallEvent,
    /// < Joystick hat event data
    pub jhat: SDL_JoyHatEvent,
    /// < Joystick button event data
    pub jbutton: SDL_JoyButtonEvent,
    /// < Joystick device change event data
    pub jdevice: SDL_JoyDeviceEvent,
    /// < Game Controller axis event data
    pub caxis: SDL_ControllerAxisEvent,
    /// < Game Controller button event data
    pub cbutton: SDL_ControllerButtonEvent,
    /// < Game Controller device event data
    pub cdevice: SDL_ControllerDeviceEvent,
    /// < Audio device event data
    pub adevice: SDL_AudioDeviceEvent,
    /// < Quit request event data
    pub quit: SDL_QuitEvent,
    /// < Custom event data
    pub user: SDL_UserEvent,
    /// < System dependent window event data
    pub syswm: SDL_SysWMEvent,
    /// < Touch finger event data
    pub tfinger: SDL_TouchFingerEvent,
    /// < Gesture event data
    pub mgesture: SDL_MultiGestureEvent,
    /// < Gesture event data
    pub dgesture: SDL_DollarGestureEvent,
    /// < Drag and drop event data
    pub drop: SDL_DropEvent,
    pub padding: [Uint8; 56usize],
    _bindgen_union_align: [u64; 7usize],
}

/// \brief Fields shared by every event
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_CommonEvent {
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
}
impl Clone for SDL_CommonEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_WindowEvent {
    /// < ::SDL_WINDOWEVENT
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The associated window
    pub windowID: Uint32,
    /// < ::SDL_WindowEventID
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    /// < event dependent data
    pub data1: Sint32,
    /// < event dependent data
    pub data2: Sint32,
}
impl Clone for SDL_WindowEvent {
    fn clone(&self) -> Self { *self }
}

/// \brief Keyboard button event structure (event.key.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_KeyboardEvent {
    /// < ::SDL_KEYDOWN or ::SDL_KEYUP
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The window with keyboard focus, if any
    pub windowID: Uint32,
    /// < ::SDL_PRESSED or ::SDL_RELEASED
    pub state: Uint8,
    /// < Non-zero if this is a key repeat
    pub repeat: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    /// < The key that was pressed or released
    pub keysym: SDL_Keysym,
}
impl Clone for SDL_KeyboardEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Keyboard text editing event structure (event.edit.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_TextEditingEvent {
    /// < ::SDL_TEXTEDITING
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The window with keyboard focus, if any
    pub windowID: Uint32,
    /// < The editing text
    pub text: [::std::os::raw::c_char; 32usize],
    /// < The start cursor of selected editing text
    pub start: Sint32,
    /// < The length of selected editing text
    pub length: Sint32,
}
impl Clone for SDL_TextEditingEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Keyboard text input event structure (event.text.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_TextInputEvent {
    /// < ::SDL_TEXTINPUT
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The window with keyboard focus, if any
    pub windowID: Uint32,
    /// < The input text
    pub text: [::std::os::raw::c_char; 32usize],
}
impl Clone for SDL_TextInputEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Mouse motion event structure (event.motion.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_MouseMotionEvent {
    /// < ::SDL_MOUSEMOTION
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The window with mouse focus, if any
    pub windowID: Uint32,
    /// < The mouse instance id, or SDL_TOUCH_MOUSEID
    pub which: Uint32,
    /// < The current button state
    pub state: Uint32,
    /// < X coordinate, relative to window
    pub x: Sint32,
    /// < Y coordinate, relative to window
    pub y: Sint32,
    /// < The relative motion in the X direction
    pub xrel: Sint32,
    /// < The relative motion in the Y direction
    pub yrel: Sint32,
}

impl Clone for SDL_MouseMotionEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Mouse button event structure (event.button.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_MouseButtonEvent {
    /// < ::SDL_MOUSEBUTTONDOWN or ::SDL_MOUSEBUTTONUP
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The window with mouse focus, if any
    pub windowID: Uint32,
    /// < The mouse instance id, or SDL_TOUCH_MOUSEID
    pub which: Uint32,
    /// < The mouse button index
    pub button: Uint8,
    /// < ::SDL_PRESSED or ::SDL_RELEASED
    pub state: Uint8,
    /// < 1 for single-click, 2 for double-click, etc.
    pub clicks: Uint8,
    pub padding1: Uint8,
    /// < X coordinate, relative to window
    pub x: Sint32,
    /// < Y coordinate, relative to window
    pub y: Sint32,
}

impl Clone for SDL_MouseButtonEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Mouse wheel event structure (event.wheel.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_MouseWheelEvent {
    /// < ::SDL_MOUSEWHEEL
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The window with mouse focus, if any
    pub windowID: Uint32,
    /// < The mouse instance id, or SDL_TOUCH_MOUSEID
    pub which: Uint32,
    /// < The amount scrolled horizontally, positive to the right and negative to the left
    pub x: Sint32,
    /// < The amount scrolled vertically, positive away from the user and negative toward the user
    pub y: Sint32,
    /// < Set to one of the SDL_MOUSEWHEEL_* defines. When FLIPPED the values in X and Y will be opposite. Multiply by -1 to change them back
    pub direction: Uint32,
}

impl Clone for SDL_MouseWheelEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Joystick axis motion event structure (event.jaxis.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_JoyAxisEvent {
    /// < ::SDL_JOYAXISMOTION
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The joystick instance id
    pub which: SDL_JoystickID,
    /// < The joystick axis index
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    /// < The axis value (range: -32768 to 32767)
    pub value: Sint16,
    pub padding4: Uint16,
}

impl Clone for SDL_JoyAxisEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Joystick trackball motion event structure (event.jball.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_JoyBallEvent {
    /// < ::SDL_JOYBALLMOTION
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The joystick instance id
    pub which: SDL_JoystickID,
    /// < The joystick trackball index
    pub ball: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    /// < The relative motion in the X direction
    pub xrel: Sint16,
    /// < The relative motion in the Y direction
    pub yrel: Sint16,
}

impl Clone for SDL_JoyBallEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Joystick hat position change event structure (event.jhat.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_JoyHatEvent {
    /// < ::SDL_JOYHATMOTION
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The joystick instance id
    pub which: SDL_JoystickID,
    /// < The joystick hat index
    pub hat: Uint8,
    /// < The hat position value.
    /// \sa ::SDL_HAT_LEFTUP ::SDL_HAT_UP ::SDL_HAT_RIGHTUP
    /// \sa ::SDL_HAT_LEFT ::SDL_HAT_CENTERED ::SDL_HAT_RIGHT
    /// \sa ::SDL_HAT_LEFTDOWN ::SDL_HAT_DOWN ::SDL_HAT_RIGHTDOWN
    ///
    /// Note that zero means the POV is centered.
    pub value: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}

impl Clone for SDL_JoyHatEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Joystick button event structure (event.jbutton.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_JoyButtonEvent {
    /// < ::SDL_JOYBUTTONDOWN or ::SDL_JOYBUTTONUP
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The joystick instance id
    pub which: SDL_JoystickID,
    /// < The joystick button index
    pub button: Uint8,
    /// < ::SDL_PRESSED or ::SDL_RELEASED
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}

impl Clone for SDL_JoyButtonEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Joystick device event structure (event.jdevice.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_JoyDeviceEvent {
    /// < ::SDL_JOYDEVICEADDED or ::SDL_JOYDEVICEREMOVED
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The joystick device index for the ADDED event, instance id for the REMOVED event
    pub which: Sint32,
}
impl Clone for SDL_JoyDeviceEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Game controller axis motion event structure (event.caxis.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_ControllerAxisEvent {
    /// < ::SDL_CONTROLLERAXISMOTION
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The joystick instance id
    pub which: SDL_JoystickID,
    /// < The controller axis (SDL_GameControllerAxis)
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    /// < The axis value (range: -32768 to 32767)
    pub value: Sint16,
    pub padding4: Uint16,
}
impl Clone for SDL_ControllerAxisEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Game controller button event structure (event.cbutton.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_ControllerButtonEvent {
    /// < ::SDL_CONTROLLERBUTTONDOWN or ::SDL_CONTROLLERBUTTONUP
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The joystick instance id
    pub which: SDL_JoystickID,
    /// < The controller button (SDL_GameControllerButton)
    pub button: Uint8,
    /// < ::SDL_PRESSED or ::SDL_RELEASED
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
impl Clone for SDL_ControllerButtonEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Controller device event structure (event.cdevice.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_ControllerDeviceEvent {
    /// < ::SDL_CONTROLLERDEVICEADDED, ::SDL_CONTROLLERDEVICEREMOVED, or ::SDL_CONTROLLERDEVICEREMAPPED
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The joystick device index for the ADDED event, instance id for the REMOVED or REMAPPED event
    pub which: Sint32,
}
impl Clone for SDL_ControllerDeviceEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Audio device event structure (event.adevice.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_AudioDeviceEvent {
    /// < ::SDL_AUDIODEVICEADDED, or ::SDL_AUDIODEVICEREMOVED
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The audio device index for the ADDED event (valid until next SDL_GetNumAudioDevices() call), SDL_AudioDeviceID for the REMOVED event
    pub which: Uint32,
    /// < zero if an output device, non-zero if a capture device.
    pub iscapture: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}
impl Clone for SDL_AudioDeviceEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Touch finger event structure (event.tfinger.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_TouchFingerEvent {
    /// < ::SDL_FINGERMOTION or ::SDL_FINGERDOWN or ::SDL_FINGERUP
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The touch device id
    pub touchId: SDL_TouchID,
    pub fingerId: SDL_FingerID,
    /// < Normalized in the range 0...1
    pub x: f32,
    /// < Normalized in the range 0...1
    pub y: f32,
    /// < Normalized in the range -1...1
    pub dx: f32,
    /// < Normalized in the range -1...1
    pub dy: f32,
    /// < Normalized in the range 0...1
    pub pressure: f32,
}
impl Clone for SDL_TouchFingerEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Multiple Finger Gesture Event (event.mgesture.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_MultiGestureEvent {
    /// < ::SDL_MULTIGESTURE
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The touch device id
    pub touchId: SDL_TouchID,
    pub dTheta: f32,
    pub dDist: f32,
    pub x: f32,
    pub y: f32,
    pub numFingers: Uint16,
    pub padding: Uint16,
}
impl Clone for SDL_MultiGestureEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief Dollar Gesture Event (event.dgesture.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_DollarGestureEvent {
    /// < ::SDL_DOLLARGESTURE or ::SDL_DOLLARRECORD
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The touch device id
    pub touchId: SDL_TouchID,
    pub gestureId: SDL_GestureID,
    pub numFingers: Uint32,
    pub error: f32,
    /// < Normalized center of gesture
    pub x: f32,
    /// < Normalized center of gesture
    pub y: f32,
}
impl Clone for SDL_DollarGestureEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief An event used to request a file open by the system (event.drop.*)
/// This event is enabled by default, you can disable it with SDL_EventState().
/// \note If this event is enabled, you must free the filename in the event.
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_DropEvent {
    /// < ::SDL_DROPBEGIN or ::SDL_DROPFILE or ::SDL_DROPTEXT or ::SDL_DROPCOMPLETE
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The file name, which should be freed with SDL_free(), is NULL on begin/complete
    pub file: *mut ::std::os::raw::c_char,
    /// < The window that was dropped on, if any
    pub windowID: Uint32,
}
impl Clone for SDL_DropEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief The "quit requested" event
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_QuitEvent {
    /// < ::SDL_QUIT
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
}
impl Clone for SDL_QuitEvent {
    fn clone(&self) -> Self { *self }
}
/// \brief OS Specific event
// #[repr(C)]
// #[derive(Debug, Copy)]
// pub struct SDL_OSEvent {
//     /// < ::SDL_QUIT
//     pub type_: Uint32,
//     /// < In milliseconds, populated using SDL_GetTicks()
//     pub timestamp: Uint32,
// }
// impl Clone for SDL_OSEvent {
//     fn clone(&self) -> Self { *self }
// }
/// \brief A user-defined event type (event.user.*)
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_UserEvent {
    /// < ::SDL_USEREVENT through ::SDL_LASTEVENT-1
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < The associated window if any
    pub windowID: Uint32,
    /// < User defined event code
    pub code: Sint32,
    /// < User defined data pointer
    pub data1: *mut ::std::os::raw::c_void,
    /// < User defined data pointer
    pub data2: *mut ::std::os::raw::c_void,
}
impl Clone for SDL_UserEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_SysWMmsg {
    _unused: [u8; 0],
}
/// \brief A video driver dependent system event (event.syswm.*)
/// This event is disabled by default, you can enable it with SDL_EventState()
///
/// \note If you want to use this event, you should include SDL_syswm.h.
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_SysWMEvent {
    /// < ::SDL_SYSWMEVENT
    pub type_: Uint32,
    /// < In milliseconds, populated using SDL_GetTicks()
    pub timestamp: Uint32,
    /// < driver dependent data, defined in SDL_syswm.h
    pub msg: *mut SDL_SysWMmsg,
}
impl Clone for SDL_SysWMEvent {
    fn clone(&self) -> Self { *self }
}
impl Clone for SDL_Event {
    fn clone(&self) -> Self { *self }
}
// #[repr(u32)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub enum SDL_eventaction {
//     SDL_ADDEVENT = 0,
//     SDL_PEEKEVENT = 1,
//     SDL_GETEVENT = 2,
// }

/// \brief The SDL keysym structure, used in key events.
///
/// \note  If you are looking for translated character input, see the ::SDL_TEXTINPUT event.
#[repr(C)]
#[derive(Debug, Copy)]
pub struct SDL_Keysym {
    /// < SDL physical key code - see ::SDL_Scancode for details
    pub scancode: SDL_Scancode,
    /// < SDL virtual key code - see ::SDL_Keycode for details
    pub sym: SDL_Keycode,
    /// < current key modifiers
    pub mod_: Uint16,
    pub unused: Uint32,
}
impl Clone for SDL_Keysym {
    fn clone(&self) -> Self { *self }
}
// \brief The SDL keyboard scancode representation.
//
// Values of this type are used to represent keyboard keys, among other places
// in the \link SDL_Keysym::scancode key.keysym.scancode \endlink field of the
// SDL_Event structure.
//
// The values in this enumeration are based on the USB usage page standard:
// http://www.usb.org/developers/hidpage/Hut1_12v2.pdf
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SDL_Scancode {
    SDL_SCANCODE_UNKNOWN = 0,
    // SDL_SCANCODE_A = 4,
    // SDL_SCANCODE_B = 5,
    // SDL_SCANCODE_C = 6,
    // SDL_SCANCODE_D = 7,
    // SDL_SCANCODE_E = 8,
    // SDL_SCANCODE_F = 9,
    // SDL_SCANCODE_G = 10,
    // SDL_SCANCODE_H = 11,
    // SDL_SCANCODE_I = 12,
    // SDL_SCANCODE_J = 13,
    // SDL_SCANCODE_K = 14,
    // SDL_SCANCODE_L = 15,
    // SDL_SCANCODE_M = 16,
    // SDL_SCANCODE_N = 17,
    // SDL_SCANCODE_O = 18,
    // SDL_SCANCODE_P = 19,
    // SDL_SCANCODE_Q = 20,
    // SDL_SCANCODE_R = 21,
    // SDL_SCANCODE_S = 22,
    // SDL_SCANCODE_T = 23,
    // SDL_SCANCODE_U = 24,
    // SDL_SCANCODE_V = 25,
    // SDL_SCANCODE_W = 26,
    // SDL_SCANCODE_X = 27,
    // SDL_SCANCODE_Y = 28,
    // SDL_SCANCODE_Z = 29,
    // SDL_SCANCODE_1 = 30,
    // SDL_SCANCODE_2 = 31,
    // SDL_SCANCODE_3 = 32,
    // SDL_SCANCODE_4 = 33,
    // SDL_SCANCODE_5 = 34,
    // SDL_SCANCODE_6 = 35,
    // SDL_SCANCODE_7 = 36,
    // SDL_SCANCODE_8 = 37,
    // SDL_SCANCODE_9 = 38,
    // SDL_SCANCODE_0 = 39,
    // SDL_SCANCODE_RETURN = 40,
    // SDL_SCANCODE_ESCAPE = 41,
    // SDL_SCANCODE_BACKSPACE = 42,
    // SDL_SCANCODE_TAB = 43,
    // SDL_SCANCODE_SPACE = 44,
    // SDL_SCANCODE_MINUS = 45,
    // SDL_SCANCODE_EQUALS = 46,
    // SDL_SCANCODE_LEFTBRACKET = 47,
    // SDL_SCANCODE_RIGHTBRACKET = 48,
    // SDL_SCANCODE_BACKSLASH = 49,
    // SDL_SCANCODE_NONUSHASH = 50,
    // SDL_SCANCODE_SEMICOLON = 51,
    // SDL_SCANCODE_APOSTROPHE = 52,
    // SDL_SCANCODE_GRAVE = 53,
    // SDL_SCANCODE_COMMA = 54,
    // SDL_SCANCODE_PERIOD = 55,
    // SDL_SCANCODE_SLASH = 56,
    // SDL_SCANCODE_CAPSLOCK = 57,
    // SDL_SCANCODE_F1 = 58,
    // SDL_SCANCODE_F2 = 59,
    // SDL_SCANCODE_F3 = 60,
    // SDL_SCANCODE_F4 = 61,
    // SDL_SCANCODE_F5 = 62,
    // SDL_SCANCODE_F6 = 63,
    // SDL_SCANCODE_F7 = 64,
    // SDL_SCANCODE_F8 = 65,
    // SDL_SCANCODE_F9 = 66,
    // SDL_SCANCODE_F10 = 67,
    // SDL_SCANCODE_F11 = 68,
    // SDL_SCANCODE_F12 = 69,
    // SDL_SCANCODE_PRINTSCREEN = 70,
    // SDL_SCANCODE_SCROLLLOCK = 71,
    // SDL_SCANCODE_PAUSE = 72,
    // SDL_SCANCODE_INSERT = 73,
    // SDL_SCANCODE_HOME = 74,
    // SDL_SCANCODE_PAGEUP = 75,
    // SDL_SCANCODE_DELETE = 76,
    // SDL_SCANCODE_END = 77,
    // SDL_SCANCODE_PAGEDOWN = 78,
    // SDL_SCANCODE_RIGHT = 79,
    // SDL_SCANCODE_LEFT = 80,
    // SDL_SCANCODE_DOWN = 81,
    // SDL_SCANCODE_UP = 82,
    // SDL_SCANCODE_NUMLOCKCLEAR = 83,
    // SDL_SCANCODE_KP_DIVIDE = 84,
    // SDL_SCANCODE_KP_MULTIPLY = 85,
    // SDL_SCANCODE_KP_MINUS = 86,
    // SDL_SCANCODE_KP_PLUS = 87,
    // SDL_SCANCODE_KP_ENTER = 88,
    // SDL_SCANCODE_KP_1 = 89,
    // SDL_SCANCODE_KP_2 = 90,
    // SDL_SCANCODE_KP_3 = 91,
    // SDL_SCANCODE_KP_4 = 92,
    // SDL_SCANCODE_KP_5 = 93,
    // SDL_SCANCODE_KP_6 = 94,
    // SDL_SCANCODE_KP_7 = 95,
    // SDL_SCANCODE_KP_8 = 96,
    // SDL_SCANCODE_KP_9 = 97,
    // SDL_SCANCODE_KP_0 = 98,
    // SDL_SCANCODE_KP_PERIOD = 99,
    // SDL_SCANCODE_NONUSBACKSLASH = 100,
    // SDL_SCANCODE_APPLICATION = 101,
    // SDL_SCANCODE_POWER = 102,
    // SDL_SCANCODE_KP_EQUALS = 103,
    // SDL_SCANCODE_F13 = 104,
    // SDL_SCANCODE_F14 = 105,
    // SDL_SCANCODE_F15 = 106,
    // SDL_SCANCODE_F16 = 107,
    // SDL_SCANCODE_F17 = 108,
    // SDL_SCANCODE_F18 = 109,
    // SDL_SCANCODE_F19 = 110,
    // SDL_SCANCODE_F20 = 111,
    // SDL_SCANCODE_F21 = 112,
    // SDL_SCANCODE_F22 = 113,
    // SDL_SCANCODE_F23 = 114,
    // SDL_SCANCODE_F24 = 115,
    // SDL_SCANCODE_EXECUTE = 116,
    // SDL_SCANCODE_HELP = 117,
    // SDL_SCANCODE_MENU = 118,
    // SDL_SCANCODE_SELECT = 119,
    // SDL_SCANCODE_STOP = 120,
    // SDL_SCANCODE_AGAIN = 121,
    // SDL_SCANCODE_UNDO = 122,
    // SDL_SCANCODE_CUT = 123,
    // SDL_SCANCODE_COPY = 124,
    // SDL_SCANCODE_PASTE = 125,
    // SDL_SCANCODE_FIND = 126,
    // SDL_SCANCODE_MUTE = 127,
    // SDL_SCANCODE_VOLUMEUP = 128,
    // SDL_SCANCODE_VOLUMEDOWN = 129,
    // SDL_SCANCODE_KP_COMMA = 133,
    // SDL_SCANCODE_KP_EQUALSAS400 = 134,
    // SDL_SCANCODE_INTERNATIONAL1 = 135,
    // SDL_SCANCODE_INTERNATIONAL2 = 136,
    // SDL_SCANCODE_INTERNATIONAL3 = 137,
    // SDL_SCANCODE_INTERNATIONAL4 = 138,
    // SDL_SCANCODE_INTERNATIONAL5 = 139,
    // SDL_SCANCODE_INTERNATIONAL6 = 140,
    // SDL_SCANCODE_INTERNATIONAL7 = 141,
    // SDL_SCANCODE_INTERNATIONAL8 = 142,
    // SDL_SCANCODE_INTERNATIONAL9 = 143,
    // SDL_SCANCODE_LANG1 = 144,
    // SDL_SCANCODE_LANG2 = 145,
    // SDL_SCANCODE_LANG3 = 146,
    // SDL_SCANCODE_LANG4 = 147,
    // SDL_SCANCODE_LANG5 = 148,
    // SDL_SCANCODE_LANG6 = 149,
    // SDL_SCANCODE_LANG7 = 150,
    // SDL_SCANCODE_LANG8 = 151,
    // SDL_SCANCODE_LANG9 = 152,
    // SDL_SCANCODE_ALTERASE = 153,
    // SDL_SCANCODE_SYSREQ = 154,
    // SDL_SCANCODE_CANCEL = 155,
    // SDL_SCANCODE_CLEAR = 156,
    // SDL_SCANCODE_PRIOR = 157,
    // SDL_SCANCODE_RETURN2 = 158,
    // SDL_SCANCODE_SEPARATOR = 159,
    // SDL_SCANCODE_OUT = 160,
    // SDL_SCANCODE_OPER = 161,
    // SDL_SCANCODE_CLEARAGAIN = 162,
    // SDL_SCANCODE_CRSEL = 163,
    // SDL_SCANCODE_EXSEL = 164,
    // SDL_SCANCODE_KP_00 = 176,
    // SDL_SCANCODE_KP_000 = 177,
    // SDL_SCANCODE_THOUSANDSSEPARATOR = 178,
    // SDL_SCANCODE_DECIMALSEPARATOR = 179,
    // SDL_SCANCODE_CURRENCYUNIT = 180,
    // SDL_SCANCODE_CURRENCYSUBUNIT = 181,
    // SDL_SCANCODE_KP_LEFTPAREN = 182,
    // SDL_SCANCODE_KP_RIGHTPAREN = 183,
    // SDL_SCANCODE_KP_LEFTBRACE = 184,
    // SDL_SCANCODE_KP_RIGHTBRACE = 185,
    // SDL_SCANCODE_KP_TAB = 186,
    // SDL_SCANCODE_KP_BACKSPACE = 187,
    // SDL_SCANCODE_KP_A = 188,
    // SDL_SCANCODE_KP_B = 189,
    // SDL_SCANCODE_KP_C = 190,
    // SDL_SCANCODE_KP_D = 191,
    // SDL_SCANCODE_KP_E = 192,
    // SDL_SCANCODE_KP_F = 193,
    // SDL_SCANCODE_KP_XOR = 194,
    // SDL_SCANCODE_KP_POWER = 195,
    // SDL_SCANCODE_KP_PERCENT = 196,
    // SDL_SCANCODE_KP_LESS = 197,
    // SDL_SCANCODE_KP_GREATER = 198,
    // SDL_SCANCODE_KP_AMPERSAND = 199,
    // SDL_SCANCODE_KP_DBLAMPERSAND = 200,
    // SDL_SCANCODE_KP_VERTICALBAR = 201,
    // SDL_SCANCODE_KP_DBLVERTICALBAR = 202,
    // SDL_SCANCODE_KP_COLON = 203,
    // SDL_SCANCODE_KP_HASH = 204,
    // SDL_SCANCODE_KP_SPACE = 205,
    // SDL_SCANCODE_KP_AT = 206,
    // SDL_SCANCODE_KP_EXCLAM = 207,
    // SDL_SCANCODE_KP_MEMSTORE = 208,
    // SDL_SCANCODE_KP_MEMRECALL = 209,
    // SDL_SCANCODE_KP_MEMCLEAR = 210,
    // SDL_SCANCODE_KP_MEMADD = 211,
    // SDL_SCANCODE_KP_MEMSUBTRACT = 212,
    // SDL_SCANCODE_KP_MEMMULTIPLY = 213,
    // SDL_SCANCODE_KP_MEMDIVIDE = 214,
    // SDL_SCANCODE_KP_PLUSMINUS = 215,
    // SDL_SCANCODE_KP_CLEAR = 216,
    // SDL_SCANCODE_KP_CLEARENTRY = 217,
    // SDL_SCANCODE_KP_BINARY = 218,
    // SDL_SCANCODE_KP_OCTAL = 219,
    // SDL_SCANCODE_KP_DECIMAL = 220,
    // SDL_SCANCODE_KP_HEXADECIMAL = 221,
    // SDL_SCANCODE_LCTRL = 224,
    // SDL_SCANCODE_LSHIFT = 225,
    // SDL_SCANCODE_LALT = 226,
    // SDL_SCANCODE_LGUI = 227,
    // SDL_SCANCODE_RCTRL = 228,
    // SDL_SCANCODE_RSHIFT = 229,
    // SDL_SCANCODE_RALT = 230,
    // SDL_SCANCODE_RGUI = 231,
    // SDL_SCANCODE_MODE = 257,
    // SDL_SCANCODE_AUDIONEXT = 258,
    // SDL_SCANCODE_AUDIOPREV = 259,
    // SDL_SCANCODE_AUDIOSTOP = 260,
    // SDL_SCANCODE_AUDIOPLAY = 261,
    // SDL_SCANCODE_AUDIOMUTE = 262,
    // SDL_SCANCODE_MEDIASELECT = 263,
    // SDL_SCANCODE_WWW = 264,
    // SDL_SCANCODE_MAIL = 265,
    // SDL_SCANCODE_CALCULATOR = 266,
    // SDL_SCANCODE_COMPUTER = 267,
    // SDL_SCANCODE_AC_SEARCH = 268,
    // SDL_SCANCODE_AC_HOME = 269,
    // SDL_SCANCODE_AC_BACK = 270,
    // SDL_SCANCODE_AC_FORWARD = 271,
    // SDL_SCANCODE_AC_STOP = 272,
    // SDL_SCANCODE_AC_REFRESH = 273,
    // SDL_SCANCODE_AC_BOOKMARKS = 274,
    // SDL_SCANCODE_BRIGHTNESSDOWN = 275,
    // SDL_SCANCODE_BRIGHTNESSUP = 276,
    // SDL_SCANCODE_DISPLAYSWITCH = 277,
    // SDL_SCANCODE_KBDILLUMTOGGLE = 278,
    // SDL_SCANCODE_KBDILLUMDOWN = 279,
    // SDL_SCANCODE_KBDILLUMUP = 280,
    // SDL_SCANCODE_EJECT = 281,
    // SDL_SCANCODE_SLEEP = 282,
    // SDL_SCANCODE_APP1 = 283,
    // SDL_SCANCODE_APP2 = 284,
    // SDL_SCANCODE_AUDIOREWIND = 285,
    // SDL_SCANCODE_AUDIOFASTFORWARD = 286,
    // SDL_NUM_SCANCODES = 512,
}
/// \brief The SDL virtual key representation.
///
/// Values of this type are used to represent keyboard keys using the current
/// layout of the keyboard.  These values include Unicode values representing
/// the unmodified character that would be generated by pressing the key, or
/// an SDLK_* constant for those keys that do not generate characters.
///
/// A special exception is the number keys at the top of the keyboard which
/// always map to SDLK_0...SDLK_9, regardless of layout.
pub type SDL_Keycode = Sint32;
// pub const SDLK_UNKNOWN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_UNKNOWN;
// pub const SDLK_RETURN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RETURN;
// pub const SDLK_ESCAPE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_ESCAPE;
// pub const SDLK_BACKSPACE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_BACKSPACE;
// pub const SDLK_TAB: _bindgen_ty_7 = _bindgen_ty_7::SDLK_TAB;
// pub const SDLK_SPACE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_SPACE;
// pub const SDLK_EXCLAIM: _bindgen_ty_7 = _bindgen_ty_7::SDLK_EXCLAIM;
// pub const SDLK_QUOTEDBL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_QUOTEDBL;
// pub const SDLK_HASH: _bindgen_ty_7 = _bindgen_ty_7::SDLK_HASH;
// pub const SDLK_PERCENT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PERCENT;
// pub const SDLK_DOLLAR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_DOLLAR;
// pub const SDLK_AMPERSAND: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AMPERSAND;
// pub const SDLK_QUOTE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_QUOTE;
// pub const SDLK_LEFTPAREN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_LEFTPAREN;
// pub const SDLK_RIGHTPAREN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RIGHTPAREN;
// pub const SDLK_ASTERISK: _bindgen_ty_7 = _bindgen_ty_7::SDLK_ASTERISK;
// pub const SDLK_PLUS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PLUS;
// pub const SDLK_COMMA: _bindgen_ty_7 = _bindgen_ty_7::SDLK_COMMA;
// pub const SDLK_MINUS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_MINUS;
// pub const SDLK_PERIOD: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PERIOD;
// pub const SDLK_SLASH: _bindgen_ty_7 = _bindgen_ty_7::SDLK_SLASH;
// pub const SDLK_0: _bindgen_ty_7 = _bindgen_ty_7::SDLK_0;
// pub const SDLK_1: _bindgen_ty_7 = _bindgen_ty_7::SDLK_1;
// pub const SDLK_2: _bindgen_ty_7 = _bindgen_ty_7::SDLK_2;
// pub const SDLK_3: _bindgen_ty_7 = _bindgen_ty_7::SDLK_3;
// pub const SDLK_4: _bindgen_ty_7 = _bindgen_ty_7::SDLK_4;
// pub const SDLK_5: _bindgen_ty_7 = _bindgen_ty_7::SDLK_5;
// pub const SDLK_6: _bindgen_ty_7 = _bindgen_ty_7::SDLK_6;
// pub const SDLK_7: _bindgen_ty_7 = _bindgen_ty_7::SDLK_7;
// pub const SDLK_8: _bindgen_ty_7 = _bindgen_ty_7::SDLK_8;
// pub const SDLK_9: _bindgen_ty_7 = _bindgen_ty_7::SDLK_9;
// pub const SDLK_COLON: _bindgen_ty_7 = _bindgen_ty_7::SDLK_COLON;
// pub const SDLK_SEMICOLON: _bindgen_ty_7 = _bindgen_ty_7::SDLK_SEMICOLON;
// pub const SDLK_LESS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_LESS;
// pub const SDLK_EQUALS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_EQUALS;
// pub const SDLK_GREATER: _bindgen_ty_7 = _bindgen_ty_7::SDLK_GREATER;
// pub const SDLK_QUESTION: _bindgen_ty_7 = _bindgen_ty_7::SDLK_QUESTION;
// pub const SDLK_AT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AT;
// pub const SDLK_LEFTBRACKET: _bindgen_ty_7 = _bindgen_ty_7::SDLK_LEFTBRACKET;
// pub const SDLK_BACKSLASH: _bindgen_ty_7 = _bindgen_ty_7::SDLK_BACKSLASH;
// pub const SDLK_RIGHTBRACKET: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RIGHTBRACKET;
// pub const SDLK_CARET: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CARET;
// pub const SDLK_UNDERSCORE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_UNDERSCORE;
// pub const SDLK_BACKQUOTE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_BACKQUOTE;
// pub const SDLK_a: _bindgen_ty_7 = _bindgen_ty_7::SDLK_a;
// pub const SDLK_b: _bindgen_ty_7 = _bindgen_ty_7::SDLK_b;
// pub const SDLK_c: _bindgen_ty_7 = _bindgen_ty_7::SDLK_c;
// pub const SDLK_d: _bindgen_ty_7 = _bindgen_ty_7::SDLK_d;
// pub const SDLK_e: _bindgen_ty_7 = _bindgen_ty_7::SDLK_e;
// pub const SDLK_f: _bindgen_ty_7 = _bindgen_ty_7::SDLK_f;
// pub const SDLK_g: _bindgen_ty_7 = _bindgen_ty_7::SDLK_g;
// pub const SDLK_h: _bindgen_ty_7 = _bindgen_ty_7::SDLK_h;
// pub const SDLK_i: _bindgen_ty_7 = _bindgen_ty_7::SDLK_i;
// pub const SDLK_j: _bindgen_ty_7 = _bindgen_ty_7::SDLK_j;
// pub const SDLK_k: _bindgen_ty_7 = _bindgen_ty_7::SDLK_k;
// pub const SDLK_l: _bindgen_ty_7 = _bindgen_ty_7::SDLK_l;
// pub const SDLK_m: _bindgen_ty_7 = _bindgen_ty_7::SDLK_m;
// pub const SDLK_n: _bindgen_ty_7 = _bindgen_ty_7::SDLK_n;
// pub const SDLK_o: _bindgen_ty_7 = _bindgen_ty_7::SDLK_o;
// pub const SDLK_p: _bindgen_ty_7 = _bindgen_ty_7::SDLK_p;
// pub const SDLK_q: _bindgen_ty_7 = _bindgen_ty_7::SDLK_q;
// pub const SDLK_r: _bindgen_ty_7 = _bindgen_ty_7::SDLK_r;
// pub const SDLK_s: _bindgen_ty_7 = _bindgen_ty_7::SDLK_s;
// pub const SDLK_t: _bindgen_ty_7 = _bindgen_ty_7::SDLK_t;
// pub const SDLK_u: _bindgen_ty_7 = _bindgen_ty_7::SDLK_u;
// pub const SDLK_v: _bindgen_ty_7 = _bindgen_ty_7::SDLK_v;
// pub const SDLK_w: _bindgen_ty_7 = _bindgen_ty_7::SDLK_w;
// pub const SDLK_x: _bindgen_ty_7 = _bindgen_ty_7::SDLK_x;
// pub const SDLK_y: _bindgen_ty_7 = _bindgen_ty_7::SDLK_y;
// pub const SDLK_z: _bindgen_ty_7 = _bindgen_ty_7::SDLK_z;
// pub const SDLK_CAPSLOCK: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CAPSLOCK;
// pub const SDLK_F1: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F1;
// pub const SDLK_F2: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F2;
// pub const SDLK_F3: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F3;
// pub const SDLK_F4: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F4;
// pub const SDLK_F5: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F5;
// pub const SDLK_F6: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F6;
// pub const SDLK_F7: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F7;
// pub const SDLK_F8: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F8;
// pub const SDLK_F9: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F9;
// pub const SDLK_F10: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F10;
// pub const SDLK_F11: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F11;
// pub const SDLK_F12: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F12;
// pub const SDLK_PRINTSCREEN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PRINTSCREEN;
// pub const SDLK_SCROLLLOCK: _bindgen_ty_7 = _bindgen_ty_7::SDLK_SCROLLLOCK;
// pub const SDLK_PAUSE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PAUSE;
// pub const SDLK_INSERT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_INSERT;
// pub const SDLK_HOME: _bindgen_ty_7 = _bindgen_ty_7::SDLK_HOME;
// pub const SDLK_PAGEUP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PAGEUP;
// pub const SDLK_DELETE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_DELETE;
// pub const SDLK_END: _bindgen_ty_7 = _bindgen_ty_7::SDLK_END;
// pub const SDLK_PAGEDOWN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PAGEDOWN;
// pub const SDLK_RIGHT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RIGHT;
// pub const SDLK_LEFT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_LEFT;
// pub const SDLK_DOWN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_DOWN;
// pub const SDLK_UP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_UP;
// pub const SDLK_NUMLOCKCLEAR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_NUMLOCKCLEAR;
// pub const SDLK_KP_DIVIDE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_DIVIDE;
// pub const SDLK_KP_MULTIPLY: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_MULTIPLY;
// pub const SDLK_KP_MINUS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_MINUS;
// pub const SDLK_KP_PLUS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_PLUS;
// pub const SDLK_KP_ENTER: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_ENTER;
// pub const SDLK_KP_1: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_1;
// pub const SDLK_KP_2: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_2;
// pub const SDLK_KP_3: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_3;
// pub const SDLK_KP_4: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_4;
// pub const SDLK_KP_5: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_5;
// pub const SDLK_KP_6: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_6;
// pub const SDLK_KP_7: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_7;
// pub const SDLK_KP_8: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_8;
// pub const SDLK_KP_9: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_9;
// pub const SDLK_KP_0: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_0;
// pub const SDLK_KP_PERIOD: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_PERIOD;
// pub const SDLK_APPLICATION: _bindgen_ty_7 = _bindgen_ty_7::SDLK_APPLICATION;
// pub const SDLK_POWER: _bindgen_ty_7 = _bindgen_ty_7::SDLK_POWER;
// pub const SDLK_KP_EQUALS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_EQUALS;
// pub const SDLK_F13: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F13;
// pub const SDLK_F14: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F14;
// pub const SDLK_F15: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F15;
// pub const SDLK_F16: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F16;
// pub const SDLK_F17: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F17;
// pub const SDLK_F18: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F18;
// pub const SDLK_F19: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F19;
// pub const SDLK_F20: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F20;
// pub const SDLK_F21: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F21;
// pub const SDLK_F22: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F22;
// pub const SDLK_F23: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F23;
// pub const SDLK_F24: _bindgen_ty_7 = _bindgen_ty_7::SDLK_F24;
// pub const SDLK_EXECUTE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_EXECUTE;
// pub const SDLK_HELP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_HELP;
// pub const SDLK_MENU: _bindgen_ty_7 = _bindgen_ty_7::SDLK_MENU;
// pub const SDLK_SELECT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_SELECT;
// pub const SDLK_STOP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_STOP;
// pub const SDLK_AGAIN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AGAIN;
// pub const SDLK_UNDO: _bindgen_ty_7 = _bindgen_ty_7::SDLK_UNDO;
// pub const SDLK_CUT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CUT;
// pub const SDLK_COPY: _bindgen_ty_7 = _bindgen_ty_7::SDLK_COPY;
// pub const SDLK_PASTE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PASTE;
// pub const SDLK_FIND: _bindgen_ty_7 = _bindgen_ty_7::SDLK_FIND;
// pub const SDLK_MUTE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_MUTE;
// pub const SDLK_VOLUMEUP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_VOLUMEUP;
// pub const SDLK_VOLUMEDOWN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_VOLUMEDOWN;
// pub const SDLK_KP_COMMA: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_COMMA;
// pub const SDLK_KP_EQUALSAS400: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_EQUALSAS400;
// pub const SDLK_ALTERASE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_ALTERASE;
// pub const SDLK_SYSREQ: _bindgen_ty_7 = _bindgen_ty_7::SDLK_SYSREQ;
// pub const SDLK_CANCEL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CANCEL;
// pub const SDLK_CLEAR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CLEAR;
// pub const SDLK_PRIOR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_PRIOR;
// pub const SDLK_RETURN2: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RETURN2;
// pub const SDLK_SEPARATOR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_SEPARATOR;
// pub const SDLK_OUT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_OUT;
// pub const SDLK_OPER: _bindgen_ty_7 = _bindgen_ty_7::SDLK_OPER;
// pub const SDLK_CLEARAGAIN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CLEARAGAIN;
// pub const SDLK_CRSEL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CRSEL;
// pub const SDLK_EXSEL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_EXSEL;
// pub const SDLK_KP_00: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_00;
// pub const SDLK_KP_000: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_000;
// pub const SDLK_THOUSANDSSEPARATOR: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_THOUSANDSSEPARATOR;
// pub const SDLK_DECIMALSEPARATOR: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_DECIMALSEPARATOR;
// pub const SDLK_CURRENCYUNIT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CURRENCYUNIT;
// pub const SDLK_CURRENCYSUBUNIT: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_CURRENCYSUBUNIT;
// pub const SDLK_KP_LEFTPAREN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_LEFTPAREN;
// pub const SDLK_KP_RIGHTPAREN: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_RIGHTPAREN;
// pub const SDLK_KP_LEFTBRACE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_LEFTBRACE;
// pub const SDLK_KP_RIGHTBRACE: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_RIGHTBRACE;
// pub const SDLK_KP_TAB: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_TAB;
// pub const SDLK_KP_BACKSPACE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_BACKSPACE;
// pub const SDLK_KP_A: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_A;
// pub const SDLK_KP_B: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_B;
// pub const SDLK_KP_C: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_C;
// pub const SDLK_KP_D: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_D;
// pub const SDLK_KP_E: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_E;
// pub const SDLK_KP_F: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_F;
// pub const SDLK_KP_XOR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_XOR;
// pub const SDLK_KP_POWER: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_POWER;
// pub const SDLK_KP_PERCENT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_PERCENT;
// pub const SDLK_KP_LESS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_LESS;
// pub const SDLK_KP_GREATER: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_GREATER;
// pub const SDLK_KP_AMPERSAND: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_AMPERSAND;
// pub const SDLK_KP_DBLAMPERSAND: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_DBLAMPERSAND;
// pub const SDLK_KP_VERTICALBAR: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_VERTICALBAR;
// pub const SDLK_KP_DBLVERTICALBAR: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_DBLVERTICALBAR;
// pub const SDLK_KP_COLON: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_COLON;
// pub const SDLK_KP_HASH: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_HASH;
// pub const SDLK_KP_SPACE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_SPACE;
// pub const SDLK_KP_AT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_AT;
// pub const SDLK_KP_EXCLAM: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_EXCLAM;
// pub const SDLK_KP_MEMSTORE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_MEMSTORE;
// pub const SDLK_KP_MEMRECALL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_MEMRECALL;
// pub const SDLK_KP_MEMCLEAR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_MEMCLEAR;
// pub const SDLK_KP_MEMADD: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_MEMADD;
// pub const SDLK_KP_MEMSUBTRACT: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_MEMSUBTRACT;
// pub const SDLK_KP_MEMMULTIPLY: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_MEMMULTIPLY;
// pub const SDLK_KP_MEMDIVIDE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_MEMDIVIDE;
// pub const SDLK_KP_PLUSMINUS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_PLUSMINUS;
// pub const SDLK_KP_CLEAR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_CLEAR;
// pub const SDLK_KP_CLEARENTRY: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_CLEARENTRY;
// pub const SDLK_KP_BINARY: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_BINARY;
// pub const SDLK_KP_OCTAL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_OCTAL;
// pub const SDLK_KP_DECIMAL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KP_DECIMAL;
// pub const SDLK_KP_HEXADECIMAL: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KP_HEXADECIMAL;
// pub const SDLK_LCTRL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_LCTRL;
// pub const SDLK_LSHIFT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_LSHIFT;
// pub const SDLK_LALT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_LALT;
// pub const SDLK_LGUI: _bindgen_ty_7 = _bindgen_ty_7::SDLK_LGUI;
// pub const SDLK_RCTRL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RCTRL;
// pub const SDLK_RSHIFT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RSHIFT;
// pub const SDLK_RALT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RALT;
// pub const SDLK_RGUI: _bindgen_ty_7 = _bindgen_ty_7::SDLK_RGUI;
// pub const SDLK_MODE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_MODE;
// pub const SDLK_AUDIONEXT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AUDIONEXT;
// pub const SDLK_AUDIOPREV: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AUDIOPREV;
// pub const SDLK_AUDIOSTOP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AUDIOSTOP;
// pub const SDLK_AUDIOPLAY: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AUDIOPLAY;
// pub const SDLK_AUDIOMUTE: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AUDIOMUTE;
// pub const SDLK_MEDIASELECT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_MEDIASELECT;
// pub const SDLK_WWW: _bindgen_ty_7 = _bindgen_ty_7::SDLK_WWW;
// pub const SDLK_MAIL: _bindgen_ty_7 = _bindgen_ty_7::SDLK_MAIL;
// pub const SDLK_CALCULATOR: _bindgen_ty_7 = _bindgen_ty_7::SDLK_CALCULATOR;
// pub const SDLK_COMPUTER: _bindgen_ty_7 = _bindgen_ty_7::SDLK_COMPUTER;
// pub const SDLK_AC_SEARCH: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AC_SEARCH;
// pub const SDLK_AC_HOME: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AC_HOME;
// pub const SDLK_AC_BACK: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AC_BACK;
// pub const SDLK_AC_FORWARD: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AC_FORWARD;
// pub const SDLK_AC_STOP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AC_STOP;
// pub const SDLK_AC_REFRESH: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AC_REFRESH;
// pub const SDLK_AC_BOOKMARKS: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AC_BOOKMARKS;
// pub const SDLK_BRIGHTNESSDOWN: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_BRIGHTNESSDOWN;
// pub const SDLK_BRIGHTNESSUP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_BRIGHTNESSUP;
// pub const SDLK_DISPLAYSWITCH: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_DISPLAYSWITCH;
// pub const SDLK_KBDILLUMTOGGLE: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_KBDILLUMTOGGLE;
// pub const SDLK_KBDILLUMDOWN: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KBDILLUMDOWN;
// pub const SDLK_KBDILLUMUP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_KBDILLUMUP;
// pub const SDLK_EJECT: _bindgen_ty_7 = _bindgen_ty_7::SDLK_EJECT;
// pub const SDLK_SLEEP: _bindgen_ty_7 = _bindgen_ty_7::SDLK_SLEEP;
// pub const SDLK_APP1: _bindgen_ty_7 = _bindgen_ty_7::SDLK_APP1;
// pub const SDLK_APP2: _bindgen_ty_7 = _bindgen_ty_7::SDLK_APP2;
// pub const SDLK_AUDIOREWIND: _bindgen_ty_7 = _bindgen_ty_7::SDLK_AUDIOREWIND;
// pub const SDLK_AUDIOFASTFORWARD: _bindgen_ty_7 =
//     _bindgen_ty_7::SDLK_AUDIOFASTFORWARD;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_7 {
    SDLK_UNKNOWN = 0,
    // SDLK_RETURN = 13,
    // SDLK_ESCAPE = 27,
    // SDLK_BACKSPACE = 8,
    // SDLK_TAB = 9,
    // SDLK_SPACE = 32,
    // SDLK_EXCLAIM = 33,
    // SDLK_QUOTEDBL = 34,
    // SDLK_HASH = 35,
    // SDLK_PERCENT = 37,
    // SDLK_DOLLAR = 36,
    // SDLK_AMPERSAND = 38,
    // SDLK_QUOTE = 39,
    // SDLK_LEFTPAREN = 40,
    // SDLK_RIGHTPAREN = 41,
    // SDLK_ASTERISK = 42,
    // SDLK_PLUS = 43,
    // SDLK_COMMA = 44,
    // SDLK_MINUS = 45,
    // SDLK_PERIOD = 46,
    // SDLK_SLASH = 47,
    // SDLK_0 = 48,
    // SDLK_1 = 49,
    // SDLK_2 = 50,
    // SDLK_3 = 51,
    // SDLK_4 = 52,
    // SDLK_5 = 53,
    // SDLK_6 = 54,
    // SDLK_7 = 55,
    // SDLK_8 = 56,
    // SDLK_9 = 57,
    // SDLK_COLON = 58,
    // SDLK_SEMICOLON = 59,
    // SDLK_LESS = 60,
    // SDLK_EQUALS = 61,
    // SDLK_GREATER = 62,
    // SDLK_QUESTION = 63,
    // SDLK_AT = 64,
    // SDLK_LEFTBRACKET = 91,
    // SDLK_BACKSLASH = 92,
    // SDLK_RIGHTBRACKET = 93,
    // SDLK_CARET = 94,
    // SDLK_UNDERSCORE = 95,
    // SDLK_BACKQUOTE = 96,
    // SDLK_a = 97,
    // SDLK_b = 98,
    // SDLK_c = 99,
    // SDLK_d = 100,
    // SDLK_e = 101,
    // SDLK_f = 102,
    // SDLK_g = 103,
    // SDLK_h = 104,
    // SDLK_i = 105,
    // SDLK_j = 106,
    // SDLK_k = 107,
    // SDLK_l = 108,
    // SDLK_m = 109,
    // SDLK_n = 110,
    // SDLK_o = 111,
    // SDLK_p = 112,
    // SDLK_q = 113,
    // SDLK_r = 114,
    // SDLK_s = 115,
    // SDLK_t = 116,
    // SDLK_u = 117,
    // SDLK_v = 118,
    // SDLK_w = 119,
    // SDLK_x = 120,
    // SDLK_y = 121,
    // SDLK_z = 122,
    // SDLK_CAPSLOCK = 1073741881,
    // SDLK_F1 = 1073741882,
    // SDLK_F2 = 1073741883,
    // SDLK_F3 = 1073741884,
    // SDLK_F4 = 1073741885,
    // SDLK_F5 = 1073741886,
    // SDLK_F6 = 1073741887,
    // SDLK_F7 = 1073741888,
    // SDLK_F8 = 1073741889,
    // SDLK_F9 = 1073741890,
    // SDLK_F10 = 1073741891,
    // SDLK_F11 = 1073741892,
    // SDLK_F12 = 1073741893,
    // SDLK_PRINTSCREEN = 1073741894,
    // SDLK_SCROLLLOCK = 1073741895,
    // SDLK_PAUSE = 1073741896,
    // SDLK_INSERT = 1073741897,
    // SDLK_HOME = 1073741898,
    // SDLK_PAGEUP = 1073741899,
    // SDLK_DELETE = 127,
    // SDLK_END = 1073741901,
    // SDLK_PAGEDOWN = 1073741902,
    // SDLK_RIGHT = 1073741903,
    // SDLK_LEFT = 1073741904,
    // SDLK_DOWN = 1073741905,
    // SDLK_UP = 1073741906,
    // SDLK_NUMLOCKCLEAR = 1073741907,
    // SDLK_KP_DIVIDE = 1073741908,
    // SDLK_KP_MULTIPLY = 1073741909,
    // SDLK_KP_MINUS = 1073741910,
    // SDLK_KP_PLUS = 1073741911,
    // SDLK_KP_ENTER = 1073741912,
    // SDLK_KP_1 = 1073741913,
    // SDLK_KP_2 = 1073741914,
    // SDLK_KP_3 = 1073741915,
    // SDLK_KP_4 = 1073741916,
    // SDLK_KP_5 = 1073741917,
    // SDLK_KP_6 = 1073741918,
    // SDLK_KP_7 = 1073741919,
    // SDLK_KP_8 = 1073741920,
    // SDLK_KP_9 = 1073741921,
    // SDLK_KP_0 = 1073741922,
    // SDLK_KP_PERIOD = 1073741923,
    // SDLK_APPLICATION = 1073741925,
    // SDLK_POWER = 1073741926,
    // SDLK_KP_EQUALS = 1073741927,
    // SDLK_F13 = 1073741928,
    // SDLK_F14 = 1073741929,
    // SDLK_F15 = 1073741930,
    // SDLK_F16 = 1073741931,
    // SDLK_F17 = 1073741932,
    // SDLK_F18 = 1073741933,
    // SDLK_F19 = 1073741934,
    // SDLK_F20 = 1073741935,
    // SDLK_F21 = 1073741936,
    // SDLK_F22 = 1073741937,
    // SDLK_F23 = 1073741938,
    // SDLK_F24 = 1073741939,
    // SDLK_EXECUTE = 1073741940,
    // SDLK_HELP = 1073741941,
    // SDLK_MENU = 1073741942,
    // SDLK_SELECT = 1073741943,
    // SDLK_STOP = 1073741944,
    // SDLK_AGAIN = 1073741945,
    // SDLK_UNDO = 1073741946,
    // SDLK_CUT = 1073741947,
    // SDLK_COPY = 1073741948,
    // SDLK_PASTE = 1073741949,
    // SDLK_FIND = 1073741950,
    // SDLK_MUTE = 1073741951,
    // SDLK_VOLUMEUP = 1073741952,
    // SDLK_VOLUMEDOWN = 1073741953,
    // SDLK_KP_COMMA = 1073741957,
    // SDLK_KP_EQUALSAS400 = 1073741958,
    // SDLK_ALTERASE = 1073741977,
    // SDLK_SYSREQ = 1073741978,
    // SDLK_CANCEL = 1073741979,
    // SDLK_CLEAR = 1073741980,
    // SDLK_PRIOR = 1073741981,
    // SDLK_RETURN2 = 1073741982,
    // SDLK_SEPARATOR = 1073741983,
    // SDLK_OUT = 1073741984,
    // SDLK_OPER = 1073741985,
    // SDLK_CLEARAGAIN = 1073741986,
    // SDLK_CRSEL = 1073741987,
    // SDLK_EXSEL = 1073741988,
    // SDLK_KP_00 = 1073742000,
    // SDLK_KP_000 = 1073742001,
    // SDLK_THOUSANDSSEPARATOR = 1073742002,
    // SDLK_DECIMALSEPARATOR = 1073742003,
    // SDLK_CURRENCYUNIT = 1073742004,
    // SDLK_CURRENCYSUBUNIT = 1073742005,
    // SDLK_KP_LEFTPAREN = 1073742006,
    // SDLK_KP_RIGHTPAREN = 1073742007,
    // SDLK_KP_LEFTBRACE = 1073742008,
    // SDLK_KP_RIGHTBRACE = 1073742009,
    // SDLK_KP_TAB = 1073742010,
    // SDLK_KP_BACKSPACE = 1073742011,
    // SDLK_KP_A = 1073742012,
    // SDLK_KP_B = 1073742013,
    // SDLK_KP_C = 1073742014,
    // SDLK_KP_D = 1073742015,
    // SDLK_KP_E = 1073742016,
    // SDLK_KP_F = 1073742017,
    // SDLK_KP_XOR = 1073742018,
    // SDLK_KP_POWER = 1073742019,
    // SDLK_KP_PERCENT = 1073742020,
    // SDLK_KP_LESS = 1073742021,
    // SDLK_KP_GREATER = 1073742022,
    // SDLK_KP_AMPERSAND = 1073742023,
    // SDLK_KP_DBLAMPERSAND = 1073742024,
    // SDLK_KP_VERTICALBAR = 1073742025,
    // SDLK_KP_DBLVERTICALBAR = 1073742026,
    // SDLK_KP_COLON = 1073742027,
    // SDLK_KP_HASH = 1073742028,
    // SDLK_KP_SPACE = 1073742029,
    // SDLK_KP_AT = 1073742030,
    // SDLK_KP_EXCLAM = 1073742031,
    // SDLK_KP_MEMSTORE = 1073742032,
    // SDLK_KP_MEMRECALL = 1073742033,
    // SDLK_KP_MEMCLEAR = 1073742034,
    // SDLK_KP_MEMADD = 1073742035,
    // SDLK_KP_MEMSUBTRACT = 1073742036,
    // SDLK_KP_MEMMULTIPLY = 1073742037,
    // SDLK_KP_MEMDIVIDE = 1073742038,
    // SDLK_KP_PLUSMINUS = 1073742039,
    // SDLK_KP_CLEAR = 1073742040,
    // SDLK_KP_CLEARENTRY = 1073742041,
    // SDLK_KP_BINARY = 1073742042,
    // SDLK_KP_OCTAL = 1073742043,
    // SDLK_KP_DECIMAL = 1073742044,
    // SDLK_KP_HEXADECIMAL = 1073742045,
    // SDLK_LCTRL = 1073742048,
    // SDLK_LSHIFT = 1073742049,
    // SDLK_LALT = 1073742050,
    // SDLK_LGUI = 1073742051,
    // SDLK_RCTRL = 1073742052,
    // SDLK_RSHIFT = 1073742053,
    // SDLK_RALT = 1073742054,
    // SDLK_RGUI = 1073742055,
    // SDLK_MODE = 1073742081,
    // SDLK_AUDIONEXT = 1073742082,
    // SDLK_AUDIOPREV = 1073742083,
    // SDLK_AUDIOSTOP = 1073742084,
    // SDLK_AUDIOPLAY = 1073742085,
    // SDLK_AUDIOMUTE = 1073742086,
    // SDLK_MEDIASELECT = 1073742087,
    // SDLK_WWW = 1073742088,
    // SDLK_MAIL = 1073742089,
    // SDLK_CALCULATOR = 1073742090,
    // SDLK_COMPUTER = 1073742091,
    // SDLK_AC_SEARCH = 1073742092,
    // SDLK_AC_HOME = 1073742093,
    // SDLK_AC_BACK = 1073742094,
    // SDLK_AC_FORWARD = 1073742095,
    // SDLK_AC_STOP = 1073742096,
    // SDLK_AC_REFRESH = 1073742097,
    // SDLK_AC_BOOKMARKS = 1073742098,
    // SDLK_BRIGHTNESSDOWN = 1073742099,
    // SDLK_BRIGHTNESSUP = 1073742100,
    // SDLK_DISPLAYSWITCH = 1073742101,
    // SDLK_KBDILLUMTOGGLE = 1073742102,
    // SDLK_KBDILLUMDOWN = 1073742103,
    // SDLK_KBDILLUMUP = 1073742104,
    // SDLK_EJECT = 1073742105,
    // SDLK_SLEEP = 1073742106,
    // SDLK_APP1 = 1073742107,
    // SDLK_APP2 = 1073742108,
    // SDLK_AUDIOREWIND = 1073742109,
    // SDLK_AUDIOFASTFORWARD = 1073742110,
}
#[repr(u32)]
/// \brief Enumeration of valid key mods (possibly OR'd together).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SDL_Keymod {
    KMOD_NONE = 0,
    // KMOD_LSHIFT = 1,
    // KMOD_RSHIFT = 2,
    // KMOD_LCTRL = 64,
    // KMOD_RCTRL = 128,
    // KMOD_LALT = 256,
    // KMOD_RALT = 512,
    // KMOD_LGUI = 1024,
    // KMOD_RGUI = 2048,
    // KMOD_NUM = 4096,
    // KMOD_CAPS = 8192,
    // KMOD_MODE = 16384,
    // KMOD_RESERVED = 32768,
}

#[repr(u32)]
/// \brief The types of events that can be delivered.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SDL_EventType {
    SDL_FIRSTEVENT = 0,
    // SDL_QUIT = 256,
    // SDL_APP_TERMINATING = 257,
    // SDL_APP_LOWMEMORY = 258,
    // SDL_APP_WILLENTERBACKGROUND = 259,
    // SDL_APP_DIDENTERBACKGROUND = 260,
    // SDL_APP_WILLENTERFOREGROUND = 261,
    // SDL_APP_DIDENTERFOREGROUND = 262,
    // SDL_WINDOWEVENT = 512,
    // SDL_SYSWMEVENT = 513,
    // SDL_KEYDOWN = 768,
    // SDL_KEYUP = 769,
    // SDL_TEXTEDITING = 770,
    // SDL_TEXTINPUT = 771,
    // SDL_KEYMAPCHANGED = 772,
    // SDL_MOUSEMOTION = 1024,
    // SDL_MOUSEBUTTONDOWN = 1025,
    // SDL_MOUSEBUTTONUP = 1026,
    // SDL_MOUSEWHEEL = 1027,
    // SDL_JOYAXISMOTION = 1536,
    // SDL_JOYBALLMOTION = 1537,
    // SDL_JOYHATMOTION = 1538,
    // SDL_JOYBUTTONDOWN = 1539,
    // SDL_JOYBUTTONUP = 1540,
    // SDL_JOYDEVICEADDED = 1541,
    // SDL_JOYDEVICEREMOVED = 1542,
    // SDL_CONTROLLERAXISMOTION = 1616,
    // SDL_CONTROLLERBUTTONDOWN = 1617,
    // SDL_CONTROLLERBUTTONUP = 1618,
    // SDL_CONTROLLERDEVICEADDED = 1619,
    // SDL_CONTROLLERDEVICEREMOVED = 1620,
    // SDL_CONTROLLERDEVICEREMAPPED = 1621,
    // SDL_FINGERDOWN = 1792,
    // SDL_FINGERUP = 1793,
    // SDL_FINGERMOTION = 1794,
    // SDL_DOLLARGESTURE = 2048,
    // SDL_DOLLARRECORD = 2049,
    // SDL_MULTIGESTURE = 2050,
    // SDL_CLIPBOARDUPDATE = 2304,
    // SDL_DROPFILE = 4096,
    // SDL_DROPTEXT = 4097,
    // SDL_DROPBEGIN = 4098,
    // SDL_DROPCOMPLETE = 4099,
    // SDL_AUDIODEVICEADDED = 4352,
    // SDL_AUDIODEVICEREMOVED = 4353,
    // SDL_RENDER_TARGETS_RESET = 8192,
    // SDL_RENDER_DEVICE_RESET = 8193,
    // SDL_USEREVENT = 32768,
    // SDL_LASTEVENT = 65535,
}

pub const SDL_PIXELFORMAT_UNKNOWN: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_UNKNOWN;
pub const SDL_PIXELFORMAT_INDEX1LSB: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_INDEX1LSB;
pub const SDL_PIXELFORMAT_INDEX1MSB: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_INDEX1MSB;
pub const SDL_PIXELFORMAT_INDEX4LSB: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_INDEX4LSB;
pub const SDL_PIXELFORMAT_INDEX4MSB: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_INDEX4MSB;
pub const SDL_PIXELFORMAT_INDEX8: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_INDEX8;
pub const SDL_PIXELFORMAT_RGB332: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGB332;
pub const SDL_PIXELFORMAT_RGB444: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGB444;
pub const SDL_PIXELFORMAT_RGB555: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGB555;
pub const SDL_PIXELFORMAT_BGR555: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGR555;
pub const SDL_PIXELFORMAT_ARGB4444: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ARGB4444;
pub const SDL_PIXELFORMAT_RGBA4444: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGBA4444;
pub const SDL_PIXELFORMAT_ABGR4444: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ABGR4444;
pub const SDL_PIXELFORMAT_BGRA4444: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGRA4444;
pub const SDL_PIXELFORMAT_ARGB1555: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ARGB1555;
pub const SDL_PIXELFORMAT_RGBA5551: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGBA5551;
pub const SDL_PIXELFORMAT_ABGR1555: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ABGR1555;
pub const SDL_PIXELFORMAT_BGRA5551: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGRA5551;
pub const SDL_PIXELFORMAT_RGB565: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGB565;
pub const SDL_PIXELFORMAT_BGR565: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGR565;
pub const SDL_PIXELFORMAT_RGB24: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGB24;
pub const SDL_PIXELFORMAT_BGR24: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGR24;
pub const SDL_PIXELFORMAT_RGB888: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGB888;
pub const SDL_PIXELFORMAT_RGBX8888: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGBX8888;
pub const SDL_PIXELFORMAT_BGR888: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGR888;
pub const SDL_PIXELFORMAT_BGRX8888: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGRX8888;
pub const SDL_PIXELFORMAT_ARGB8888: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ARGB8888;
pub const SDL_PIXELFORMAT_RGBA8888: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGBA8888;
pub const SDL_PIXELFORMAT_ABGR8888: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ABGR8888;
pub const SDL_PIXELFORMAT_BGRA8888: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGRA8888;
pub const SDL_PIXELFORMAT_ARGB2101010: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ARGB2101010;
pub const SDL_PIXELFORMAT_RGBA32: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ABGR8888;
pub const SDL_PIXELFORMAT_ARGB32: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_BGRA8888;
pub const SDL_PIXELFORMAT_BGRA32: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_ARGB8888;
pub const SDL_PIXELFORMAT_ABGR32: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_RGBA8888;
pub const SDL_PIXELFORMAT_YV12: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_YV12;
pub const SDL_PIXELFORMAT_IYUV: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_IYUV;
pub const SDL_PIXELFORMAT_YUY2: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_YUY2;
pub const SDL_PIXELFORMAT_UYVY: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_UYVY;
pub const SDL_PIXELFORMAT_YVYU: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_YVYU;
pub const SDL_PIXELFORMAT_NV12: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_NV12;
pub const SDL_PIXELFORMAT_NV21: _bindgen_ty_6 =
    _bindgen_ty_6::SDL_PIXELFORMAT_NV21;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_6 {
    SDL_PIXELFORMAT_UNKNOWN = 0,
    SDL_PIXELFORMAT_INDEX1LSB = 286261504,
    SDL_PIXELFORMAT_INDEX1MSB = 287310080,
    SDL_PIXELFORMAT_INDEX4LSB = 303039488,
    SDL_PIXELFORMAT_INDEX4MSB = 304088064,
    SDL_PIXELFORMAT_INDEX8 = 318769153,
    SDL_PIXELFORMAT_RGB332 = 336660481,
    SDL_PIXELFORMAT_RGB444 = 353504258,
    SDL_PIXELFORMAT_RGB555 = 353570562,
    SDL_PIXELFORMAT_BGR555 = 357764866,
    SDL_PIXELFORMAT_ARGB4444 = 355602434,
    SDL_PIXELFORMAT_RGBA4444 = 356651010,
    SDL_PIXELFORMAT_ABGR4444 = 359796738,
    SDL_PIXELFORMAT_BGRA4444 = 360845314,
    SDL_PIXELFORMAT_ARGB1555 = 355667970,
    SDL_PIXELFORMAT_RGBA5551 = 356782082,
    SDL_PIXELFORMAT_ABGR1555 = 359862274,
    SDL_PIXELFORMAT_BGRA5551 = 360976386,
    SDL_PIXELFORMAT_RGB565 = 353701890,
    SDL_PIXELFORMAT_BGR565 = 357896194,
    SDL_PIXELFORMAT_RGB24 = 386930691,
    SDL_PIXELFORMAT_BGR24 = 390076419,
    SDL_PIXELFORMAT_RGB888 = 370546692,
    SDL_PIXELFORMAT_RGBX8888 = 371595268,
    SDL_PIXELFORMAT_BGR888 = 374740996,
    SDL_PIXELFORMAT_BGRX8888 = 375789572,
    SDL_PIXELFORMAT_ARGB8888 = 372645892,
    SDL_PIXELFORMAT_RGBA8888 = 373694468,
    SDL_PIXELFORMAT_ABGR8888 = 376840196,
    SDL_PIXELFORMAT_BGRA8888 = 377888772,
    SDL_PIXELFORMAT_ARGB2101010 = 372711428,
    SDL_PIXELFORMAT_YV12 = 842094169,
    SDL_PIXELFORMAT_IYUV = 1448433993,
    SDL_PIXELFORMAT_YUY2 = 844715353,
    SDL_PIXELFORMAT_UYVY = 1498831189,
    SDL_PIXELFORMAT_YVYU = 1431918169,
    SDL_PIXELFORMAT_NV12 = 842094158,
    SDL_PIXELFORMAT_NV21 = 825382478,
}