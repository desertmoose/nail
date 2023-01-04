use phf::phf_map;

pub const DASH_UTF8: u8 = 45;
pub const DOT_UTF8: u8 = 46;
pub const SPACE_UTF8: u8 = 32;
pub const PIPE_UTF8: u8 = 124;

pub const AMINO_ALPHABET: [&str; 20] = [
    "A", "C", "D", "E", "F", "G", "H", "I", "K", "L", "M", "N", "P", "Q", "R", "S", "T", "V", "W",
    "Y",
];

pub const AMINO_ALPHABET_WITH_DEGENERATE: [&str; 29] = [
    "A", "C", "D", "E", "F", "G", "H", "I", "K", "L", "M", "N", "P", "Q", "R", "S", "T", "V", "W",
    "Y", "-", "B", "J", "Z", "O", "U", "X", "*", "~",
];

pub const AMINO_MAP: phf::Map<char, u8> = phf_map! {
    'A' => 0,
    'a' => 0,
    'C' => 1,
    'c' => 1,
    'D' => 2,
    'd' => 2,
    'E' => 3,
    'e' => 3,
    'F' => 4,
    'f' => 4,
    'G' => 5,
    'g' => 5,
    'H' => 6,
    'h' => 6,
    'I' => 7,
    'i' => 7,
    'K' => 8,
    'k' => 8,
    'L' => 9,
    'l' => 9,
    'M' => 10,
    'm' => 10,
    'N' => 11,
    'n' => 11,
    'P' => 12,
    'p' => 12,
    'Q' => 13,
    'q' => 13,
    'R' => 14,
    'r' => 14,
    'S' => 15,
    's' => 15,
    'T' => 16,
    't' => 16,
    'V' => 17,
    'v' => 17,
    'W' => 18,
    'w' => 18,
    'Y' => 19,
    'y' => 19,
    // end base alphabet
    'O' => 20,
    'o' => 20,
    'U' => 21,
    'u' => 21,
    'X' => 22,
    'x' => 22,
    'B' => 23,
    'b' => 23,
    'Z' => 24,
    'z' => 24,
    'J' => 25,
    'j' => 25,
};

pub const AMINO_INVERSE_MAP: phf::Map<u8, char> = phf_map! {
    0u8  => 'A',
    1u8  => 'C',
    2u8  => 'D',
    3u8  => 'E',
    4u8  => 'F',
    5u8  => 'G',
    6u8  => 'H',
    7u8  => 'I',
    8u8  => 'K',
    9u8  => 'L',
    10u8 => 'M',
    11u8 => 'N',
    12u8 => 'P',
    13u8 => 'Q',
    14u8 => 'R',
    15u8 => 'S',
    16u8 => 'T',
    17u8 => 'V',
    18u8 => 'W',
    19u8 => 'Y',
    // end base alphabet
    20u8 => 'O',
    21u8 => 'U',
    22u8 => 'X',
    23u8 => 'B',
    24u8 => 'Z',
    25u8 => 'J',
    45u8 => '-',
    46u8 => '.',
    255u8 => ' ',
};

pub const AMINO_BACKGROUND_FREQUENCIES: [f32; 20] = [
    0.0787945, // A
    0.0151600, // C
    0.0535222, // D
    0.0668298, // E
    0.0397062, // F
    0.0695071, // G
    0.0229198, // H
    0.0590092, // I
    0.0594422, // K
    0.0963728, // L
    0.0237718, // M
    0.0414386, // N
    0.0482904, // P
    0.0395639, // Q
    0.0540978, // R
    0.0683364, // S
    0.0540687, // T
    0.0673417, // V
    0.0114135, // W
    0.0304133, // Y
];

pub fn string_from_amino_bytes(bytes: &Vec<u8>) -> String {
    bytes.iter().map(|b| AMINO_INVERSE_MAP[b]).collect()
}
