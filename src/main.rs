#![no_main]
#![no_std]

use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        delay::Delay,
        gpio::{p0::P0_00, Level, Output, PushPull},
        pac::{self, interrupt, TIMER1},
        prelude::*,
    },
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

/// Displays a single dot in the center of the LED screen.
/// This is default for when the microbit is not falling.
/// Iterates through the 5x5 array representing the image 
/// displayed on the LED screen and sets the center bit to 9
/// (the brightest) and all others to 0.
/// Creates a GreyscaleImage with the given 2D array,
/// then displays the image on the microbit.
/// # Arguments
///
/// * `image` - A mutable reference to a 5x5 array of u8 values representing
///             the image to be displayed. The dot will be placed in the center
///             of this image array.
/// # Panics
///
/// This function panics if it fails to acquire a lock on the display.
fn display_a_single_dot(image: &mut [[u8; 5]; 5]) {
    for (row, row_array) in image.iter_mut().enumerate().take(5) {
        for (col, col_value) in row_array.iter_mut().enumerate().take(5) {
            *col_value = match (row, col) {
                (2, 2) => 9,
                _ => 0,
            };
        }
    }
    let led_display = GreyscaleImage::new(image);
    DISPLAY.with_lock(|display| display.show(&led_display));
}
fn board_is_falling() -> bool {
    true
}

/// Makes the speaker emit a 1Khz square wave.
/// This happens in tandem with the display showing
/// an exclaimation.
/// It toggles the speaker pin between high and low voltage levels 
/// with a delay in between to produce the pitch.
/// # Arguments
///
/// * `speaker` - A mutable reference to the speaker pin (e.g., GPIO pin) that will emit
///               the sound. It should implement the `Output<PushPull>` trait.
///
/// * `delay` - A mutable reference to a delay provider (e.g., `Delay` struct from the `embedded_hal` crate)
///             used to control the duration of each phase of the sound effect.
/// # Panics
///
/// This function may panic if it fails to set the speaker pin to a high or low state.
fn yell(speaker: &mut P0_00<Output<PushPull>>, delay: &mut Delay) {
    speaker.set_high().unwrap();
    delay.delay_us(500u16);
    speaker.set_low().unwrap();
    delay.delay_us(500u16);
}

/// Displays an exclaimation in the center of the LED screen.
/// This is the image shown when the microbit is falling.
/// Iterates through the 5x5 array representing the image 
/// displayed on the LED screen and sets the exclaimation point
/// image LEDs to 9 (the brightest) and all others to 0.
/// Creates a GreyscaleImage with the given 2D array,
/// then displays the image on the microbit.
/// # Arguments
///
/// * `image` - A mutable reference to a 5x5 array of u8 values representing
///             the image to be displayed. The dot will be placed in the center
///             of this image array.
/// # Panics
///
/// This function panics if it fails to acquire a lock on the display.
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

/// The entry point grabs all the needed peripherals off the board.
/// It sets up the interrupts and timers needed for nonblocking display.
/// While the board is not falling, it displays a single dot.
/// While the board is falling, it displays an exclaimation point and
/// yells.
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let display = Display::new(board.TIMER1, board.display_pins);
    DISPLAY.init(display);

    let mut delay = Delay::new(board.SYST);
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);

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
