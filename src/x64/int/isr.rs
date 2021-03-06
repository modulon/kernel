use io::pio::*;
use env;

#[no_mangle]
pub fn sys() {
    print!("System Call\n");
}

#[no_mangle]
pub fn pit() {
    // EOI
    outb(0x20, 0x20);

    env::time::increment();
}

#[no_mangle]
pub fn kb() {
    // EOI
    outb(0x20, 0x20);

    let mut scancode = 0;

    while scancode < 1 {
        scancode = inb(0x60);
    }

    let key = US_KB[scancode as usize];

    if key != '\x09' {
        print!("Current uptime: {} seconds\n", env::time::seconds());
    }

}

#[no_mangle]
pub fn primary_ata() {
    // EOI
    outb(0x20, 0x20);
    outb(0xa0, 0x20);

    print!("Primary ATA");
}

#[no_mangle]
pub fn secondary_ata() {
    // EOI
    outb(0x20, 0x20);
    outb(0xa0, 0x20);

    print!("Secondary ATA");
}

/// US keyboard layout
/// Modified version of layout found here: http://www.osdever.net/bkerndev/Docs/keyboard.htm
/// The escaped "x09" represents a null character and is ignored.
pub const US_KB: [char; 256] = ['0', '2', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-',
                                '=', '\x08', '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o',
                                'p', '[', ']', '\n', '\x09', 'a', 's', 'd', 'f', 'g', 'h', 'j',
                                'k', 'l', ';', '\'', '`', '\x09', '\\', 'z', 'x', 'c', 'v', 'b',
                                'n', 'm', ',', '.', '/', '\x09', '*', '\x09', ' ', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '-', '\x09',
                                '\x09', '\x09', '+', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09', '\x09',
                                '\x09', '\x09', '\x09', '\x09'];
