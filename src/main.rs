#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unsafe_op_in_unsafe_fn)]

use std::ffi::{CStr, CString};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// SAFETY: The function declarations given below are in
// line with the header files of `my_c_library`.
#[link(name = "my_func")]
unsafe extern "C" {
    fn my_func(x: i32, y: i32) -> i32;
    fn qt_show_window() -> i32;
}

fn main() {
    unsafe {
        println!("Hello, world!, {}", my_func(1, 2));
        if let Err(err) = list_block_devices() {
            eprintln!("Failed to list block devices: {err}");
        }
        let rc = qt_show_window();
        println!("Qt exited with code {rc}");
    }
}

/// Enumerate block devices via libudev and print their device nodes.
unsafe fn list_block_devices() -> Result<(), String> {
    let udev = udev_new();
    if udev.is_null() {
        return Err("udev_new returned null".into());
    }

    let en = udev_enumerate_new(udev);
    if en.is_null() {
        udev_unref(udev);
        return Err("udev_enumerate_new returned null".into());
    }

    let subsystem = CString::new("block").unwrap();
    if udev_enumerate_add_match_subsystem(en, subsystem.as_ptr()) != 0 {
        udev_enumerate_unref(en);
        udev_unref(udev);
        return Err("udev_enumerate_add_match_subsystem failed".into());
    }

    if udev_enumerate_scan_devices(en) != 0 {
        udev_enumerate_unref(en);
        udev_unref(udev);
        return Err("udev_enumerate_scan_devices failed".into());
    }

    let mut entry = udev_enumerate_get_list_entry(en);
    while !entry.is_null() {
        let name_ptr = udev_list_entry_get_name(entry);
        if !name_ptr.is_null() {
            let dev = udev_device_new_from_syspath(udev, name_ptr);
            if !dev.is_null() {
                let devnode_ptr = udev_device_get_devnode(dev);
                if !devnode_ptr.is_null() {
                    let devnode = CStr::from_ptr(devnode_ptr).to_string_lossy();
                    println!("{devnode}");
                }
                udev_device_unref(dev);
            }
        }
        entry = udev_list_entry_get_next(entry);
    }

    udev_enumerate_unref(en);
    udev_unref(udev);
    Ok(())
}
