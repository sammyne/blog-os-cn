#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    blog_os::init();

    // Note: The actual address might be different for you. Use the address that
    // your page fault handler reports.
    let ptr = 0x20399c as *mut u32;

    // read from a code page
    unsafe {
        let x = *ptr;
    }
    println!("read worked");

    // write to a code page
    unsafe {
        *ptr = 42;
    }
    println!("write worked");

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
