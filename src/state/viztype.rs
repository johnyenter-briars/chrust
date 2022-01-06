use core::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum VizType {
    Term,
    Web,
}

impl fmt::Display for VizType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
