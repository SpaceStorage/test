// vim: ts=4 sw=4 et autoindent backspace=indent,eol,start ruler showcmd
pub struct Config {
    pub client_addr: String,
    pub internode_addr: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            client_addr: "0.0.0.0:10100".to_string(),
            internode_addr: "0.0.0.0:10101".to_string(),
        }
    }
}
