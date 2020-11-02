pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Gpu {
    pub screen: [u8; 64 * 32]
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu{ screen: [0u8; 64 * 32] }
    }

    pub fn clear(&mut self) {
        self.screen.iter_mut().for_each(|m| *m = 0)
    }

    pub fn draw(&mut self, x: u8, y: u8, n: usize) -> u8 {
        // let mut erased = 0;
        // let mut coord_x = x as usize;
        // let mut coord_y = y as usize;
        // let mut b:u8 = n as u8;
        //
        // println!("drawing to {}, {}, {}", x, y, n);
        //
        // let mut collision = false;
        // for byte in 0..n {
        //     for bit in 0..8 {
        //         let index = WIDTH * coord_y + coord_x;
        //         let color = (self.ram[self.i + byte] >> (7 - bit)) & 1;
        //         collision = color & self.vram[y][x];
        //         self.screen[index] ^= color;
        //     }
        // }
        //
        // let mut owned_string: String = "".to_owned();
        //
        //
        // println!("START");
        // for i in 0..(WIDTH * HEIGHT) {
        //     if (self.screen[i] > 0) {
        //         owned_string.push_str("#");
        //     }
        //     else {
        //         owned_string.push_str(" ");
        //     }
        //
        //     if (i % WIDTH == 0) {
        //         println!("{}", owned_string);
        //         owned_string = "".to_string();
        //     }
        // }
        // println!("END");
        // erased

        0
    }
}