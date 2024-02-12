#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::board::Board;

fn display_a_single_dot() {}
fn board_is_falling() -> bool {true}
fn yell() {}
fn show_exclaimation() {}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _board = Board::take().unwrap();

    loop {
        while !board_is_falling() {
            display_a_single_dot();
        }
        while board_is_falling() {
            yell();
            show_exclaimation();
        }
    }
}
