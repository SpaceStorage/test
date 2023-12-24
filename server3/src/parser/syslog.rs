use std::str;
use crate::parser::record::{Record};

//pub fn parse(data: &[u8]) {
//    match str::from_utf8(data) {
//        Ok(v) => {
//            let ret_result = syslog_loose::parse_message(v);
//            println!("proto {:?}, facility {:?}, severity {:?}, timestamp {:?}, hostname {:?}, appname {:?}, msg {:?}, structured {:?}", ret_result.protocol, ret_result.facility, ret_result.severity, ret_result.timestamp, ret_result.hostname, ret_result.appname, ret_result.msg, ret_result.structured_data);
//        }
//        Err(_) => {}
//    };
//}

pub fn run(rec: &mut Record) {
    match str::from_utf8(&rec.data) {
        Ok(v) => {
            let ret_result = syslog_loose::parse_message(v);
            if let Some(v) = ret_result.appname {
                rec.field.insert("appname".to_string(), v.as_bytes().to_vec());
            }
            if let Some(v) = ret_result.hostname {
                rec.field.insert("hostname".to_string(), v.as_bytes().to_vec());
            }
            //if let Some(v) = ret_result.protocol {
            //    rec.field.insert("protocol".to_string(), v.as_str().as_bytes().to_vec());
            //}
            if let Some(v) = ret_result.facility {
                rec.field.insert("facility".to_string(), v.as_str().as_bytes().to_vec());
            }
            if let Some(v) = ret_result.severity {
                rec.field.insert("severity".to_string(), v.as_str().as_bytes().to_vec());
            }
            //if let Some(v) = ret_result.timestamp {
            //    rec.field.insert("timestamp".to_string(), v.as_bytes().to_vec());
            //}
            rec.field.insert("msg".to_string(), v.as_bytes().to_vec());
        }
        Err(_) => {}
    };
}
