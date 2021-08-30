use core::fmt;

#[derive(Debug, PartialEq)]
pub enum VizType {
    GUI, 
    TERM,
    WEB,
}

impl fmt::Display for VizType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}