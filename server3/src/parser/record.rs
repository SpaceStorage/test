use std::collections::HashMap;
use crate::parser::{syslog};
pub struct Record {
    pub field: HashMap<String, Vec<u8>>,
    pub data: Vec<u8>,
}

impl Record {
    pub fn new(res: Vec<u8>) -> Self {
        return Record {
            data: res,
            field: HashMap::new(),
        };
    }

    pub fn is_syslog(self) -> bool {
        if self.data[0] == '<' as u8 {
            return true;
        } else {
            return false;
        }
    }

    pub fn syslog_parse(&mut self) {
        syslog::run(self);
    }
}
