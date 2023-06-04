mod add;
mod div;
mod mul;
mod sub;

use std::fmt;

use anyhow::Result;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type word = u64;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type dword = u128;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct uHuge {
    digits: Vec<word>,
}

impl uHuge {
    pub fn from_str(s: &str) -> Result<Self> {
        let mut digits = Vec::new();
        let mut end = s.len();
        let width = word::BITS as usize / 4;
        while end > width {
            digits.push(word::from_str_radix(&s[(end - width)..end], 16)?);
            end -= width;
        }
        digits.push(word::from_str_radix(&s[..end], 16)?);
        pop_zero(&mut digits);
        Ok(uHuge { digits })
    }
}

pub(crate) fn pop_zero(digits: &mut Vec<word>) {
    while let Some(digit) = digits.pop() {
        if digit != 0 {
            digits.push(digit);
            break;
        }
    }
}

impl fmt::Display for uHuge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{:0w$X} ", digit, w = word::BITS as usize)?;
        }
        Ok(())
    }
}
