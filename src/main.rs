#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        pac::{self, interrupt, TIMER1},
        prelude::*,
        timer::Timer,
    },
};

use critical_section_lock_mut::LockMut;

fn display_a_single_dot(image: &mut [[u8; 5]; 5]) {
    image[2][2] = 9;
    let led_display = GreyscaleImage::new(&image);
    DISPLAY.with_lock(|display| display.show(&led_display))
}
fn board_is_falling() -> bool {true}
fn yell() {}
fn show_exclaimation() {}

static DISPLAY: LockMut<Display<TIMER1>> = LockMut::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let display = Display::new(board.TIMER1, board.display_pins);
    DISPLAY.init(display);
    let mut timer = Timer::new(board.TIMER0);
    unsafe {
        board.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }
    let mut image = [[0; 5]; 5];

    loop {
        display_a_single_dot(&mut image);
        //while board_is_falling() {
        //    yell();
        //    show_exclaimation();
        //}
    }
}

#[interrupt]
fn TIMER1() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}
