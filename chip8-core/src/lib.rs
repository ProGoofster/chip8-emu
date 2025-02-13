//screen size.
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub const RAM_SIZE: usize = 4096;
pub const NUM_REGS: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const NUM_KEYS: usize = 16;
pub const START_ADDR: u16 = 0x200;
//emulator data
pub struct Chip8 {
    pc: u16, //program counter
    ram: [u8; RAM_SIZE], //Chip-8 doesn't have a set amount of memory, but 4KiB is what it was designed for.
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    v_regs: [u8; NUM_REGS],
    i_reg: u16, //index register
    sp: u16, //stack pointer
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8, //delay timer
    st: u8, //sound timer
}

impl Chip8 {
    //constructor
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            v_regs: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize] //return is not needed when the returned value is last.
    }
}