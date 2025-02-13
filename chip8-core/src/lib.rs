//screen size.
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub const RAM_SIZE: usize = 4096;
pub const NUM_REGS: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const NUM_KEYS: usize = 16;
pub const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

//emulator data
pub struct Chip8 {
    pc: u16,             //program counter
    ram: [u8; RAM_SIZE], //Chip-8 doesn't have a set amount of memory, but 4KiB is what it was designed for.
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    v_regs: [u8; NUM_REGS],
    i_reg: u16, //index register
    sp: u16,    //stack pointer
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8, //delay timer
    st: u8, //sound timer
}

impl Chip8 {
    //constructor
    pub fn new() -> Self {
        let mut chip8 = Self {
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
        };
        
        chip8.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        chip8
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
        self.v_regs = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;

        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

    }

    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode & Execute
        self.execute(op);
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // do sound
            }
            self.st -= 1;
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

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        (higher_byte << 8) | lower_byte
    }

    fn execute(&mut self, op: u16){
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (_,_,_,_) => unimplemented!("Unimplemented opcode {}", op),
        }
    }
}

//currently on 4.2
