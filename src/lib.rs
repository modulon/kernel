//!-----------------------------------------------------------------------------------------------
//!`src/lib.rs`
//!
//!Main rust file, declares all other modules to be included and implements kmain().
//!-----------------------------------------------------------------------------------------------

#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]

#![no_std]

//Multiboot crate for retrieving boot information
extern crate multiboot2;

//Spinlock crate
extern crate spin;

//Bitflags crate
#[macro_use]
extern crate bitflags;

//Architecture specific code
pub mod arch;

//System functions
pub mod system;

//Standard support library
pub mod support;

//Version information
pub const VERSION_MAJOR: u16 = 0;
pub const VERSION_MID: u16 = 1;
pub const VERSION_MINOR: u16 = 6;
pub const VERSION_COMMIT: u16 = 4;

//Use x86_64 architecture components
pub use arch::x86_64::*;

//use arch::x86_64::memory::alloc::FrameAlloc;

use arch::x86_64::io::display::*;

///Main kernel entry point, called by assembly
#[no_mangle]
pub extern fn kmain(mb_info_address: usize) {
	//Create terminal for logging
	terminal::TERM.lock().clear();

	//Display version information
	terminal::TERM.lock().set_color(common_color::GREEN);
	print!("Modulon");
	terminal::TERM.lock().set_color(common_color::WHITE);
	print!(" v{}.{}.{}.{} Buttered Potato\n\n", VERSION_MAJOR, VERSION_MID,
		VERSION_MINOR, VERSION_COMMIT);

	//Initialize frame allocation
	print!(" >> Initializing memory management\n");
	let mut alloc = memory::init_area_frame_alloc(mb_info_address);
	
	terminal::TERM.lock().scroll();
	
	memory::page::test::test();
	
	//Initialization complete
}
