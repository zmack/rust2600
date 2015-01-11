use std::collections::HashMap;

pub struct Memory {
    mem: Box<HashMap<u16, u8>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: box HashMap::new()
        }
    }

    pub fn get(&self, offset: u16) -> u8 {
        match self.mem.get(&offset) {
            Some(e) => *e,
            _ => 0
        }
    }

    pub fn set(&mut self, offset: u16, data: u8) {
        self.mem.insert(offset, data);
    }

    pub fn load(&mut self, data: Vec<u8>, offset: u16) {
        let mut index = 0;

        for byte in data.iter() {
            self.set(offset + index, *byte);
            index += 1;
        }
    }

}
