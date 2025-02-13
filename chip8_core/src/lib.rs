use rand::random;

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

    pub fn get_display(&self) -> &[[bool; 64]; 32] {
        &self.screen
    }

    pub fn set_key(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn press_key(&mut self, idx: usize) {
        self.set_key(idx, true);
    }

    pub fn release_key(&mut self, idx: usize) {
        self.set_key(idx, false);
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
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
            //NO OP
            (0,0,0,0) => return,
            // clear
            (0,0,0xE,0) => self.screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            // return
            (0,0,0xE,0xE) => self.pc = self.pop(),
            // jump NNN
            (1,_,_,_) => self.pc = op & 0xFFF,
            // call NNN
            (2,_,_,_) => {
                self.push(self.pc);
                self.pc = op & 0xFFF;
            },
            // if vx != NN
            // skip if vx == nn
            (3,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_regs[x] == nn {self.pc += 2}
            },
            // if vx == NN
            // skip if vx != nn
            (4,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_regs[x] == nn {self.pc += 2}
            },
            // if vx != vy
            // skip if vx == vy
            (5,_,_,_) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_regs[x] == self.v_regs[y] {self.pc += 2}
            },

            // vx = nn
            (6,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_regs[x] = nn;
            },

            // vx += nn
            (7,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_regs[x] += nn;
            },

            // vx = vy
            (8,_,_,0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_regs[x] += self.v_regs[y];
            },
            // vx |= vy
            (8,_,_,1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_regs[x] |= self.v_regs[y];
            },
            // vx &= vy
            (8,_,_,2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_regs[x] &= self.v_regs[y];
            },
            // vx ^= vy
            (8,_,_,3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_regs[x] ^= self.v_regs[y];
            },
            // vx += vy
            (8,_,_,4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.v_regs[x].overflowing_add(self.v_regs[y]);
                
                self.v_regs[x] = new_vx;
                self.v_regs[0xF] = carry as u8;
            },
            // vx -= vy
            (8,_,_,5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.v_regs[x].overflowing_sub(self.v_regs[y]);
                
                self.v_regs[x] = new_vx;
                self.v_regs[0xF] = !carry as u8;
            },
            // vx = vy >> 1 right shift
            (8,_,_,6) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let dropped_bit = self.v_regs[y] & 1;

                self.v_regs[x] = self.v_regs[y] >> 1;
                self.v_regs[0xF] = dropped_bit;
            },
            // vx = vy - vx
            (8,_,_,7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.v_regs[y].overflowing_sub(self.v_regs[x]);
                
                self.v_regs[x] = new_vx;
                self.v_regs[0xF] = !carry as u8;
            },
            // vx = vy << 1 left shift
            (8,_,_,0xE) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let dropped_bit = (self.v_regs[y] >> 7) & 1;

                self.v_regs[x] = self.v_regs[y] << 1;
                self.v_regs[0xF] = dropped_bit;
            },
            // if vx == vy
            // skip if vx != vy
            (9,_,_,0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_regs[x] != self.v_regs[y] {self.pc += 2}
            },

            // i = nnn
            (0xA,_,_,_) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            },

            // jump to nnn + v0
            (0xB,_,_,_) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_regs[0] as u16) + nnn;
            },

            (0xC,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_regs[x] = rng & nn;
            },

            // draw
            // maybe rewrite it since I don't fully understand it
            (0xD,_,_,_) => {
                //coords
                let x_coord = self.v_regs[digit2 as usize] as u16;
                let y_coord = self.v_regs[digit3 as usize] as u16;
                //height
                let num_rows = digit4;

                let mut flipped = false;

                for y_line in 0..num_rows {
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];
                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            //Sprites wrap around screen
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_WIDTH;

                            //need to figure out the logic of this
                            //this could be a failure point because I
                            //implemented the screen differently
                            flipped |= self.screen[x][y];
                            self.screen[x][y] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v_regs[0xF] = 1;
                } else {
                    self.v_regs[0xF] = 0;
                }
            },
            
            // skip if key pressed
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                
                if self.keys[self.v_regs[x] as usize] {
                    self.pc += 2;
                }
            },

            // skip if key not pressed
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                
                if !self.keys[self.v_regs[x] as usize] {
                    self.pc += 2;
                }
            },

            // get delay
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.v_regs[x] = self.dt;
            },

            // wait for key press
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;
                let mut i: usize = 0;
                while !pressed {
                    if self.keys[i] {
                        pressed = true;
                        self.v_regs[x] = i as u8;
                    }
                    i = (i + 1) % self.keys.len();
                }
            },

            // set delay timer
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.dt = self.v_regs[x];
            },

            // set sound timer
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.st = self.v_regs[x];
            },

            // load sprite data to I
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_regs[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },
            
            // set I to font address
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.v_regs[x] as u16;
                self.i_reg = c * 5;
            },

            // store BCD from VX into I
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let vx = self.v_regs[x] as f32;

                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            },

            // store V0 to VX into memory starting at I
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_regs[idx];
                }
                self.i_reg += digit2 + 1;
            },

            // Fill V0 to VX with values loaded from memory starting with I
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_regs[idx] = self.ram[i + idx];
                }
                self.i_reg += digit2 + 1;
            },

            (_,_,_,_) => unimplemented!("Unimplemented opcode {}", op),
        }
    }
}

//currently on 4.2