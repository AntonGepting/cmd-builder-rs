use std::fmt;

#[derive(Debug, Clone)]
pub struct Param<'a> {
    pub param: &'a str,
}

impl<'a> fmt::Display for Param<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.param)
    }
}

impl<'a> Param<'a> {
    pub fn new(param: &'a str) -> Self {
        Param { param }
    }

    //pub fn get_short(&self) -> Option<&'a str> {
    //self.short
    //}

    //pub fn get_long(&self) -> Option<&'a str> {
    //self.long
    //}
}
