use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub enum AsNum {
    Small(u16),
    Extended(u32),
}

/// Transition AS number to be used as peer AS when peers determine AS4 number capability
pub const AS_TRANS: AsNum = AsNum::Small(23456);

impl From<u16> for AsNum {
    fn from(value: u16) -> Self {
        AsNum::Small(value)
    }
}

impl From<u32> for AsNum {
    fn from(value: u32) -> Self {
        if value <= 65535 { AsNum::Small(value as u16) } else { AsNum::Extended(value) }
    }
}

impl fmt::Display for AsNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AsNum::Small(number) => number.fmt(f),
            AsNum::Extended(number) =>  number.fmt(f)
        }
    }
}

impl fmt::Debug for AsNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}