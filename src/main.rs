#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!"; // Pretty sure this varible stores a byte sequence (because its a pointer to a u8 list)
                                       // and the b at the "Hello World!" string.
#[no_mangle]

// This part of the code uses the C naming conventions, which is often expected for system programs (x86-64 assembly uses _start
// as a entry point in the code, so the same is used here.) This code is writing the bytes corresponding to the characters in the
// words "Hello World!" to the text buffer, the vga_buffer varible is simply a pointer to the vga buffer in memory, starting at 
// address 0xb8000. btw .offset is a method that allows you to move a pointer by a set amount of elements.

pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // Buffer is located at 0xb8000 mem address. Cast turns it into a raw pointer.

    for (i, &byte) in HELLO.iter().enumerate() { // Iterating over the  bytes in HELLO. emumerate method used to get   
        unsafe {                                            // a running varible (i).
            *vga_buffer.offset(i as isize * 2) = byte; // offset method used to write the string byte and corresponding color byte.
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // 0xb is the color byte. Light cyan.
        }
    }

    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}