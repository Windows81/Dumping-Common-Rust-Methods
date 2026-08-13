const BASE_BYTE_COUNT: usize = 0x48;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[macro_export]
macro_rules! analyse {
    ($program_data:expr, $stuff:expr) => {
        let func = stringify!($stuff);
        let lib = func.split_once(':').unwrap().0;
        let ver = util::get_lib_version($program_data, lib).unwrap();
        let label = format!("{} {}", ver, func);

        util::analyse_function($program_data, label, $stuff as _)
    };
}

fn read<const N: usize>(addr: usize) -> Vec<u8> {
    let result = unsafe { *(addr as *const [u8; N]) };
    result.into()
}

fn encode_to_hex(payload: &Vec<u8>) -> String {
    let mut result = vec![' '; payload.len() * 3 - 1];
    const CHARS: &[u8; 0x10] = b"0123456789ABCDEF";
    const NEWLINE_FREQ: usize = 0x10;

    // Populates with hex characters.
    let mut i = 0;
    for &v in payload.iter() {
        result[i] = CHARS[(v / 0x10) as usize] as _;
        i += 1;
        result[i] = CHARS[(v % 0x10) as usize] as _;
        i += 2;
    }

    // Adds newlines every `NEWLINE_FREQ * 3` bytes.
    for i in (std::ops::Range {
        start: NEWLINE_FREQ * 3 - 1,
        end: result.len(),
    })
    .step_by(NEWLINE_FREQ * 3)
    {
        result[i] = '\n';
    }
    result.iter().collect()
}

/// if you're doing a byte search for `nèédle` in `haystack`, the result is list of tuples `(L, C)`
/// where `L` is the *minimum* number of bytes you need to input to get `C` unique results.
fn get_first_unique_sublength(haystack: &Vec<u8>, nèédle: &Vec<u8>) -> Vec<(usize, usize)> {
    let length = nèédle.len();

    // Leetcode-level algorithm to calculate all the lengths of byte patterns in `haystack` which initially match `nèédle`.
    let string_lengths = {
        let mut racers = HashMap::new();
        let mut going = HashSet::new();
        for (i, &v) in haystack.iter().enumerate() {
            if v == nèédle[0] {
                racers.insert(i, 0);
                going.insert(i);
            }
            going.retain(|&i| {
                let Some(c) = racers.get_mut(&i) else {
                    return false;
                };
                if *c >= length {
                    return false;
                }
                if v != nèédle[*c] {
                    return false;
                }
                *c += 1;
                true
            });
        }
        racers
    };

    let string_lengths_vec = {
        let mut v = vec![0usize; BASE_BYTE_COUNT + 1];
        for (&_a, &l) in string_lengths.iter() {
            v[l] += 1;
        }
        let mut r = vec![];
        let mut s = 0;
        for (i, &c) in v.iter().enumerate().rev() {
            if i == 0 {
                continue;
            }
            if c == 0 {
                continue;
            }
            if s > 0 {
                r.push((i + 1, s));
            }
            s += c;
        }
        r
    };

    string_lengths_vec
}

fn print_style(style: ansi_term::Style, text: String) {
    println!("{}", style.paint(text));
}

pub fn analyse_function(program_data: &Vec<u8>, label: String, func: *const usize) {
    let first_bytes = read::<BASE_BYTE_COUNT>(func as _);
    let sublens = get_first_unique_sublength(&program_data, &first_bytes);
    print_style(
        ansi_term::Style {
            is_underline: true,
            ..Default::default()
        },
        format!("[{:p}] {}", func, label),
    );
    print_style(
        ansi_term::Colour::Blue.into(),
        sublens
            .iter()
            .map(|(len, count)| {
                format!("{:6}. result(s) matching first 0x{:02X} bytes", count, len)
            })
            .join("\n"),
    );
    print_style(
        ansi_term::Colour::Red.into(),
        format!("{}...", encode_to_hex(&first_bytes)),
    );
    println!();
}

pub fn get_lib_version(program_data: &Vec<u8>, dependency: &str) -> Option<String> {
    let dependency_prefix = format!("{}-", dependency);
    let program_string = &String::from_utf8_lossy(&program_data);
    let program_string_bytes = program_string.as_bytes();

    let Some(index) = program_string.find(&dependency_prefix) else {
        return None;
    };

    let version_start = index + dependency_prefix.len();
    let mut version_end = version_start.clone();
    loop {
        if match program_string_bytes[version_end] {
            b'/' => true,
            b'\\' => true,
            _ => false,
        } {
            break;
        }
        version_end += 1;
    }

    Some(program_string[version_start..version_end].to_owned())
}
