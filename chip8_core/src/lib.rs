pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16; // The amount of V Registers the program uses.
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

pub struct Emu {
    pc: u16, // Program Counter (keeps track of current instruction index)
    ram: [u8; RAM_SIZE], // RAM. Fixed size array of 4096 unsigned 8-bit integers.
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // Array of 2048 booleans to determine where a pixel should be black or white.
    v_reg: [u8; NUM_REGS], // V Registers are 8-bits, and we have 16 of them. 
    i_reg: u16,
    sp: u16, // Stack Pointer. Refers to the top of our stack. 
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8, // Delay Timer. Once at 0, an action is performed.
    st: u8, // Sound Timer. Once at 0, audio is played. 
}