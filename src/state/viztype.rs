use core::fmt;

#[derive(Debug)]
pub enum VizType {
    GUI, 
    TERM
}

impl fmt::Display for VizType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}