use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub enum AsNumber {
    Small(u16),
    Extended(u32),
}

/// Transition AS number to be used as peer AS when peers determine AS4 number capability
pub const AS_TRANS: AsNumber = AsNumber::Small(23456);

impl From<u16> for AsNumber {
    fn from(value: u16) -> Self {
        AsNumber::Small(value)
    }
}

impl From<u32> for AsNumber {
    fn from(value: u32) -> Self {
        if value <= 65535 { AsNumber::Small(value as u16) } else { AsNumber::Extended(value) }
    }
}

impl fmt::Display for AsNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AsNumber::Small(number) => number.fmt(f),
            AsNumber::Extended(number) => number.fmt(f)
        }
    }
}

impl fmt::Debug for AsNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
