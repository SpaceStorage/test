// vim: ts=4 sw=4 et autoindent backspace=indent,eol,start ruler showcmd
extern crate serde_json;
use crate::settings::settings::Config;

pub fn parse_config() -> Config {
    println!("hello");
    return Config::default();
    //return settings
}
