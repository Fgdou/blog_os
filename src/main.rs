#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world{}", "!");

    blog_os::init();

    #[cfg(test)]
    test_main();

    let x = Box::new(1);

    println!("It did not crash!");

    blog_os::hlt_loop()
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
