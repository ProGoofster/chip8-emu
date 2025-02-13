//screen size.
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

//emulator data
pub struct Chip8 {
    pc: u16, //program counter
    ram: [u8; 4096], //Chip-8 doesn't have a set amount of memory, but 4KiB is what it was designed for.
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    v_regs: [u8; 16],
    i_reg: u16, //index register
    sp: u16, //stack pointer
    stack: [u16; 16],
    dt: u8, //delay timer
    st: u8, //sound timer
}