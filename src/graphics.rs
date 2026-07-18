// src/graphics.rs

use std::io::{self, Write};

/// Clears the entire terminal screen and moves the cursor to the top-left (1,1).
pub fn clear_screen() {
    // \x1B[2J clears the screen, \x1B[1;1H snaps the cursor to the top-left corner
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_coord(s: &str, r: u8, c: u8) {
    print!("\x1b[{};{}H{}", r, c, s);
}

pub fn flush() {
    let _ = io::stdout().flush();
}