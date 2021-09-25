use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    White = 1,
    Black = 2,
}

impl Color {
    pub fn color_abbr(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}