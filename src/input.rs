use glutin::event::{ElementState, VirtualKeyCode};

pub struct Input {
    keys: [bool; 16]
}

impl Input {
    pub fn new() -> Input {
        Input {
            keys: [false; 16]
        }
    }

    pub fn process(&mut self, key: VirtualKeyCode, state: ElementState) {

        let offset: Option<usize> = match key {
            VirtualKeyCode::Key1 => Some(0x0),
            VirtualKeyCode::Key2 => Some(0x1),
            VirtualKeyCode::Key3 => Some(0x2),
            VirtualKeyCode::Key4 => Some(0x3),
            VirtualKeyCode::Q =>    Some(0x5),
            VirtualKeyCode::W =>    Some(0x6),
            VirtualKeyCode::E =>    Some(0x7),
            VirtualKeyCode::R =>    Some(0x7),
            VirtualKeyCode::A =>    Some(0x8),
            VirtualKeyCode::S =>    Some(0x9),
            VirtualKeyCode::D =>    Some(0xA),
            VirtualKeyCode::F =>    Some(0xB),
            VirtualKeyCode::Z =>    Some(0xC),
            VirtualKeyCode::X =>    Some(0xD),
            VirtualKeyCode::C =>    Some(0xE),
            VirtualKeyCode::V =>    Some(0xF),
            _ => None,
        };

        match offset {
            Some(x) =>  {
                self.keys[x] = state == ElementState::Pressed;
            },
            _ => ()
        };
    }

    pub fn has_keys(&mut self) -> bool {
        for (_key, value) in self.keys.iter().enumerate() {
            if *value {
                return true;
            }
        }

        return false;
    }
}