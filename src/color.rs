use std::{
    fmt::{Debug, Display},
    str::FromStr,
};


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            return Err("Invalid length");
        }

        if &s[0..1] != "#" {
            return Err("Missing #");
        }

        let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| "Invalid red")?;
        let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| "Invalid green")?;
        let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| "Invalid blue")?;

        Ok(Color(r, g, b))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2);
        write!(f, "{}", color)
    }
}
