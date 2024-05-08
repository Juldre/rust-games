use std::arch::x86_64::_popcnt64;
use std::cmp::{max, min};

fn main() {
    let mut board: [u8; 9 * 9] = [0; 9 * 9];
    let mut sol_board: [u16; 9 * 9] = [0; 9 * 9];
    board[0] = 9u8;
    sol_board[0] = 0b11;

    println!("{:?}", find_lowest(&sol_board));
}

fn find_lowest(sol_board: &[u16; 9 * 9]) -> Option<usize> {
    return match sol_board.map(|val| val.count_ones()).iter().enumerate().map(|(idx, val)| (val, idx)).min() {
        None => { None }
        Some(tuple) => { Some(tuple.1) }
    };
}

// fn get_valid_numbers(board: [u16;9*9]) -> u16 {
//
// }

fn insert_value(sol_board: &[u16; 9 * 9], board: &[u8; 9 * 9], x: u8, y: u8, value: u8) -> bool {
    if min(x, min(y, value + 1)) < 0 { return false; }
    if max(x, max(y, value - 1)) > 8 { return false; }

    return false;
}