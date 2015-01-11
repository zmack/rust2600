use std::io::{File, Open, Read };

pub struct Cart {
    pub data: Box<Vec<u8>>
}

impl Cart {
    pub fn new(filename: &str) -> Cart {
        let path = Path::new(filename);

        let mut file = match File::open_mode(&path, Open, Read) {
            Ok(f) => f,
            _ => panic!("Could not open file")
        };

        let bytes = match file.read_to_end() {
            Ok(vec) => vec,
            _ => panic!("Could not read file")
        };

        Cart {
            data: box bytes
        }
    }

    pub fn get(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn set(&mut self, index: usize, data: u8) {
        self.data[index] = data;
    }
}
