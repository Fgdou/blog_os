#![no_std]
#![no_main]

use core::panic::PanicInfo;

use blog_os::{exit_qemu, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(blog_os::QemuExitCode::Success);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();

    loop {}
}

fn should_fail() {
    serial_println!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
