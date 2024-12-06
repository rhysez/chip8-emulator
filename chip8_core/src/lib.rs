pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16; // The amount of V Registers the program uses.
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200; // The memory address of the first byte. 
const FONTSET_SIZE: usize = 80;

// Character sprite data using hexadecimal numbers.
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
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

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

// Implementation block for Emu struct. Allowing us to add our constructor method. 
impl Emu {
    pub fn new() -> Self {
        // Initialise all values to zero. Except for PC. 
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0
        };

        // ..FONTSET_SIZE specifies all array indexes from 0 up to the size of our character sprite.
        // Then we copy the values of FONTSET into RAM.
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        
        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val; // We add the new 16-bit value to the location of the sp.
        self.sp += 1; // We have pushed a new value to the stack. So we must increment sp by 1.
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1; // We have popped the last value from the stack. So we must decrement sp by 1.
        self.stack[self.sp as usize] // We return the next item at the top of the stack.
    }

    // Handles each step that the CPU makes (a tick) to execute instructions.
    // 1. Fetch the value (opcode) from the game (stored in RAM) at the memory address stored in the Program Counter.
    // 2. Decode the instruction.
    // 3. Execute the instruction (this may involve modifying the CPU or RAM).
    // 4. Move the PC to the next instruction and repeat.
    pub fn tick(&mut self) {
        // Fetch.
        let op = self.fetch();
        // Decode & execute.
        self.execute(op);
    }

    // Fetches the opcode stored at the specified memory address in Program Counter.
    // This function does not need to be public because it is only accessed within the Emu object.
    fn fetch(&mut self) -> u16 {
        // We get 
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;

        // We use a left shift bitwise operator here (<<).
        // This shifts the bits of a number to the left by a specified amount.
        // For each number we shift, this is equivalent to multiplying that number by 2.
        let op = (higher_byte << 8) | lower_byte; // We combine the higher and lower bytes to form an opcode.
        self.pc += 2; // We move the PC up by 2 bytes because every opcode will be 2 bytes in size.
        
        op
    }

    // Every frame, our timers will decrement.
    // Frames operate at a different speed to CPU ticks, so we must handle them in this function.
    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP
            }          
            self.st -= 1;
        }
    }

    fn execute(&mut self, op: u16) {
        // We use the right shift bitwise operator here (>>).
        // This shifts a values bits to the right by a specified amount.
        // Here we are getting each digit in the opcode,
        // and shifting them right by the amount we specify.
        // For each number we specify, this is equivalent to dividing the number by 2 (cut in half).

        // We also use the bitwise AND operator (&).
        // This compares two numbers in binary format (e.g 0000 0101) and gets all shared bits (where the bit is 1) between both numbers.
        // For example, if we compared 0000 0101 with 0000 0011, the result would be 0000 0001. 
        // Because this is the only shared 1 between both numbers.

        // In this function, we get each digit in the opcode.
        // (op & 0xF000) gives us the first digit, for example.
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        // Now that we have digits 1 through 4, we handle them based on the opcode they create.
        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return, // If all digits are 0, do nothing
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op), // If digits are not valid, panic
            (0, 0, 0xE, 0) => { // 000E0 clears the screen.
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT]
            },
            (0, 0, 0xE, 0xE) => { // 000EE returns from subroutine.
                let ret_addr = self.pop(); // Runs subroutine from top of stack.
                self.pc = ret_addr; // Returns the PC to the next item after popping.
            },
            (1, _, _, _) => { // 1NNN jumps.
                let nnn = op & 0xFFF;
                self.pc = nnn;
            },
        }
    }
}