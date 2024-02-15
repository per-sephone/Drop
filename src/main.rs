#![no_main]
#![no_std]

use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use lsm303agr::{interface::I2cInterface, mode::MagOneShot, Lsm303agr};
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        delay::Delay,
        gpio::{p0::P0_00, Level, Output, PushPull},
        pac::{self, interrupt, TIMER1, TWIM0},
        prelude::*,
        twim, Timer, Twim,
    },
    pac::{twim0::frequency::FREQUENCY_A, TIMER2},
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


/// Determines if the microbit is falling. 
/// Checks the acceleration status and retrieves the data.
/// Inputs the x,y,z data into the Cartesian magnitude of acceleration formula.
/// 
/// # Arguments
/// 
/// * accel - A mutable reference to a lsm303agr instance.
/// 
/// # Returns
/// 
/// * true if the board is falling, otherwise false.
fn board_is_falling(accel: &mut Lsm303agr<I2cInterface<Twim<TWIM0>>, MagOneShot>) -> bool {
    if accel.accel_status().unwrap().xyz_new_data() {
        let data = accel.acceleration().unwrap();
        let x = (data.x_mg() / 1000) as f32;
        let y = (data.y_mg() / 1000) as f32;
        let z = (data.z_mg() / 1000) as f32;
        rprintln!("{}", ((x * x) + (y * y) + (z * z)));
        return 0.25 < ((x * x) + (y * y) + (z * z));
    }
    false
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
    let duration:u16 = 200;
    for _ in 0..duration {
        speaker.set_high().unwrap();
        delay.delay_us(500u16);
        speaker.set_low().unwrap();
        delay.delay_us(500u16);
    }
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
fn show_exclaimation(image: &mut [[u8; 5]; 5], timer: &mut Timer<TIMER2>) {
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
    timer.delay_ms(1000u32);
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
    //set up display
    let display = Display::new(board.TIMER1, board.display_pins);
    DISPLAY.init(display);
    let mut timer2 = Timer::new(board.TIMER2);
    unsafe {
        board.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }
    //set up speaker
    let mut delay = Delay::new(board.SYST);
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);
    //set up accelerometer
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut timer = Timer::new(board.TIMER0);
    let mut lsm303 = Lsm303agr::new_with_i2c(i2c);
    lsm303.init().unwrap();
    lsm303
        .set_accel_mode_and_odr(
            &mut timer,
            lsm303agr::AccelMode::Normal,
            lsm303agr::AccelOutputDataRate::Hz50,
        )
        .unwrap();


    let mut image = [[0; 5]; 5];

    loop {
        display_a_single_dot(&mut image);
        if board_is_falling(&mut lsm303) {
            show_exclaimation(&mut image, &mut timer2);
            yell(&mut speaker, &mut delay);
            timer.delay(1000u32);
        }
    }
}

#[interrupt]
fn TIMER1() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}
