use std::fmt;

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
