use std::collections::HashMap;
use std::sync::Mutex;

pub struct SpaceLocalBuffer {
    pub buffer: HashMap<String, Vec<u8>>,
    pub buffer_size: usize,
}

impl SpaceLocalBuffer {
    pub fn new() -> SpaceLocalBuffer {
        return SpaceLocalBuffer {
            buffer: HashMap::new(),
            buffer_size: 1000000,
        };
    }

    pub fn insert(&mut self, name: String, value: Vec<u8>) {
        self.buffer.insert(name, value);
    }
}

lazy_static!(
    pub static ref GLOBAL: Mutex<SpaceLocalBuffer> = Mutex::new(SpaceLocalBuffer::new());
);
