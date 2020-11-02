use rand;
use rand::prelude::ThreadRng;
use rand::Rng;

use super::input;
use super::gpu;
use glutin::event::{ElementState,  VirtualKeyCode};

pub struct Cpu {
    ram: [u8; 4096],
    v: [u8; 16],
    i: usize,
    pc: usize,
    stack: [usize; 16],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
    gpu: gpu::Gpu,
    rng: ThreadRng,
    input: input::Input
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            ram: [0u8; 4096],
            v: [0u8; 16],
            i: 0,
            pc: 0x200, // by convention
            stack: [0usize; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            gpu: gpu::Gpu::new(),
            rng: rand::thread_rng(),
            input: input::Input::new()
        }
    }

    pub fn initialize(&mut self) {
        //Initialize memory with the predefined sprites from 0, 1, 2 ... F
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut i = 0;
        for sprite in &sprites {
            for ch in sprite {
                self.ram[i] = *ch;
                i += 1;
            }
        }

    }

    pub fn load(&mut self, rom: Vec<u8>) {
        for (i, &byte) in rom.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.ram[0x200 + i] = byte;
            } else {
                break;
            }
        }

        self.pc = 0x200;
    }

    pub fn tick(&mut self) {
        self.opcode();
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -=1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -=1;
        }
    }

    pub fn process_key(&mut self, key: VirtualKeyCode, status: ElementState) {
        self.input.process(key, status);
    }

    fn opcode(&mut self) {
        // shamelessly taken from https://github.com/starrhorne/chip8-rust
        // very clever and clean way to do opcode matching
        let opcode :u16 = (self.ram[self.pc] as u16) << 8 | self.ram[self.pc + 1] as u16;
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        println!("{:?}", self.v);
        println!("{:x?}", opcode);
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.clear_screen(),
            (0x00, 0x00, 0x0e, 0x0e) => self.func_return(),
            (0x01, _, _, _) => self.goto_nnn(nnn),
            (0x02, _, _, _) => self.call_sub_nnn(nnn),
            (0x03, _, _, _) => self.skip_if_equal_immediate(x, kk),
            (0x04, _, _, _) => self.skip_if_not_equal_immediate(x, kk),
            (0x05, _, _, 0x00) => self.skip_if_equal_ref(x, y),
            (0x06, _, _, _) => self.set_immediate(x, kk),
            (0x07, _, _, _) => self.increment_no_carry(x, kk),
            (0x08, _, _, 0x00) => self.assign(x, y),
            (0x08, _, _, 0x01) => self.bit_or(x, y),
            (0x08, _, _, 0x02) => self.bit_and(x, y),
            (0x08, _, _, 0x03) => self.bit_xor(x, y),
            (0x08, _, _, 0x04) => self.add_ref(x, y),
            (0x08, _, _, 0x05) => self.decrement_ref(x, y),
            (0x08, _, _, 0x06) => self.shift_right(x),
            (0x08, _, _, 0x07) => self.subtract_ref(x, y),
            (0x08, _, _, 0x0e) => self.shift_left(x),
            (0x09, _, _, 0x00) => self.skip_if_not_equal_ref(x, y),
            (0x0a, _, _, _) => self.set_i(nnn),
            (0x0b, _, _, _) => self.jump_to_nnn_with_v0_offset(nnn),
            (0x0c, _, _, _) => self.random_nnn(x, kk),
            (0x0d, _, _, _) => self.draw(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.check_stored_key_equal(x),
            (0x0e, _, 0x0a, 0x01) => self.check_stored_key_not_equal(x),
            (0x0f, _, 0x00, 0x07) => self.store_delay_timer(x),
            (0x0f, _, 0x00, 0x0a) => self.store_key(x),
            (0x0f, _, 0x01, 0x05) => self.set_delay_timer(x),
            (0x0f, _, 0x01, 0x08) => self.set_sound_timer(x),
            (0x0f, _, 0x01, 0x0e) => self.increment_i(x),
            (0x0f, _, 0x02, 0x09) => self.set_sprite_for_digit(x),
            (0x0f, _, 0x03, 0x03) => self.store_bcd_at_i(x),
            (0x0f, _, 0x05, 0x05) => self.store_registers(x),
            (0x0f, _, 0x06, 0x05) => self.load_registers(x),
            _ => (),
        };
    }

    // Op-codes

    // 0x00E0
    fn clear_screen(&mut self) {
        self.gpu.clear();
        self.pc += 2;
    }

    // 0x00EE
    fn func_return(&mut self) {
        self.sp = self.sp - 1;
        self.pc = self.stack[self.sp] as usize;
    }

    // 1NNN
    fn goto_nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }

    // 2NNN
    fn call_sub_nnn(&mut self, nnn: usize) {
        self.stack[self.sp] = self.pc;
        self.sp = self.sp + 1;
        self.pc = nnn;
    }


    // 3XKK
    fn skip_if_equal_immediate(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 4; // skip
        } else {
            self.pc += 2; // continue
        }
    }

    //4XKK
    fn skip_if_not_equal_immediate(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc +=4 ; // skip
        } else {
            self.pc += 2; // continue
        }
    }

    //5XY0
    fn skip_if_equal_ref(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc +=4 ; // skip
        } else {
            self.pc += 2; // continue
        }
    }

    //6xkk
    fn set_immediate(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
        self.pc +=2;
    }

    //7xy
    fn increment_no_carry(&mut self, x: usize, kk: u8) {
        self.v[x] = ((self.v[x] as u16 + kk as u16) % 0xFF) as u8;
        self.pc +=2;
    }

    //8XY0
    fn assign(&mut self, x:usize, y:usize) {
        self.v[x] = self.v[y];
        self.pc +=2;
    }

    //8XY1
    fn bit_or(&mut self, x:usize, y:usize) {
        self.v[x] = self.v[x] | self.v[y];
        self.pc +=2;
    }

    //8XY2
    fn bit_and(&mut self, x:usize, y:usize) {
        self.v[x] = self.v[x] & self.v[y];
        self.pc +=2;
    }

    //8XY3
    fn bit_xor(&mut self, x:usize, y:usize) {
        self.v[x] = self.v[x] ^ self.v[y];
        self.pc +=2;
    }

    //8XY4
    fn add_ref(&mut self, x:usize, y:usize) {
        let add:u16 = self.v[x] as u16 + self.v[y] as u16;

        self.v[0xF] = (add > 0xFF) as u8;

        self.v[x] = add as u8;
        self.pc +=2;
    }

    //8XY5 Vx -= Vy
    fn decrement_ref(&mut self, x:usize, y:usize) {
        self.v[0xF] = (self.v[x] > self.v[y]) as u8;

        self.v[x] = self.v[x] - self.v[y];
        self.pc +=2;
    }

    //8XY6
    fn shift_right(&mut self, x:usize,) {
        self.v[0xF] = self.v[x] & 0x01;

        self.v[x] = self.v[x] >> 1;
        self.pc +=2;
    }

    //8XY7 Vx = (Vy - Vx)
    fn subtract_ref(&mut self, x:usize, y:usize) {
        self.v[0xF] = (self.v[y] > self.v[x]) as u8;

        self.v[x] = self.v[y] - self.v[x];
        self.pc +=2;
    }

    //8XYE
    fn shift_left(&mut self, x:usize) {
        self.v[0xF] = self.v[x] & 0x80;

        self.v[x] = self.v[x] << 1;
        self.pc +=2;
    }

    //9XY0
    fn skip_if_not_equal_ref(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc +=4 ; // skip
        } else {
            self.pc +=2; // continue
        }
    }

    //ANNN
    fn set_i(&mut self, nnn: usize) {
        self.i = nnn;
        self.pc = self.pc + 2;
    }

    //BNNN
    fn jump_to_nnn_with_v0_offset(&mut self, nnn: usize) {
        self.pc = self.v[0] as usize + nnn;
    }

    //CXNN
    fn random_nnn(&mut self, x: usize, kk:u8) {
        let num: u8 = self.rng.gen();
        self.v[x] = num & kk;
        self.pc = self.pc + 2;
    }

    //DXYN
    fn draw(&mut self, x: usize, y:usize, n: usize) {

        let mut erased = 0;
        let mut coord_x = self.v[x] as usize;
        let mut coord_y = self.v[y] as usize;
        let mut b:u8 = n as u8;

        println!("drawing to {}, {}, {}, {:x?}", coord_x, coord_y, n, self.i);

        self.v[0x0f] = 0;

        for byte in 0..n {
            for bit in 0..8 {
                let index = (64 * (coord_y + byte)) + (coord_x + bit);
                let color = (self.ram[self.i + byte] >> (7 - bit)) & 1;
                self.v[0x0f] |= color & self.gpu.screen[index];
                println!("{}",index);
                println!("{}",color);
                self.gpu.screen[index] ^= color;
            }
        }

        let mut owned_string: String = "".to_owned();

        println!("START");
        for i in 0..(64 * 32) {
            if (self.gpu.screen[i] > 0) {
                owned_string.push_str("#");
            }
            else {
                owned_string.push_str(" ");
            }

            if (i % 64 == 0) {
                println!("{}", owned_string);
                owned_string = "".to_string();
            }
        }
        println!("END");

        self.pc = self.pc + 2;
    }

    //EX9E
    fn check_stored_key_equal(&mut self, x: usize) {
        self.pc = self.pc + 2; // TODO get key?
    }

    //EXA1
    fn check_stored_key_not_equal(&mut self, x: usize) {
        self.pc = self.pc + 2; // TODO get key?
    }

    //FX07
    fn store_delay_timer(&mut self, x: usize) {
        self.v[x] = self.delay_timer; // TODO CHECK THIS
        self.pc = self.pc + 2;
    }

    //FX0A
    fn store_key(&mut self, x: usize) {
        self.v[x] = 5; // TODO get key()
        self.pc = self.pc + 2;
    }

    //FX15
    fn set_delay_timer(&mut self, x: usize) {
        self.delay_timer = self.v[x];
        self.pc = self.pc + 2;
    }

    //FX18
    fn set_sound_timer(&mut self, x: usize) {
        self.sound_timer = self.v[x];
        self.pc = self.pc + 2;
    }

    //FX1E
    fn increment_i(&mut self, x: usize) {
        self.i = self.i + self.v[x] as usize;
        self.pc = self.pc + 2;
    }

    //FX29
    fn set_sprite_for_digit(&mut self, x: usize) {
        self.i = self.v[x] as usize * 5; // sprite data starts at 0 and only takes 5 bytes
        self.pc = self.pc + 2;
    }

    //FX33
    fn store_bcd_at_i(&mut self, x: usize) {
        let num = self.v[x];

        self.ram[self.i] = num / 100;
        self.ram[self.i + 1] = (num / 10) % 10;
        self.ram[self.i + 2] = num % 10;
        self.pc = self.pc + 2;
    }

    //FX55
    fn store_registers(&mut self, x:usize) {
        for (y, item) in self.v[0 .. x].iter().enumerate() {
            self.ram[self.i + y] = *item;
        }
    }

    //FX65
    fn load_registers(&mut self, x:usize) {
        for (y, item) in self.ram[self.i .. (self.i + x)].iter().enumerate() {
            self.v[y] = *item;
        }
    }
}