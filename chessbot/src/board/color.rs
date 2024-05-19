
#[derive(Debug,PartialEq, Clone, Copy)]
pub enum Color {
    White = 8,
    Black = 16
}

impl Color {
    pub fn from_char(c: char) -> Color{
        if c.is_uppercase() || c == 'w'{
            Color::White
        }else {
            Color::Black
        }
    }

    pub fn from_int(i: u8) -> Color{
        if i & 8 == 8 {
            Color::White
        }else if i & 16 == 16 {
            Color::Black
        }else {
            panic!("{} not a valid color", i)
        }
    }

    pub fn other(&self) -> Color{
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn to_0_1(&self) -> usize{
        match self {
            Color::White => 0,
            Color::Black => 1
        }
    }
}