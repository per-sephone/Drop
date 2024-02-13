#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        delay::Delay, gpio::{p0::P0_00, Level, Output, PushPull}, pac::{self, interrupt, TIMER1}, prelude::*, timer::Timer
    },
};
use critical_section_lock_mut::LockMut;

fn display_a_single_dot(image: &mut [[u8; 5]; 5]) {
    //set each element in the led array here
    for (row, row_array) in image.iter_mut().enumerate().take(5) {
        for (col, col_value) in row_array.iter_mut().enumerate().take(5) {
            *col_value = match (row, col) {
                (2, 2) => 9,
                _ => 0,
            };
        }
    }
    //turn into a GreyscaleImage for displaying
    let led_display = GreyscaleImage::new(image);
    DISPLAY.with_lock(|display| display.show(&led_display));
}
fn board_is_falling() -> bool {true}

fn yell(speaker: &mut P0_00<Output<PushPull>>, delay: &mut Delay) {
    speaker.set_high().unwrap();
    delay.delay_us(500u16);
    speaker.set_low().unwrap();
    delay.delay_us(500u16);
}

fn show_exclaimation(image: &mut [[u8; 5]; 5]) {
    for (row, row_array) in image.iter_mut().enumerate().take(5) {
        for (col, col_value) in row_array.iter_mut().enumerate().take(5) {
            *col_value = match (row, col) {
                (0, 2) | (1, 2) | (2, 2) | (4, 2) => 9,
                _ => 0,
            };
        }
    }
    let led_display = GreyscaleImage::new(image);
    DISPLAY.with_lock(|display| display.show(&led_display));
}

static DISPLAY: LockMut<Display<TIMER1>> = LockMut::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let display = Display::new(board.TIMER1, board.display_pins);
    DISPLAY.init(display);

    let mut delay = Delay::new(board.SYST);
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);

    //let mut timer = Timer::new(board.TIMER0);
    unsafe {
        board.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }
    let mut image = [[0; 5]; 5];

    loop {
        display_a_single_dot(&mut image);
        while board_is_falling() {
            yell(&mut speaker, &mut delay);
            show_exclaimation(&mut image);
        }
    }
}

#[interrupt]
fn TIMER1() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}
