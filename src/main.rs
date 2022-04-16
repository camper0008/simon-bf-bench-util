mod error_display;
use crate::error_display::{buffer_fill_error, file_open_error, no_file_name_given_error};
use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| no_file_name_given_error());

    let mut deltas: Vec<u32> = (0..=8).map(|_| 0).collect();
    let mut prev_value = 0;
    let mut acc_value = 0;
    let mut idx = 255;
    let mut digits = 0;

    let mut f = File::open(&file_name).unwrap_or_else(|err| file_open_error(err, file_name));
    let mut buf = [0; 512];

    const NEWLINE_BYTE: u8 = '\n' as u8;
    const TAB_BYTE: u8 = '\t' as u8;
    const SPACE_BYTE: u8 = ' ' as u8;
    const CARRIAGE_RETURN_BYTE: u8 = 13 as u8;

    loop {
        let bytes_read = f
            .read(&mut buf)
            .unwrap_or_else(|err| buffer_fill_error(err));
        if bytes_read == 0 {
            break;
        }
        (0..bytes_read).for_each(|i| {
            let c = buf[i];

            // special characters
            if idx == 255 {
                idx = c - 48;
                return;
            } else if c == TAB_BYTE || c == SPACE_BYTE || c == CARRIAGE_RETURN_BYTE {
                return;
            } else if c == NEWLINE_BYTE {
                deltas[idx as usize] += acc_value - prev_value;
                digits = 0;
                prev_value = acc_value;
                acc_value = 0;
                idx = 255;
                return;
            }

            // padded 0s
            if digits == 0 && c - 48 == 0 {
                return;
            }

            acc_value = (acc_value * 10) + ((c as u32) - 48);
            digits += 1;
        })
    }

    deltas
        .iter()
        .enumerate()
        .for_each(|(idx, v)| println!("{}: {}", idx, v));
}
