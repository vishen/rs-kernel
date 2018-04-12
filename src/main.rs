#![feature(lang_items)] // required for defining the panic handler
#![feature(const_fn)] // allow declaring functions as const
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;


// Writing an executable without stdlib -
//  https://doc.rust-lang.org/unstable-book/language-features/lang-items.html#writing-an-executable-without-stdlib
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32
) -> ! {
    loop {}
}



// On linux the default entry point is called '_start'. The linker just looks for a function with
// that name and sets this function as entry point for the executable.
// The '!' return type means that the function is not allowed to ever return
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!\nthis is a new line\nas is this");
    println!("Another println! with only one argument: {}", "THIS IS AN ARGUMENT");
    println!("We can use ints={} and floats={}", 42, 1.0/3.0);
    loop {}
}

