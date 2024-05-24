#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case, unused)]

mod ul_sys;
use libc::*;
use ul_sys::*;

use std::{rc::Rc, sync::OnceLock};

#[repr(transparent)]
struct Render(*mut ul_sys::C_Renderer);
unsafe impl Send for Render {}
unsafe impl Sync for Render {}
static RENDERER: OnceLock<Render> = OnceLock::new();

#[no_mangle]
pub extern "C" fn wkeInit() {
    println!("{}",123123123);
    unsafe {
        
        ulEnablePlatformFontLoader();
        let file_system= ulCreateString(c"./".as_ptr());
        ulEnablePlatformFileSystem(file_system);

        let log_path = ulCreateString(c"./ultralight.log".as_ptr());
        ulEnableDefaultLogger(log_path);
        ulDestroyString(log_path);

        let config = ulCreateConfig();
        println!("{:?}", config);
        let render = ulCreateRenderer(config);
        println!("{:?}", render);
        ulDestroyConfig(config);
        ulDestroyString(file_system);
        RENDERER.set(Render(render));
    }
}

#[no_mangle]
pub extern "C" fn wkeCreateWebView() -> *mut c_void {
    let renderer = RENDERER.get().unwrap();
    unsafe {
        let config = ulCreateViewConfig();
        ulViewConfigSetIsAccelerated(config, false);
        let view = ulCreateView(renderer.0, 400, 400, config, 0 as _);
        ulDestroyViewConfig(config);
        view as _
    }
}

#[no_mangle]
pub extern "C" fn wkeLoadURL(view: *mut c_void, url: *const c_char) {
    unsafe {
        let url = ulCreateString(url);
        ulViewLoadURL(view as _, url);
        ulDestroyString(url);
    }
}

#[no_mangle]
pub extern "C" fn wkeSetZoomFactor(view: *mut c_void, factor: c_float) {
    unsafe { ulViewSetDeviceScale(view as _, factor as _) }
}

#[no_mangle]
pub extern "C" fn wkeFocus(view: *mut c_void) {
    unsafe { ulViewFocus(view as _) }
}

#[no_mangle]
pub extern "C" fn wkePaint(view: *mut c_void, bits: *mut c_void, pitch: c_int) {
    unsafe {
        let render = RENDERER.get().unwrap().0;
        ulUpdate(render);
        ulRender(render);
        let surface = ulViewGetSurface(view as _);
        let bitmap = ulBitmapSurfaceGetBitmap(surface);
        let length = ulBitmapGetSize(bitmap);
        let pixels = ulBitmapLockPixels(bitmap);
        memcpy(bits, pixels, length);
        ulBitmapUnlockPixels(bitmap);
    }
}

#[no_mangle]
pub extern "C" fn wkeResize(view: *mut c_void, w: c_int, h: c_int) {
    unsafe {
        ulViewResize(view as _, w as _, h as _);
    }
}

const WKE_LBUTTON: wkeMouseFlags = 1;
const WKE_RBUTTON: wkeMouseFlags = 2;
const WKE_SHIFT: wkeMouseFlags = 4;
const WKE_CONTROL: wkeMouseFlags = 8;
const WKE_MBUTTON: wkeMouseFlags = 16;
type wkeMouseFlags = c_uint;
const WKE_EXTENDED: wkeKeyFlags = 256;
const WKE_REPEAT: wkeKeyFlags = 16384;
type wkeKeyFlags = c_uint;
const WKE_MSG_MOUSEMOVE: wkeMouseMsg = 512;
const WKE_MSG_LBUTTONDOWN: wkeMouseMsg = 513;
const WKE_MSG_LBUTTONUP: wkeMouseMsg = 514;
const WKE_MSG_LBUTTONDBLCLK: wkeMouseMsg = 515;
const WKE_MSG_RBUTTONDOWN: wkeMouseMsg = 516;
const WKE_MSG_RBUTTONUP: wkeMouseMsg = 517;
const WKE_MSG_RBUTTONDBLCLK: wkeMouseMsg = 518;
const WKE_MSG_MBUTTONDOWN: wkeMouseMsg = 519;
const WKE_MSG_MBUTTONUP: wkeMouseMsg = 520;
const WKE_MSG_MBUTTONDBLCLK: wkeMouseMsg = 521;
const WKE_MSG_MOUSEWHEEL: wkeMouseMsg = 522;
type wkeMouseMsg = c_uint;

#[no_mangle]
pub extern "C" fn wkeMouseEvent(
    view: *mut c_void,
    message: c_uint,
    x: c_int,
    y: c_int,
    flags: c_uint,
) {

    let type_ = match message {
        WKE_MSG_MOUSEMOVE => kMouseEventType_MouseMoved,
        WKE_MSG_LBUTTONDOWN | WKE_MSG_RBUTTONDOWN | WKE_MSG_MBUTTONDOWN => kMouseEventType_MouseDown,
        WKE_MSG_LBUTTONUP| WKE_MSG_RBUTTONUP |WKE_MSG_MBUTTONUP=> kMouseEventType_MouseUp,
        _ => return
    };
    let button = match message {
        WKE_MSG_LBUTTONDOWN | WKE_MSG_LBUTTONUP | WKE_MSG_LBUTTONDBLCLK => kMouseButton_Left,
        WKE_MSG_RBUTTONDOWN | WKE_MSG_RBUTTONUP | WKE_MSG_RBUTTONDBLCLK => kMouseButton_Right,
        WKE_MSG_MBUTTONDOWN | WKE_MSG_MBUTTONUP | WKE_MSG_MBUTTONDBLCLK => kMouseButton_Middle,
        _ => return
    };
    unsafe {let mouse_event = ulCreateMouseEvent(type_, x, y, button);
    ulViewFireMouseEvent(view as _, mouse_event)}
}
