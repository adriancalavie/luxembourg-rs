use std::mem;

#[derive(Hash, Eq, PartialEq)]
pub struct HF64((u64, i16, i8));

impl HF64 {
    pub fn new(val: f64) -> HF64 {
        HF64(integer_decode(val))
    }
}

// https://stackoverflow.com/a/39639200/11294167
fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}
