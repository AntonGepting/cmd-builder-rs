use crate::Flag;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Opt<'a> {
    pub flag: Flag<'a>,
    pub opt: &'a str,
}

impl<'a> fmt::Display for Opt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.flag.to_string(), self.opt)
    }
}

impl<'a> Opt<'a> {
    pub fn new_short(flag: &'a str, opt: &'a str) -> Self {
        Opt {
            flag: Flag::new_short(flag),
            opt: opt,
        }
    }

    pub fn new_long(flag: &'a str, opt: &'a str) -> Self {
        Opt {
            flag: Flag::new_long(flag),
            opt: opt,
        }
    }

    //pub fn get(&self) -> (Flag, &'a str) {
    //self.short
    //}
}
