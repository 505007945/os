#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PnaicInfo) -> !{
    loop{}
}
