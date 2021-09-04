#![allow(non_upper_case_globals, unused_variables, improper_ctypes)]
extern crate libc;

#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "CoreGraphics", kind = "framework")]

pub type CFRunLoopRef = *const libc::c_void;
pub type CGEventType = libc::c_uint;
pub type CGEventTapProxy = *const libc::c_void;
pub type CGEventRef = *const libc::c_void;
pub type CFMachPortRef = *const libc::c_void;
pub type CGEventMask = u64;
pub type CFRunLoopSourceRef = *const libc::c_void;
pub type CFStringRef = *const libc::c_void;

pub struct __CFBoolean(libc::c_void);

pub type CFBooleanRef = *const __CFBoolean;
pub static kCGEventKeyDown: CGEventType = 10;
pub static kCGEventKeyUp: CGEventType = 11;
pub static kCGEventFlagsChanged: CGEventType = 12;
use std::ptr;

pub type CGEventTapCallBack = extern fn(CGEventTapProxy, CGEventType,
                                        CGEventRef, *const libc::c_void)
    -> CGEventRef;

#[repr(u32)]
#[non_exhaustive]
pub enum EventField {
    KeyboardEventKeycode = 9
}

extern {
    pub static kCFBooleanTrue: CFBooleanRef;

    pub static kCFRunLoopDefaultMode: CFStringRef;
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CGEventTapCreate(tap: u32, place: u32,
                            options: u32, events: CGEventMask,
                            callback: CGEventTapCallBack,
                            user_info: *const libc::c_void ) -> CFMachPortRef;

    pub fn CGEventGetIntegerValueField(event: CGEventRef, field: EventField) -> i64;
    pub fn CFMachPortCreateRunLoopSource(allocator: *const libc::c_void,
                                         port: CFMachPortRef,
                                         order: u64)
        -> CFRunLoopSourceRef;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef,
                              mode: CFStringRef);
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: CFBooleanRef);
    pub fn CFRunLoopRun();
}

extern fn event_tap_callback(_: CGEventTapProxy, event_type: CGEventType, event: CGEventRef, arg: *const libc::c_void) -> CGEventRef {

    let keycode = unsafe{CGEventGetIntegerValueField(event, EventField::KeyboardEventKeycode)} as u16;


    println!("event_type: {:?}, key: {:?}", event_type, keycode);
    event
}

fn main() {
    let keydown = 1 << kCGEventKeyDown;
    let keyup = 1 << kCGEventKeyUp;
    let modifier = 1 << kCGEventFlagsChanged;


    unsafe {
        let tap = CGEventTapCreate(0, 0, 0, modifier | keydown | keyup, event_tap_callback, ptr::null());

        if tap.is_null() {
            panic!("Not enough priviledges");
        }
        let source = CFMachPortCreateRunLoopSource(
            ::std::ptr::null(), tap, 0
            );

        let run_loop = CFRunLoopGetCurrent();
        CFRunLoopAddSource(run_loop, source, kCFRunLoopDefaultMode);
        CGEventTapEnable(tap, kCFBooleanTrue);
        CFRunLoopRun();
    }
}
