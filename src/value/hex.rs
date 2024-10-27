use core::mem::size_of;

pub trait ParseHex: Sized {
    fn parse_hex(input: &str) -> (&'_ str, Self);
}

macro_rules! impl_parse_hex {
    ($($num:ident)*) => {$(
        impl<const LEN: usize> ParseHex for [$num; LEN] {
            fn parse_hex(input: &str) -> (&str, Self) {
                let mut chars = input.chars();
                let mut array: [$num; LEN] = [0; LEN];

                for item in array.iter_mut().rev() {
                    const DIGIT_COUNT: usize = size_of::<$num>() * 2;
                    for digit_pos in 0..DIGIT_COUNT {
                        let Some(digit_value) = chars.next_back().and_then(parse_hex_digit) else {
                            return (chars.as_str(), array);
                        };
                        *item |= (digit_value as $num) << (digit_pos * 4);
                    }
                }

                (chars.as_str(), array)
            }
        }
    )*};
}

impl_parse_hex!(u8 u16 u32 u64 u128);

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
fn parse_hex() {
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

    assert_eq!(
        <[u16; 4]>::parse_hex(hex),
        ("", [0x0000, 0x0000, 0x017a, 0xfb88]),
    );
    assert_eq!(<[u16; 3]>::parse_hex(hex), ("", [0x0000, 0x017a, 0xfb88]));
    assert_eq!(<[u16; 2]>::parse_hex(hex), ("", [0x017a, 0xfb88]));
    assert_eq!(<[u16; 1]>::parse_hex(hex), ("17a", [0xfb88]));
    assert_eq!(<[u16; 0]>::parse_hex(hex), (hex, []));

    assert_eq!(
        <[u32; 3]>::parse_hex(hex),
        ("", [0x00000000, 0x00000000, 0x017afb88]),
    );
    assert_eq!(<[u32; 2]>::parse_hex(hex), ("", [0x00000000, 0x017afb88]));
    assert_eq!(<[u32; 1]>::parse_hex(hex), ("", [0x017afb88]));
    assert_eq!(<[u32; 0]>::parse_hex(hex), (hex, []));

    assert_eq!(<[u64; 3]>::parse_hex(hex), ("", [0, 0, 0x17afb88]));
    assert_eq!(<[u64; 2]>::parse_hex(hex), ("", [0, 0x17afb88]));
    assert_eq!(<[u64; 1]>::parse_hex(hex), ("", [0x17afb88]));
    assert_eq!(<[u64; 0]>::parse_hex(hex), (hex, []));
}
