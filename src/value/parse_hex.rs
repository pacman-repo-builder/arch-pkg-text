use core::{
    mem::size_of,
    ops::{BitOrAssign, Shl},
    str::Chars,
};

pub trait ParseHex: Sized {
    fn parse_hex(input: &str) -> (&'_ str, Self);
}

impl<const LEN: usize> ParseHex for [u8; LEN] {
    fn parse_hex(input: &str) -> (&str, Self) {
        let mut chars = input.chars();
        let mut array: [u8; LEN] = [0; LEN];

        for item in array.iter_mut().rev() {
            *item = parse_hex_value::<u8, { size_of::<u8>() }>(&mut chars);
        }

        (chars.as_str(), array)
    }
}

impl ParseHex for u128 {
    fn parse_hex(input: &str) -> (&'_ str, Self) {
        let mut chars = input.chars();
        let value = parse_hex_value::<_, { size_of::<u128>() }>(&mut chars);
        (chars.as_str(), value)
    }
}

fn parse_hex_value<Value, const SIZE: usize>(chars: &mut Chars) -> Value
where
    Value: From<u8> + BitOrAssign + Shl<usize, Output = Value>,
{
    let mut value: Value = 0.into();
    for digit_pos in 0..(SIZE * 2) {
        let Some(digit_value) = chars.next_back().and_then(parse_hex_digit) else {
            return value;
        };
        value |= Value::from(digit_value) << (digit_pos * 4);
    }
    value
}

fn parse_hex_digit(digit: char) -> Option<u8> {
    let code = match digit {
        '0'..='9' => digit as u32 - '0' as u32,
        'a'..='f' => 0xA + digit as u32 - 'a' as u32,
        'A'..='F' => 0xA + digit as u32 - 'A' as u32,
        _ => return None,
    };
    debug_assert!(code <= 0xF);
    Some(code as u8)
}

#[test]
fn parse_hex_u8() {
    use pretty_assertions::assert_eq;
    let hex = "17afb88";

    assert_eq!(
        <[u8; 6]>::parse_hex(hex),
        ("", [0x00, 0x00, 0x01, 0x7a, 0xfb, 0x88]),
    );
    assert_eq!(
        <[u8; 5]>::parse_hex(hex),
        ("", [0x00, 0x01, 0x7a, 0xfb, 0x88]),
    );
    assert_eq!(<[u8; 4]>::parse_hex(hex), ("", [0x01, 0x7a, 0xfb, 0x88]));
    assert_eq!(<[u8; 3]>::parse_hex(hex), ("1", [0x7a, 0xfb, 0x88]));
    assert_eq!(<[u8; 2]>::parse_hex(hex), ("17a", [0xfb, 0x88]));
    assert_eq!(<[u8; 1]>::parse_hex(hex), ("17afb", [0x88]));
    assert_eq!(<[u8; 0]>::parse_hex(hex), (hex, []));
}
