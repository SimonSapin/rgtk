// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! General — Library initialization and miscellaneous functions

use std::ptr;
use gdk::ffi;
use std::ffi::CString;

pub fn init() {
    unsafe { ffi::gdk_init(ptr::null_mut(), ptr::null_mut()) }
}

/*pub fn init_check(argc: *mut c_int, argv: *mut *mut *mut c_char) -> bool {
    unsafe { gtk::ffi::to_bool(ffi::gdk_init_check(argc, argv)) }
}

pub fn parse_args(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { ffi::gdk_parse_args(argc, argv) }
}*/

pub fn get_display_arg_name() -> Option<String> {
    let tmp = unsafe { ffi::gdk_get_display_arg_name() };

    if tmp.is_null() {
        None
    } else {
        unsafe { Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp)).to_string()) }
    }
}

pub fn notify_startup_complete() {
    unsafe { ffi::gdk_notify_startup_complete() }
}

pub fn notify_startup_complete_with_id(startup_id: &str) {
    unsafe {
        let c_str = CString::from_slice(startup_id.as_bytes());

        ffi::gdk_notify_startup_complete_with_id(c_str.as_ptr())
    }
}

pub fn set_allowed_backends(backends: &str) {
    unsafe {
        let c_str = CString::from_slice(backends.as_bytes());

        ffi::gdk_set_allowed_backends(c_str.as_ptr())
    }
}

pub fn get_program_class() -> Option<String> {
    let tmp = unsafe { ffi::gdk_get_program_class() };

    if tmp.is_null() {
        None
    } else {
        unsafe { Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp)).to_string()) }
    }
}

pub fn set_program_class(program_class: &str) {
    unsafe {
        let c_str = CString::from_slice(program_class.as_bytes());

        ffi::gdk_set_program_class(c_str.as_ptr())
    }
}

pub fn flush() {
    unsafe { ffi::gdk_flush() }
}

pub fn screen_width() -> i32 {
    unsafe { ffi::gdk_screen_width() }
}

pub fn screen_height() -> i32 {
    unsafe { ffi::gdk_screen_height() }
}

pub fn screen_width_mm() -> i32 {
    unsafe { ffi::gdk_screen_width_mm() }
}

pub fn screen_height_mm() -> i32 {
    unsafe { ffi::gdk_screen_height_mm() }
}

pub fn beep() {
    unsafe { ffi::gdk_flush() }
}

pub fn error_trap_push() {
    unsafe { ffi::gdk_error_trap_push() }
}

pub fn error_trap_pop() {
    unsafe { ffi::gdk_error_trap_pop() }
}

pub fn error_trap_pop_ignored() {
    unsafe { ffi::gdk_error_trap_pop_ignored() }
}