use std::fmt;

#[derive(Debug, Clone)]
pub enum Arg<'a> {
    Flag(Flag<'a>),
    Opt(Opt<'a>),
    Param(Param<'a>),
}

impl<'a> fmt::Display for Arg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Flag(flag) => flag.to_string(),
            Self::Opt(opt) => opt.to_string(),
            Self::Param(param) => param.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl<'a> Arg<'a> {
    pub fn flag(flag: &'a str) -> Self {
        Self::Flag(Flag::new_short(flag))
    }

    pub fn opt(flag: &'a str, opt: &'a str) -> Self {
        Self::Opt(Opt::new_short(flag, opt))
    }

    pub fn param(param: &'a str) -> Self {
        Self::Param(Param::new(param))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Flag<'a> {
    pub short: Option<&'a str>,
    pub long: Option<&'a str>,
}

impl<'a> Flag<'a> {
    pub fn new_short(flag: &'a str) -> Self {
        Flag {
            short: Some(flag),
            ..Default::default()
        }
    }

    pub fn new_long(flag: &'a str) -> Self {
        Flag {
            long: Some(flag),
            ..Default::default()
        }
    }

    pub fn get_short(&self) -> Option<&'a str> {
        self.short
    }

    pub fn get_long(&self) -> Option<&'a str> {
        self.long
    }
}

impl<'a> fmt::Display for Flag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(short) = self.short {
            return write!(f, "{}", short);
        }
        write!(f, "")
    }
}

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

#[test]
fn args_list() {}
