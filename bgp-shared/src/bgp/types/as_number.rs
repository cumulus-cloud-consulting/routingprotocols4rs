use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone,Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_yield_small_as_number() {
        match AsNumber::from(4096u16) {
            AsNumber::Small(port_number) => assert_eq!(port_number, 4096),
            AsNumber::Extended(_) => panic!("Should yield small AS number")
        }
    }

    #[test]
    fn should_yield_large_as_number_from_u32_number() {
        match AsNumber::from(131072u32) {
            AsNumber::Small(_) => panic!("Should yield small AS number"),
            AsNumber::Extended(port_number) => assert_eq!(port_number, 131072)
        }
    }

    #[test]
    fn should_yield_small_as_number_from_u16_number() {
        match AsNumber::from(4096u32) {
            AsNumber::Small(port_number) => assert_eq!(port_number, 4096),
            AsNumber::Extended(_) => panic!("Should yield small AS number")
        }
    }
}