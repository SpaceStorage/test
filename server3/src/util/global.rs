use std::collections::HashMap;
use std::sync::Mutex;

use crate::metrics::prometheus;

pub struct SpaceLocalBuffer {
    pub buffer: HashMap<String, Vec<u8>>,
    pub buffer_size: usize,
    pub metrics_tree: prometheus::Prometheus,
}

impl SpaceLocalBuffer {
    pub fn new() -> SpaceLocalBuffer {
        return SpaceLocalBuffer {
            buffer: HashMap::new(),
            buffer_size: 1000000,
            metrics_tree: prometheus::Prometheus::new(),
        };
    }

    pub fn insert(&mut self, name: String, value: Vec<u8>) {
        self.buffer.insert(name, value);
    }
}

lazy_static!(
    pub static ref GLOBAL: Mutex<SpaceLocalBuffer> = Mutex::new(SpaceLocalBuffer::new());
);
