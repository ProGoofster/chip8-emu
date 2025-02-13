//screen size.
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

//Chip-8 doesn't have a set amount of memory, but 4KiB is what it was designed for.
const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;

//emulator data
pub struct emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
}