#![allow(dead_code, unused_imports, unused_variables)]

use std::char;
use std::fmt;

use char_utf8::encode_utf8;

const MAX_UTF8_BYTES: usize = 4;

mod char_utf8;

pub enum Utf8Sequence {
    One(Utf8Range),
    Two([Utf8Range; 2]),
    Three([Utf8Range; 3]),
    Four([Utf8Range; 4]),
}

impl fmt::Debug for Utf8Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Utf8Sequence::*;
        match *self {
            One(ref r) => write!(f, "{:?}", r),
            Two(ref r) => write!(f, "{:?}{:?}", r[0], r[1]),
            Three(ref r) => write!(f, "{:?}{:?}{:?}", r[0], r[1], r[2]),
            Four(ref r) => write!(f, "{:?}{:?}{:?}{:?}",
                                  r[0], r[1], r[2], r[3]),
        }
    }
}

pub struct Utf8Range {
    start: u8,
    end: u8,
}

impl fmt::Debug for Utf8Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.start == self.end {
            write!(f, "[{:X}]", self.start)
        } else {
            write!(f, "[{:X}-{:X}]", self.start, self.end)
        }
    }
}

pub fn utf8_ranges(start: char, end: char) -> Vec<Utf8Sequence> {
    // This is largely based on RE2's UTF-8 automaton compiler, but less fancy.
    let mut seqs = vec![];
    add_ranges(&mut seqs, start as u32, end as u32);
    seqs
}

fn add_ranges(seqs: &mut Vec<Utf8Sequence>, start: u32, end: u32) {
    if start > end {
        return;
    }
    for i in 1..MAX_UTF8_BYTES {
        let max = max_scalar_value(i);
        if start <= max && max < end {
            add_ranges(seqs, start, max);
            add_ranges(seqs, max + 1, end);
            return;
        }
    }
    if end <= 0x7f {
        seqs.push(Utf8Sequence::One(Utf8Range {
            start: start as u8,
            end: end as u8,
        }));
        return;
    }
    for i in 1..MAX_UTF8_BYTES {
        let m = (1 << (6 * i)) - 1;
        if (start & !m) != (end & !m) {
            if (start & m) != 0 {
                add_ranges(seqs, start, start | m);
                add_ranges(seqs, (start | m) + 1, end);
                return;
            }
            if (end & m) != m {
                add_ranges(seqs, start, (end & !m) - 1);
                add_ranges(seqs, end & !m, end);
                return;
            }
        }
    }
    let mut start_bytes = [0; MAX_UTF8_BYTES];
    let mut end_bytes = [0; MAX_UTF8_BYTES];
    let char_start = char::from_u32(start).unwrap();
    let char_end = char::from_u32(end).unwrap();
    let n = encode_utf8(char_start, &mut start_bytes).unwrap();
    let m = encode_utf8(char_end, &mut end_bytes).unwrap();
    assert_eq!(n, m);
    let seq = match n {
        2 => Utf8Sequence::Two([
            Utf8Range {
                start: start_bytes[0],
                end: end_bytes[0],
            },
            Utf8Range {
                start: start_bytes[1],
                end: end_bytes[1],
            },
        ]),
        3 => Utf8Sequence::Three([
            Utf8Range {
                start: start_bytes[0],
                end: end_bytes[0],
            },
            Utf8Range {
                start: start_bytes[1],
                end: end_bytes[1],
            },
            Utf8Range {
                start: start_bytes[2],
                end: end_bytes[2],
            },
        ]),
        4 => Utf8Sequence::Four([
            Utf8Range {
                start: start_bytes[0],
                end: end_bytes[0],
            },
            Utf8Range {
                start: start_bytes[1],
                end: end_bytes[1],
            },
            Utf8Range {
                start: start_bytes[2],
                end: end_bytes[2],
            },
            Utf8Range {
                start: start_bytes[3],
                end: end_bytes[3],
            },
        ]),
        _ => unreachable!("invalid encoded length: {}", n),
    };
    seqs.push(seq);
}

fn max_scalar_value(nbytes: usize) -> u32 {
    (match nbytes {
        1 => '\u{007F}',
        2 => '\u{07FF}',
        3 => '\u{FFFF}',
        4 => '\u{10FFFF}',
        _ => unreachable!("invalid UTF-8 byte sequence size"),
    }) as u32
}

trait CharUtil {
    fn next(self) -> Self;
    fn prev(self) -> Self;
}

// impl CharUtil for char {
    // fn next(self) -> char {
        // match self {
            // char::MAX => char::MAX,
            // '\u{D7FF}' => '\u{E000}',
            // c => char::from_u32(c as u32 + 1).unwrap(),
        // }
    // }
//
    // fn prev(self) -> char {
        // match self {
            // '\u{0000}' => '\u{0000}',
            // '\u{E000}' => '\u{D7FF}',
            // c => char::from_u32(c as u32 - 1).unwrap(),
        // }
    // }
//
    // fn leading_bytes_to(self, ith_continuation_byte: usize) -> u32 {
        // (self as u32) & !((1 << (6 * ith_continuation_byte)) - 1)
    // }
// }

#[cfg(test)]
mod tests {
    use super::utf8_ranges;

    #[test]
    fn scratch() {
        println!("{:#?}", utf8_ranges('\u{0}', '\u{FFFF}'));
        println!("{:#?}", utf8_ranges('\u{80}', '\u{10FFFF}'));
        println!("{:#?}", utf8_ranges('\u{0}', '\u{10FFFF}'));
    }
}
