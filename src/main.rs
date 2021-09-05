#![allow(non_upper_case_globals, unused_variables, improper_ctypes)]
extern crate libc;

#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "CoreGraphics", kind = "framework")]

use std::ptr;
use std::process::Command;

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

pub type CGEventTapCallBack = extern fn(CGEventTapProxy, CGEventType,
                                        CGEventRef, *const libc::c_void)
    -> CGEventRef;

#[repr(u32)]
#[non_exhaustive]
pub enum EventField {
    KeyboardEventKeycode = 9
}

pub enum FlagType {
    None = 0,
    Control = 1 << 0,
    Option  = 1 << 1,
    Command = 1 << 2,
    Shift = 1 << 3,
    Fn = 1 << 4,
    Hyper = 1 << 5,
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

static mut flags:u16 = 0;

extern fn event_tap_callback(_: CGEventTapProxy, event_type: CGEventType, event: CGEventRef, arg: *const libc::c_void) -> CGEventRef {
    let keycode = unsafe{CGEventGetIntegerValueField(event, EventField::KeyboardEventKeycode)} as u16;

    let flag = flag_from_key(keycode);
    let mut should_capture_event = false;

    if ! matches!(flag, FlagType::None) {
        set_flags(flag, event_type);
        should_capture_event = event_type == kCGEventKeyDown; // If the key is not a flag key but it's detected as a flag then the event should be captured
    } else {
        if event_type == kCGEventKeyDown {
            should_capture_event = find_and_exec_key(keycode);
        }
    }

    unsafe{ println!("event_type: {:?}, key: {:#X}, flag: {:07b}", event_type, keycode, flags as u16) };

    if should_capture_event { ptr::null() } else { event }
}

fn find_and_exec_key(key: u16) -> bool{
    unsafe {
        if flags == FlagType::Hyper as u16 | FlagType::Shift as u16 && key == 0x25 {
            run_command("yabai -m window --swap east");
            true
        } else if flags == FlagType::Hyper as u16 | FlagType::Shift as u16 && key == 0x04 {
            run_command("yabai -m window --swap west");
            true
        } else if flags == FlagType::Hyper as u16 && key == 0x18 {
            run_command("yabai -m space --balance");
            true
        } else if flags == FlagType::Hyper as u16 && key == 0x03 {
            run_command("yabai -m window --toggle native-fullscreen");
            true
        } else if flags == FlagType::Hyper as u16 && key == 0x12 {
            run_command("yabai -m display --focus 1");
            true
        } else if flags == FlagType::Hyper as u16 && key == 0x13 {
            run_command("yabai -m display --focus 2");
            true
        } else if flags == FlagType::Hyper as u16 && key == 0x14 {
            run_command("yabai -m display --focus 3");
            true
        } else {
            false
        }
    }
}

fn run_command(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to run");
}

fn set_flags(flag: FlagType, event_type: CGEventType) {
    let mask = flag as u16;

    unsafe{
        if event_type == kCGEventFlagsChanged {
            flags = flags ^ mask
        } else if event_type == kCGEventKeyDown {
            flags = flags | mask
        } else if event_type == kCGEventKeyUp {
            flags = flags ^ mask
        } 
    }
}

fn flag_from_key(keycode:u16) -> FlagType{
    match keycode {
        0x3B => FlagType::Control,
        0x3A => FlagType::Option,
        0x3D => FlagType::Option, // Right
        0x36 => FlagType::Command, // Right
        0x37 => FlagType::Command,
        0x38 => FlagType::Shift,
        0x3C => FlagType::Shift, // Right
        0x3F => FlagType::Fn,
        0x4F => FlagType::Hyper,
        _ => FlagType::None
    }
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
            ptr::null(), tap, 0
            );

        let run_loop = CFRunLoopGetCurrent();
        CFRunLoopAddSource(run_loop, source, kCFRunLoopDefaultMode);
        CGEventTapEnable(tap, kCFBooleanTrue);
        CFRunLoopRun();
    }
}
