use std::collections::HashMap;

pub struct SpaceLocalBuffer {
    pub buffer: HashMap<String, Vec<u8>>,
}

impl SpaceLocalBuffer {
    pub fn new() -> SpaceLocalBuffer {
        return SpaceLocalBuffer {
            buffer: HashMap::new(),
        };
    }

    pub fn insert(&mut self, name: String, value: Vec<u8>) {
        self.buffer.insert(name, value);
    }
}

lazy_static!(
    pub static ref GLOBAL: SpaceLocalBuffer = SpaceLocalBuffer::new();
);
